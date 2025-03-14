use std::sync::Arc;

use crate::system::system::System;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio::sync::Mutex;
use warp::Filter;

use super::filters;
use std::fmt;
use tokio::sync::broadcast;

use warp::ws::{Message, WebSocket};

use warp::{reject, reply, Rejection, Reply};

#[derive(Debug)]
pub enum MyError {
    InvalidRequest,
    InternalError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyError::InvalidRequest => write!(f, "Invalid Request"),
            MyError::InternalError => write!(f, "Internal Server Error"),
        }
    }
}

impl warp::reject::Reject for MyError {}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(MyError::InvalidRequest) = err.find() {
        eprintln!("Invalid request received: {:?}", err);
        Ok(reply::with_status(
            "Invalid request",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if let Some(MyError::InternalError) = err.find() {
        eprintln!("Internal error: {:?}", err);
        Ok(reply::with_status(
            "Internal server error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        // Log unexpected errors
        eprintln!("Unexpected error: {:?}", err);
        Err(err)
    }
}

pub(crate) type Tx = broadcast::Sender<String>;
type Rx = broadcast::Receiver<String>;

fn websocket_filter(
    tx: Tx,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("humidity_updates")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone();
            ws.on_upgrade(move |socket| {
                let tx = tx.clone();
                handle_websocket(socket, tx)
            })
        })
}

async fn handle_websocket(ws: WebSocket, tx: Tx) {
    let mut rx = tx.subscribe();
    let (mut ws_tx, mut ws_rx) = ws.split();

    // println!(
    //     "New WebSocket client connected! Subscribers: {}",
    //     tx.receiver_count()
    // );

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            //println!("Sending WebSocket message to client: {}", msg);
            if let Err(e) = ws_tx.send(Message::text(msg)).await {
                println!("WebSocket send error: {:?}", e);
            }
        }
    });

    while let Some(Ok(_)) = ws_rx.next().await {} // Keep the connection open
}

pub struct Server {
    system: Arc<Mutex<System>>,
}

impl Server {
    pub fn new(system: Arc<Mutex<System>>) -> Server {
        Server { system }
    }

    pub async fn run(&self) {
        println!("Starting server...");

        let (tx, _) = broadcast::channel::<String>(16);

        let create_plant = filters::create_plant(Arc::clone(&self.system));
        let water_plant = filters::water_plant(Arc::clone(&self.system));
        let activate_device = filters::activate_device(Arc::clone(&self.system));
        let cycle_complete = filters::cycle_complete(Arc::clone(&self.system));
        let humidity_updates = websocket_filter(tx.clone());

        let humidity_plant = filters::humidity_plant(tx);

        let routes = create_plant
            .or(water_plant)
            .or(activate_device)
            .or(cycle_complete)
            .or(humidity_plant)
            .or(humidity_updates)
            .with(warp::log("plant"));

        warp::serve(routes).run(([0, 0, 0, 0], 3031)).await;
    }
}
