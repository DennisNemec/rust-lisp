#[derive(Clone, Debug, PartialEq)]
pub enum AuthenticationType {
    NoAuthentication = 0x00,
    GSSAPI,
    UsernamePassword,
    ChallengeHandshakeAuthentication,
    Unassigned,
    ChallengeResponseAuthentication,
    SecureSocketsLayer,
    NdsAuthentication,
    MultiAuthenticationFramework,
    JsonParameterBlock,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Authentication {
    Custom(u8),
    Assigned(AuthenticationType)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionType {
    Connect = 0x01,
    Bind,
    UdpAssociate,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AddressType {
    IPv4 = 0x01,
    IPv6 = 0x04,
    DNS = 0x03,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Reply {
    Succeeded = 0x00,
    GeneralSocksServerFailure,
    ConnectionNotAllowedByRuleset,
    NetworkUnreachable,
    HostUnreachable,
    ConnectionRefused,
    TTLExpired,
    CommandNotSupported,
    AddressTypeNotSupported,
}

pub enum ConnectionState {
    Initiated,
    AuthRequestWaiting,
    ConnectionRequestWaiting,
    Established,
}
