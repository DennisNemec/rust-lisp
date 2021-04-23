use lisp::constants::AuthenticationType;

fn main() {
    let t: AuthenticationType = AuthenticationType::NoAuthentication;
    println!("{:?}", t);
}
