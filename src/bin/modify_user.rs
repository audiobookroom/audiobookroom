#[cfg(feature = "ssr")]
mod ssr {

    use audiobookroom::server_api::ssr::init_logger_info;
    use clap::Parser;
    use sea_orm::Database;
    pub async fn main() {
        dotenv::dotenv().ok();
        init_logger_info();
        let Cli {
            db,
            name,
            password,
            role,
        } = Cli::parse();

        let db = Database::connect(&db).await.unwrap();
        audiobookroom::tools::alter_user(name, password, role, &db).await;
    }

    #[derive(Debug, Parser)]
    pub struct Cli {
        /// the database url,start at "mysql://"
        #[clap(short, long)]
        db: String,
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        password: Option<String>,
        #[clap(short, long)]
        role: Option<i32>,
    }
}

#[cfg(feature = "ssr")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    ssr::main().await;
}

#[cfg(not(feature = "ssr"))]
fn main() {
    println!("this should run in server, enable \"ssr\" feature to run this code.");
}
