use std::sync::Arc;
use migration_service_adapter::{Migrator, MigratorTrait};
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;

#[derive(Clone, Debug)]
pub struct AppDatabase {
    db: Arc<DatabaseConnection>,
}

impl AppDatabase {
    pub async fn new() -> Self {
        dotenv().ok();
        let db_url = dotenv::var("DATABASE_URL").expect("Expected DB URL");
        let db = Database::connect(db_url)
            .await
            .expect("Failed to create database connection");
        println!("DB connection created");
        AppDatabase { db: Arc::new(db) }
    }

    pub fn get_db(&self) -> &DatabaseConnection {
       Arc::as_ref(&self.db)
    }
}

static DB: OnceCell<AppDatabase> = OnceCell::new();

pub async fn init_database() {
    let db = AppDatabase::new().await;
    DB.set(db).expect("Failed to initialize database");
}

pub fn get_db_connection() -> &'static DatabaseConnection {
    &DB.get().expect("Database not initialized").get_db()
}

pub async fn auto_migrator() {
    println!("Automigrator starting...");
    let db = get_db_connection();
    Migrator::up(db, None).await.unwrap();
}