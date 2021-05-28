use crate::protocol::constants::{Authentication, AuthenticationType};
use tokio::net::TcpStream;
use std::sync::Arc;

pub trait AuthHandler {
    /*
    Handles the authentication of the SOCKS5 negotiation.
    */
    fn authenticate(auth: Authentication, tcp_stream: Arc<TcpStream>) -> bool {
        true
    }

    fn choose_auth_method(requested_auth_methods: Vec<Authentication>) -> Authentication {
        Authentication::Assigned(AuthenticationType::NoAuthentication)
    }
}
