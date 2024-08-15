pub async fn tracing_sub() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();
}
// TODO extend logging levels
