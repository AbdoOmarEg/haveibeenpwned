use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use std::path::Path;

// use haveibeenpwned-v2::db;

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone, FromRow, Debug)]
struct User {
    id: i64,
    email: String,
}

pub async fn initialize_database() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    //
    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db)
        .await
        .expect("can't connect to database");

    // migration code
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

pub async fn insert_email(email: &str) -> Result<(), sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query("INSERT INTO fishy_website_com (email) VALUES (?)")
        .bind(email)
        .execute(&db)
        .await;

    match result {
        Ok(_) => {
            println!("Email inserted into the database: {}", email);
            Ok(())
        }
        Err(err) => {
            println!(
                "Email already exists in the database: {}, with error: {}",
                email, err
            );
            Err(err)
        }
    }
}
