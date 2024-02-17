#[cfg(feature = "ssr")]
mod ssr {
    use std::path::Path;

    use audiobookroom::server_api::ssr::init_logger_info;
    use clap::Parser;
    use sea_orm::Database;
    pub async fn main() {
        dotenv::dotenv().ok();
        init_logger_info();
        let Cli {
            db,
            book_dir,
            new_book_name,
            author_name,
            source_dir,
        } = Cli::parse();

        let db = Database::connect(&db).await.unwrap();
        audiobookroom::tools::create_new_book(
            author_name,
            new_book_name,
            Path::new(&book_dir),
            Path::new(&source_dir),
            &db,
        )
        .await
        .unwrap();
    }

    #[derive(Debug, Parser)]
    pub struct Cli {
        /// the database url,start at "mysql://"
        #[clap(
            short,
            long,
        )]
        db: String,

        /// the path store all books
        #[clap(short, long)]
        book_dir: String,

        /// the name of the book to be created
        #[clap(short, long)]
        new_book_name: String,
        /// the name of the author of the book to be created
        #[clap(short, long)]
        author_name: String,
        /// the source dir of the book to be find
        #[clap(short, long)]
        source_dir: String,
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
