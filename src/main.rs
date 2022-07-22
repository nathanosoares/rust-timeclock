use application::HttpApp;
use application::App;

pub mod application;

#[tokio::main]
async fn main() {
    let http = HttpApp::new();

    http.start().await;
}
