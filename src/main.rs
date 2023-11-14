use simple_hyper_client::{Client, HttpConnector};

fn main() {
    let connector = HttpConnector::new();
    let client = Client::with_connector(connector);
    println!("ok");
}
