use crate::protocol::constants::*;
use crate::protocol::Serialization;

use crate::protocol::{extract_auth_type_from_byte, extract_auth_u8_from_auth_type};

/*
In order to establish a SOCKS5 connection the client needs to
do a handshake with the SOCKS5 server. The client tells the server
his supported protocol of authentication methods.
 */
#[derive(PartialEq, Debug)]
pub struct AuthenticationRequest {
    /*
    Version of the used protocol. Fixed value of 0x05.
    */
    pub ver: u8,

    /*
    Amount of supported authentication protocols by the client.
     */
    pub nauth: u8,

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
    pub auth: Vec<Authentication>,
}

/*
After the server received and processed the handshake request by the client,
it chooses a supported type of authentication and sends it to the client.
 */
#[derive(PartialEq, Debug)]
pub struct AuthenticationResponse {
    /*
    Version of the SOCKS protocol. Fixed to 0x05 in SOCKS5.
     */
    ver: u8,

    /*
    Chosen type of authentication. If the value is 0xFF it means that
    none of the given authentication is supported by the server.
     */
    cauth: Authentication,
}

/* Trait implementation */

impl Serialization<AuthenticationRequest> for AuthenticationRequest {
    fn serialize(self) -> Vec<u8> {
        let mut auth_binary: Vec<u8> = Vec::new();
        auth_binary.push(self.ver);
        auth_binary.push(self.nauth);
        auth_binary.append(&mut self.auth.into_iter().map(|auth| {
            extract_auth_u8_from_auth_type(auth).unwrap()
        }).collect::<Vec<u8>>());

        auth_binary
    }

    fn deserialize(binary: Vec<u8>) -> Result<AuthenticationRequest, String> {
        return if binary.len() < 3 {
            Err("Invalid binary format.".to_string())
        } else {
            Ok(AuthenticationRequest {
                ver: binary[0],
                nauth: binary[1],
                auth: binary[2..binary.len()].iter().map(|&auth_method| extract_auth_type_from_byte(auth_method)).collect::<Vec<Authentication>>()
            })
        };
    }
}

impl Serialization<AuthenticationResponse> for AuthenticationResponse {
    fn serialize(self) -> Vec<u8> {
        let mut auth_response_binary: Vec<u8> = Vec::new();

        auth_response_binary.push(self.ver);
        auth_response_binary.push(extract_auth_u8_from_auth_type(self.cauth).unwrap());

        auth_response_binary
    }

    fn deserialize(binary: Vec<u8>) -> Result<AuthenticationResponse, String> {
        if binary.len() < 2 {
            Err("Invalid binary format.".to_string())
        } else {
            let auth_type: Authentication = extract_auth_type_from_byte(binary[1]);

            Ok(AuthenticationResponse {
                ver: binary[0],
                cauth: auth_type,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_authentication_request() -> AuthenticationRequest {
        AuthenticationRequest {
            ver: 0x05,
            nauth: 0x01,
            auth: vec![Authentication::Assigned(AuthenticationType::NoAuthentication)],
        }
    }

    fn get_example_authentication_request_serialized() -> Vec<u8> {
        vec![0x05, 0x01, 0x00]
    }

    fn get_example_authentication_response() -> AuthenticationResponse {
        AuthenticationResponse {
            ver: 0x05,
            cauth: Authentication::Assigned(AuthenticationType::NoAuthentication),
        }
    }

    fn get_example_authentication_response_serialized() -> Vec<u8> {
        vec![0x05, 0x00] // 0x00 = AuthenticationType::NoAuthentication
    }

    // Standard functionality. No edge cases.

    // AuthenticationRequest
    #[test]
    fn test_auth_request_serialize() {
        let auth_request_obj = get_example_authentication_request();
        let auth_request_serialized = get_example_authentication_request_serialized();

        assert_eq!(auth_request_obj.serialize(), auth_request_serialized)
    }

    #[test]
    fn test_auth_request_deserialize() {
        let auth_request_binary = get_example_authentication_request_serialized();
        let auth_request_obj = AuthenticationRequest::deserialize(auth_request_binary).unwrap();

        assert_eq!(auth_request_obj, get_example_authentication_request())
    }

    // AuthenticationResponse
    #[test]
    fn test_auth_response_deserialize() {
        let auth_response_vec: Vec<u8> = get_example_authentication_response_serialized();
        let auth_response_obj: AuthenticationResponse =
            AuthenticationResponse::deserialize(auth_response_vec).unwrap();

        assert_eq!(auth_response_obj, get_example_authentication_response())
    }

    #[test]
    fn test_auth_response_serialize() {
        let auth_response_obj = get_example_authentication_response();

        assert_eq!(
            auth_response_obj.serialize(),
            get_example_authentication_response_serialized()
        )
    }

}
