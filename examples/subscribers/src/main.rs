use novu::Novu;
use std::env;

#[async_std::main]
async fn main() {
    let novu = Novu::new(env::var("API_TOKEN").unwrap(), None).unwrap();

    let subscribers = novu.subscribers;

    let list = subscribers.list(0).await.unwrap();

    println!("{:?}", list);
}
