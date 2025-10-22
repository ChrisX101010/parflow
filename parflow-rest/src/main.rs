use axum::{routing::get, Router, Json};
use parflow_core::{run_example_par, run_example_seq};
use std::net::SocketAddr;

pub async fn run_rest_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/par", get(handle_par))
        .route("/seq", get(handle_seq));

    let addr = SocketAddr::from(([0,0,0,0], port));
    println!("🌐 REST server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn handle_par() -> Json<Vec<i32>> {
    let vec = run_example_par().await;
    Json(vec)
}

async fn handle_seq() -> Json<Vec<i32>> {
    let vec = run_example_seq().await;
    Json(vec)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting ParFlow REST Server");
    
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    run_rest_server(port).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handlers_direct() {
        let p = handle_par().await;
        assert_eq!(p.0, vec![1, 2]);
        let s = handle_seq().await;
        assert_eq!(s.0, vec![1, 2]);
    }
}
