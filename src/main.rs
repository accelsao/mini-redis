mod net;

fn main() -> std::io::Result<()> {
    let port = 6379;
    let mut server = net::Server::new("localhost", port);
    server.start()
}
