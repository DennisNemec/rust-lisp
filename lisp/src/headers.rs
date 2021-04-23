use crate::constants::*;

/*
In order to establish a SOCKS5 connection the client needs to
do a handshake with the SOCKS5 server. The client tells the server
his supported types of authentication methods.
 */
pub struct AuthenticationRequest {
    /*
    Version of the used protocol. Fixed value of 0x05.
    */
    ver: u8,

    /*
    Amount of supported authentication protocols by the client.
     */
    nauth: u8,

    /*
    List of supported protocols by the client. 1 Byte per Method supported.
    The authentication methods supported are numbered as follows:

    0x00: No authentication
    0x01: GSSAPI[14]
    0x02: Username/password[15]
    0x03–0x7F: methods assigned by IANA[16]
        0x03: Challenge-Handshake Authentication Protocol
        0x04: Unassigned
        0x05: Challenge-Response Authentication Method
        0x06: Secure Sockets Layer
        0x07: NDS Authentication
        0x08: Multi-Authentication Framework
        0x09: JSON Parameter Block
        0x0A–0x7F: Unassigned
    0x80–0xFE: methods reserved for private use
    */
    auth: Vec<AuthenticationType>,
}

/*
After the server received and processed the handshake request by the client,
it chooses a supported type of authentication and sends it to the client.
 */
pub struct AuthenticationResponse {
    /*
    Version of the SOCKS protocol. Fixed to 0x05 in SOCKS5.
     */
    ver: u8,

    /*
    Chosen type of authentication. If the value is 0xFF it means that
    none of the given authentication is supported by the server.
     */
    cauth: AuthenticationType
}

/*
After the handshake has been completed the client tells the server
the destination to be forwarded to.
 */
pub struct ConnectionRequest {
    /*
    Protocol version. Set to 0x05 in SOCKS5.
     */
    ver: u8,

    /*
    Type of of connection.
        - CONNECT (0x01): Establish a TCP/IP stream connection.
        - BIND (0x02): Establish a TCP/IP port binding.
        - UDP ASSOCIATE (0x03): Associate a UDP port.
     */
    cmd: ConnectionType,

    /*
    Reserved. Must be 0x00.
     */
    rsv: u8,

    /*
    Address type.
        - IPv4 (0x01)
        - Domain (0x03)
        - IPv6 (0x04)
     */
    atyp: AddressType,

    /*
    Address. Depends on atyp.
        - IPv4: Length of 4 Bytes
        - IPv6: Length of 16 Bytes
        - Domain: First octet contains the number of octets of name that follow.
     */
    dst_addr: Vec<u8>,

    /*
    Destination port.
     */
    dst_port: u16
}


pub struct ConnectionResponse {
    /*
    Protocol version. Set to 0x05 in SOCKS5.
     */
    ver: u8,

    /*
    Reply. Status of processed request.

    - 0x00: succeeded
    - 0x01: general SOCKS server failure
    - 0x02: connection not allowed by ruleset
    - 0x03: network unreachable
    - 0x04: host unreachable
    - 0x05: connection refused
    - 0x06: TTL expired
    - 0x07: command not supported
    - 0x08: address type not supported
    - 0x09-0xFF: unassigned
     */
    rep: Reply,

    /*
    Reserved. Set to 0x00.
     */
    rsv: u8,

    /*
    Address type.
        - IPv4 (0x01)
        - Domain (0x03)
        - IPv6 (0x04)
     */
    atyp: AddressType,

    /*
    Address. Depends on atyp.
        - IPv4: Length of 4 Bytes
        - IPv6: Length of 16 Bytes
        - Domain: First octet contains the number of octets of name that follow.
     */
    bind_addr: Vec<u8>,

    /*
     Server bound port (in network octet order).
     */
    bind_port: u16
}