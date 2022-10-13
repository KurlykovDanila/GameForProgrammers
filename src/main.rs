use std::time::Instant;

use server::server::Server;

mod game;
mod server;

fn main() {
    let mut server = Server::<2, 2>::new();
    server.run();
}
