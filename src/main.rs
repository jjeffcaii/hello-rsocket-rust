use tokio::runtime::Runtime;
use std::thread::sleep;
use std::time::Duration;
use hello_rsocket::{responder, requester};


fn main() {
    let server_runtime = Runtime::new().unwrap();
    let mut client_runtime = Runtime::new().unwrap();

    server_runtime.spawn(async move {
        responder::start().await
    });

    let sleep_millis = Duration::from_millis(500);
    sleep(sleep_millis);

    client_runtime.block_on(async {
        let request_coon = requester::RequestCoon::new().await;

        request_coon.fnf().await;
        request_coon.meta_push().await;
        request_coon.request_response().await;
        request_coon.request_stream().await;
        request_coon.request_channel().await;
    });
}
