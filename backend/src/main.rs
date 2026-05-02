use axum::serve;
use backend::presentation::router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = router::new();
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server is running on http://localhost:3000");
    serve(listener, app).await
}
