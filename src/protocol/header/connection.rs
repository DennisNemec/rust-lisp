use crate::protocol::constants::*;
use crate::protocol::{extract_port_from_bytes_arr, extract_atyp_from_byte, extract_address_from_bytes_arr, Serialization};

/*
After the handshake has been completed the client tells the server
the destination to be forwarded to.
 */
#[derive(PartialEq, Debug)]
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
    dst_port: u16,
}

#[derive(PartialEq, Debug)]
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
    bind_port: u16,
}


impl Serialization<ConnectionRequest> for ConnectionRequest {
    fn serialize(self) -> Vec<u8> {
        let mut connection_request_binary: Vec<u8> = Vec::new();
        connection_request_binary.push(self.ver);
        connection_request_binary.push(self.cmd as u8);
        connection_request_binary.push(self.rsv);
        connection_request_binary.push(self.atyp as u8);
        connection_request_binary.append(self.dst_addr.clone().as_mut());

        // split 16 bit port to two 8 bit.
        connection_request_binary.push(((self.dst_port >> 8) as u8).clone()); // high
        connection_request_binary.push((self.dst_port as u8).clone()); // low

        connection_request_binary
    }

    fn deserialize(binary: Vec<u8>) -> Result<ConnectionRequest, String> {
        return if binary.len() < 9 {
            Err("Invalid binary format".to_string())
        } else {
            let ver = binary[0];
            let connection_type = match binary[1] {
                0x01 => ConnectionType::Connect,
                0x02 => ConnectionType::Bind,
                0x03 => ConnectionType::UdpAssociate,
                _ => ConnectionType::Connect,
            };

            let rsv = binary[2];
            let atyp = extract_atyp_from_byte(&binary[3]);
            let address = extract_address_from_bytes_arr(&binary[4..binary.len() - 2], &atyp);
            let port = extract_port_from_bytes_arr(&binary[binary.len() - 2..]);

            Ok(ConnectionRequest {
                ver,
                cmd: connection_type,
                rsv,
                atyp,
                dst_addr: address,
                dst_port: port,
            })
        };
    }
}

impl Serialization<ConnectionResponse> for ConnectionResponse {
    fn serialize(self) -> Vec<u8> {
        let mut connection_response_binary = Vec::new();
        connection_response_binary.push(self.ver);
        connection_response_binary.push(self.rep as u8);
        connection_response_binary.push(self.rsv);
        connection_response_binary.push(self.atyp as u8);
        connection_response_binary.append(self.bind_addr.clone().as_mut());
        connection_response_binary.push((self.bind_port >> 8) as u8);
        connection_response_binary.push(self.bind_port as u8);

        connection_response_binary
    }

    fn deserialize(binary: Vec<u8>) -> Result<ConnectionResponse, String> {
        return if binary.len() < 9 {
            Err("Invalid binary format".to_string())
        } else {
            let ver = binary[0];
            let reply = match binary[1] {
                0x00 => Reply::Succeeded,
                0x01 => Reply::GeneralSocksServerFailure,
                0x02 => Reply::ConnectionNotAllowedByRuleset,
                0x03 => Reply::NetworkUnreachable,
                0x04 => Reply::HostUnreachable,
                0x05 => Reply::ConnectionRefused,
                0x06 => Reply::TTLExpired,
                0x07 => Reply::CommandNotSupported,
                0x08 => Reply::AddressTypeNotSupported,
                _ => Reply::GeneralSocksServerFailure,
            };

            let rsv = binary[2];
            let atyp = extract_atyp_from_byte(&binary[3]);
            let address = extract_address_from_bytes_arr(&binary[4..binary.len() - 2], &atyp);
            let port = extract_port_from_bytes_arr(&binary[binary.len() - 2..]);

            Ok(ConnectionResponse {
                ver,
                rep: reply,
                rsv,
                atyp,
                bind_addr: address,
                bind_port: port,
            })
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_connection_request() -> ConnectionRequest {
        ConnectionRequest {
            ver: 0x05,
            cmd: ConnectionType::Connect,
            rsv: 0x00,
            atyp: AddressType::IPv4,
            dst_addr: vec![127, 0, 0, 1],
            dst_port: 1337,
        }
    }

    fn get_example_connection_response() -> ConnectionResponse {
        ConnectionResponse {
            ver: 0x05,
            rep: Reply::Succeeded,
            rsv: 0x00,
            atyp: AddressType::IPv4,
            bind_addr: vec![127, 0, 0, 1],
            bind_port: 1337,
        }
    }

    fn get_example_connection_request_serialized() -> Vec<u8> {
        vec![0x05, 0x01, 0x00, 0x01, 0x7F, 0x00, 0x00, 0x01, 0x05, 0x39]
    }

    fn get_example_connection_response_serialized() -> Vec<u8> {
        vec![0x05, 0x00, 0x00, 0x01, 0x7F, 0x00, 0x00, 0x01, 0x05, 0x39]
    }

    // Standard functionality. No edge cases.

    // ConnectionRequest
    #[test]
    fn test_connection_request_serialize() {
        assert_eq!(
            get_example_connection_request().serialize(),
            get_example_connection_request_serialized()
        )
    }

    #[test]
    fn test_connection_request_deserialize() {
        let connection_request_obj =
            ConnectionRequest::deserialize(get_example_connection_request_serialized()).unwrap();

        assert_eq!(connection_request_obj, get_example_connection_request())
    }

    // ConnectionRequest
    #[test]
    fn test_connection_response_serialize() {
        assert_eq!(
            get_example_connection_response().serialize(),
            get_example_connection_response_serialized()
        )
    }

    #[test]
    fn test_connection_response_deserialize() {
        let connection_response_obj =
            ConnectionResponse::deserialize(get_example_connection_response_serialized()).unwrap();

        assert_eq!(connection_response_obj, get_example_connection_response())
    }
}
