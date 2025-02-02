use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::system::system::System;

use super::filters;

pub struct Server {
    system: Arc<Mutex<System>>,
}

impl Server {
    pub fn new(system: Arc<Mutex<System>>) -> Server {
        Server { system }
    }
    pub async fn run(&self) {
        println!("Starting server...");
        let create_plant = filters::create_plant(Arc::clone(&self.system));
        let water_plant = filters::water_plant(Arc::clone(&self.system));

        let routes = create_plant.or(water_plant).with(warp::log("plant"));

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    }
}
