use std::env;

use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone, FromRow, Debug)]
struct User {
    id: i64,
    email: String,
}

pub async fn initialize_database() -> SqlitePool {
    // accepting args
    let args: Vec<String> = env::args().collect();

    let email = if args.len() > 1 { Some(&args[1]) } else { None };

    // db start
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await.unwrap();

    //migration script
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    //start
    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    // args inserting
    if let Some(email) = email {
        let db = SqlitePool::connect(DB_URL).await.unwrap();

        let result = sqlx::query("INSERT INTO fishy_website_com (email) VALUES (?)")
            .bind(email)
            .execute(&db)
            .await;

        match result {
            Ok(_) => {
                println!("Email inserted into the database: {}", email);
            }
            Err(err) => {
                println!(
                    "Email already exists in the database: {}, with error: {}",
                    email, err
                );
            }
        }
    }

    let email_results = sqlx::query_as::<_, User>("SELECT id, email FROM fishy_website_com")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("Emails:");
    for email in email_results {
        println!("[{}] email: {}", email.id, &email.email);
    }

    // // insert
    // let result = sqlx::query("INSERT INTO fishy_website_com (email) VALUES (?)")
    //     .bind("bobby")
    //     .execute(&pool)
    //     .await
    //     .unwrap();
    // println!("Query result: {:?}", result);
    //
    // // delete
    // let delete_result = sqlx::query("DELETE FROM fishy_website_com WHERE email=$1")
    //     .bind("bobby")
    //     .execute(&pool)
    //     .await
    //     .unwrap();
    // println!("Delete result: {:?}", delete_result);

    //end
    pool
}
