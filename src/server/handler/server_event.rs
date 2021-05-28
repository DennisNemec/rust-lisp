use crate::server::session::ConnectionInstance;
use std::sync::{Arc, Mutex};
use crate::server::handler::authentication::AuthHandler;

pub trait ServerEventHandler<A> where A: AuthHandler {
    fn accepted(instance: Arc<Mutex<ConnectionInstance>>);
    fn authenticated(instance: Arc<Mutex<ConnectionInstance>>);
    fn established(instance: Arc<Mutex<ConnectionInstance>>);
    fn closed(instance: Arc<Mutex<ConnectionInstance>>);
}
