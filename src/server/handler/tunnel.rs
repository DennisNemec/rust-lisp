use crate::server::Paket;

pub trait TunnelHandler {
    fn next_paket(data: Vec<u8>) -> Paket;
}
