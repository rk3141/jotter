use std::env::args;

use rocket::tokio;
#[tokio::main]
async fn main() {
    let args = args();
    let args: Vec<String> = args.collect();

    notes::run(args, false).await;
}
