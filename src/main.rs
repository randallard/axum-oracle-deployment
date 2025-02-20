use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .route("/", get(handler));

    // Run it with hyper on localhost:8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Hello Oracle</title>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-100 min-h-screen flex items-center justify-center">
            <div class="bg-white p-8 rounded-lg shadow-lg">
                <h1 class="text-4xl font-bold text-blue-600 mb-4">Hello Oracle</h1>
                <p class="text-gray-600">Welcome to your Axum application with Tailwind CSS!</p>
            </div>
        </body>
        </html>
    "#)
}