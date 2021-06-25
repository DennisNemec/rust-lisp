use crate::protocol::constants::{ConnectionType, Authentication};
use crate::protocol::header::{
    AuthenticationRequest, AuthenticationResponse, ConnectionRequest, ConnectionResponse,
};
use crate::protocol::Serialization;
use super::ConnectionState;

use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::borrow::Borrow;
use crate::server::handler::authentication::AuthHandler;
use crate::server::AuthenticationState;

#[derive(Clone)]
pub struct ConnectionInstance {
    src_addr: SocketAddr,
    dst_ip: Option<String>,
    state: ConnectionState,
    connection_type: Option<ConnectionType>,
    socket: Arc<TcpStream>,
    auth_method: Option<Authentication>,
}

impl ConnectionInstance {
    pub fn new(
        src_addr: SocketAddr,
        socket: Arc<TcpStream>,
    ) -> Self {
        Self {
            src_addr,
            dst_ip: None,
            state: ConnectionState::Initiated,
            connection_type: None,
            socket,
            auth_method: None,
        }
    }

    pub async fn task<A>(mut self) -> std::io::Result<()> where A: AuthHandler {
        loop {
            let mut instance = self.clone();

            instance.socket.readable().await?;
            let mut buf: Vec<u8> = Vec::new();

            match instance.socket.try_read_buf(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    instance.handle_incoming_data::<A>(buf);
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    fn handle_incoming_data<A>(mut self, data: Vec<u8>) where A: AuthHandler {
        match self.state {
            ConnectionState::Initiated => {
                self.state = ConnectionState::Authenticating(AuthenticationState::Initiated);
                self.handle_auth_request::<A>(AuthenticationRequest::deserialize(data).unwrap());
            },

            ConnectionState::Authenticating(AuthenticationState::Initiated) => {
                self.state = ConnectionState
            }

            _ => ()
        }
    }

    fn handle_auth_request<A>(mut self, auth_request: AuthenticationRequest) where A: AuthHandler {
        match self.state {
            ConnectionState::Authenticating(AuthenticationState::Initiated) => {
                self.auth_method = Some(A::choose_auth_method(auth_request.auth));
                self.send_auth_method();
            },

            ConnectionState::Authenticating(AuthenticationState::WaitForNegotiation) => {
                let authenticated = A::authenticate(self.auth_method.unwrap().clone(), self.socket.clone());

                if authenticated == false {
                    self.state = ConnectionState::Closed;
                } else {
                    self.state = ConnectionState::Authenticating(AuthenticationState::Authenticated);
                }
            },

            _ => ()
        }
    }

    fn handle_auth_negotiation<A>(self, data: Vec<u8>) {

    }

    fn handle_connection_request(self, con_request: ConnectionRequest) {
        todo!()
    }

    fn initiate_connection(self) {
        todo!()
    }

    // Communication methods
    async fn send_auth_method(self) {
        if let Some(n) = self.auth_method {
            self.socket.writable().await.expect("Error while waiting for writable.");
            self.socket.try_write(AuthenticationResponse::new(n).serialize().as_slice());
        }
    }
}
