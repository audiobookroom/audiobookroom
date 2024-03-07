#[cfg(feature = "ssr")]
mod ssr {

    use audiobookroom::{
        fallback::file_and_error_handler,
        server_api::{ssr::*, User},
        state::AppState,
        ui::app::App,
    };
    use axum::{
        body::Body as AxumBody,
        extract::{Request, State},
        middleware::Next,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use leptos::{get_configuration, logging::log, provide_context};
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    #[cfg(feature = "mysql")]
    use sqlx::mysql::MySqlPoolOptions as PoolOptions;
    #[cfg(feature = "sqlite")]
    use sqlx::sqlite::SqlitePoolOptions as PoolOptions;

    use tower::ServiceBuilder;
    use tower_http::services::ServeDir;

    async fn server_fn_handler(
        State(app_state): State<AppState>,
        auth_session: AuthSession,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.db.clone());
            },
            request,
        )
        .await
    }

    async fn leptos_routes_handler(
        auth_session: AuthSession,
        State(app_state): State<AppState>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_route_with_context(
            app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.db.clone());
            },
            App,
        );
        handler(req).await.into_response()
    }

    pub async fn main() {
        use std::env;
        dotenv::dotenv().unwrap();

        init_logger_info();

        // 1. setup the database
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        tracing::info!("db_url: {}", db_url);
        let pool = PoolOptions::new()
            .connect(&db_url)
            .await
            .expect("Could not make pool.");
        #[cfg(feature = "sqlite")]
        let db = SqlxConnector::from_sqlx_sqlite_pool(pool.clone());
        #[cfg(feature = "mysql")]
        let db = SqlxConnector::from_sqlx_mysql_pool(pool.clone());

        // 2. Auth section
        let session_config = SessionConfig::default().with_table_name("axum_sessions");
        let auth_config = AuthConfig::<i32>::default();
        let session_store =
            SessionStore::<SessionPool>::new(Some(pool.clone().into()), session_config)
                .await
                .unwrap();

        // 3. Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(App);
        tracing::info!("leptos_options: {:?}", leptos_options);
        let app_state = AppState {
            leptos_options,
            routes: routes.clone(),
            db: db.clone(),
        };

        async fn check_login(
            request: axum::extract::Request,
            next: Next,
        ) -> axum::response::Response {
            let auth = request.extensions().get::<AuthSession>().unwrap();
            match auth.current_user {
                Some(_) => next.run(request).await,
                None => {
                    // return a 401
                    axum::response::Response::builder()
                        .status(401)
                        .body("Unauthorized".into())
                        .unwrap()
                }
            }
        }

        // 4. build our application with a route, will provide the context for server functions(for api call) and leptos routes (for ssr)
        //  the fetchbook service is a static service but need behad check login layer
        let fetch_book_service = ServiceBuilder::new()
            .layer(axum::middleware::from_fn(check_login))
            .service(ServeDir::new("fetchbook"));

        let app = Router::new()
            .route(
                "/api/*fn_name",
                get(server_fn_handler).post(server_fn_handler),
            )
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .nest_service("/fetchbook", fetch_book_service)
            .fallback(file_and_error_handler)
            .layer(
                AuthSessionLayer::<User, i32, SessionPool, SqlxPool>::new(Some(pool.clone()))
                    .with_config(auth_config),
            ) // authlayer is required for AuthSession
            .layer(SessionLayer::new(session_store)) // sessionlayer is required for AuthSessionLayer
            .with_state(app_state);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    ssr::main().await;
}

#[cfg(feature = "hydrate")]
fn main() {}
