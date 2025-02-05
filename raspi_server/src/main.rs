use std::sync::Arc;

use tokio::sync::Mutex;
use victorin::{server::server::Server, system::system::System};

#[tokio::main]
async fn main() {
    let config = victorin::config::config::Config::init("src/config/system.yaml").unwrap();

    let system: Arc<Mutex<System>> = Arc::new(Mutex::new(System::init(config)));

    let server = Server::new(Arc::clone(&system));

    let system_task = tokio::spawn(async move {
        System::run(Arc::clone(&system)).await;
    });

    let server_task = tokio::spawn(async move {
        server.run().await;     
    });

    // Wait for both tasks to finish
    let _ = tokio::try_join!(server_task, system_task);
}
