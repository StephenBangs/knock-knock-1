use axum::{Router, routing::get};
//not adding axum extra crate
use tokio::net;
//make new router, init with stuff

async fn hello() -> String {
    "<html><body><p>hello world</p></body></html>".to_string()
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new().route("/", get(hello));
    let listener = net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())

}


#[tokio::main]
async fn main() {
    if let Err(err) = serve().await {
        eprintln!("error: {}", err);
        std::process::exit(1)
    }

}













//working hello whee
// async fn hello(who: &str) -> String {
//     format!("hello {}", who)
//     //"Hello, world!"
// }


// #[tokio::main]
// async fn main() {
//     println!("{}", hello("whee").await);
// }
