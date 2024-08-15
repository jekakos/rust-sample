use crate::{db::{auto_migrator, get_db_connection, init_database}, redis::init_redis, router::route_init, vendor::ServiceVendor};

pub async fn run() {
    
    init_database().await;
    auto_migrator().await;

    init_redis().await;
    
    let db = get_db_connection();
    let vendor = ServiceVendor::new();
    let web_client = reqwest::Client::new(); 

    let app = route_init(db, &web_client, &vendor).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    eprintln!("Listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();

    println!("SERVICE RUN!");
}
