mod server;

use server::server::Server;
fn main() {
    let mut server = Server::<2>::new();
    server.run();
}
