#[derive(Debug)]
pub enum AuthenticationType {
    NoAuthentication = 0x00,
    GSSAPI = 0x01,
    UsernamePassword = 0x02,
    ChallengeHandshakeAuthentication = 0x03,
    Unassigned = 0x04,
    ChallengeResponseAuthentication = 0x05,
    SecureSocketsLayer = 0x06,
    NdsAuthentication = 0x07,
    MultiAuthenticationFramework = 0x08,
    JsonParameterBlock = 0x09,
}

pub enum ConnectionType {
    Connect = 0x01,
    Bind = 0x02,
    UdpAssociate = 0x03
}

pub enum AddressType {
    IPv4 = 0x01,
    Ipv6 = 0x04,
    DNS = 0x03
}

pub enum Reply {
    Succeeded = 0x00,
    GeneralSocksServerFailure = 0x01,
    ConnectionNotAllowedByRuleset = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefused = 0x05,
    TTLExpired = 0x06,
    CommandNotSupported = 0x07,
    AddressTypeNotSupported = 0x08,
}

