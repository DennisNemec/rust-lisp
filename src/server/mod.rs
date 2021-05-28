pub mod handler;
pub mod session;
pub mod socks;

pub use socks::Socks5Server;

pub struct Paket {
    pub data: Vec<u8>,
    pub size: usize,
    pub to_be_forwarded: bool,
}

#[derive(Clone)]
pub enum AuthenticationState {
    Initiated,
    WaitForNegotiation,
    Authenticated,
}

#[derive(Clone)]
pub enum ConnectionState {
    Initiated,
    Authenticating(AuthenticationState),
    Connecting,
    Established,
    Closed,
}
