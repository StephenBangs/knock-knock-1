use axum::{Router, routing::get};
use axum::response;
//not adding axum extra crate
use tokio::net;
//make new router, init with stuff

struct Joke {
    whos_there: &'static str,
    answer: &'static str,
}

const THE_JOKE: Joke = Joke {
    whos_there: "Boo",
    answer: "Don't cry about it!",
//     whos_there.to_string(),
//     answer.to_string(),
};

fn render_joke(joke: &Joke) -> String {
    format!("<p>Knock Knock!</p><p>Who's there?</p><p>{}</p><p>{} Who?</p>", joke.whos_there, joke.answer)
}


async fn hello() -> response::Html<String> {
    //"<html><body><p>hello world</p></body></html>".to_string()
    let joke = render_joke(&THE_JOKE);
    response::Html(format!(r#"<head><title>"Knock Knock!"</title></head><body>{}</body></html>"#, joke))
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
