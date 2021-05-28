use crate::server::handler::authentication::AuthHandler;
use crate::server::handler::server_event::ServerEventHandler;
use crate::server::handler::tunnel::TunnelHandler;
use tokio::net::TcpListener;
use crate::server::session::ConnectionInstance;
use std::sync::Arc;

pub struct Socks5Server
{
    host_ip: String,
    host_port: u16,
}

impl Socks5Server {
    pub fn new(host_ip: String, host_port: u16) -> Self {
        Self {
            host_ip,
            host_port,
        }
    }

    pub async fn start<A> (mut self) -> std::io::Result<()> where A: AuthHandler{
        let listen_socket =
            TcpListener::bind(format!("{}:{}", self.host_ip, self.host_port))
                .await
                .expect("Could not bind listener socket.");

        loop {
            let (mut socket, address) = listen_socket.accept().await.expect("Could not accept connection.");

            // TODO: implement accept event handler
            tokio::task::spawn(async move {
                let mut connection_instance = ConnectionInstance::new(address, Arc::new(socket).clone());
                connection_instance.task::<A>().await.expect("error");
            });
        }
    }
}
