use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

mod server;

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays and affects state
async fn get_count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(1, Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// Client must send ping atleast once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Chat server
    addr: Addr<server::ChatServer>,
}

impl Actor for WsChatSession {}

fn main() {
    for i in 1..100 {
        match i {
            i if i % 15 == 0 => println!("fizzBuzz"),
            i if i % 5 == 0 => println!("buzz"),
            i if i % 3 == 0 => println!("fizz"),
            _ => println!("{}", i),
        }
    }
}
