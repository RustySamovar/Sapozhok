use crate::dispatch::DispatchServer;

mod dispatch;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let ds = DispatchServer::new();
    ds.run();
}
