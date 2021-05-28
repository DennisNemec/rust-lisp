pub mod constants;
pub mod header;

use constants::*;

/* Traits */
pub trait Serialization<T> {
    fn serialize(self) -> Vec<u8>;
    fn deserialize(binary: Vec<u8>) -> Result<T, String>;
}

/* Helper functions */
pub fn extract_address_from_bytes_arr(binary: &[u8], atyp: &AddressType) -> Vec<u8> {
    return if let AddressType::DNS = atyp {
        binary[1..].to_vec()
    } else {
        binary.to_vec()
    };
}

pub fn extract_port_from_bytes_arr(binary: &[u8]) -> u16 {
    ((binary[0].clone() as u16) << 8) | binary[1].clone() as u16
}

pub fn extract_auth_type_from_byte(byte: u8) -> Authentication {
    return match byte {
        0x00 => Authentication::Assigned(AuthenticationType::NoAuthentication),
        0x01 => Authentication::Assigned(AuthenticationType::GSSAPI),
        0x02 => Authentication::Assigned(AuthenticationType::UsernamePassword),
        0x03 => Authentication::Assigned(AuthenticationType::ChallengeHandshakeAuthentication),
        0x04 => Authentication::Assigned(AuthenticationType::Unassigned),
        0x05 => Authentication::Assigned(AuthenticationType::ChallengeResponseAuthentication),
        0x06 => Authentication::Assigned(AuthenticationType::SecureSocketsLayer),
        0x07 => Authentication::Assigned(AuthenticationType::NdsAuthentication),
        0x08 => Authentication::Assigned(AuthenticationType::MultiAuthenticationFramework),
        0x09 => Authentication::Assigned(AuthenticationType::JsonParameterBlock),
        0x80..=0xFE => Authentication::Custom(byte),
        _ => Authentication::Assigned(AuthenticationType::NoAuthentication),
    };
}

pub fn extract_auth_u8_from_auth_type(auth: Authentication) -> Option<u8> {
    if let Authentication::Assigned(a) = auth {
        Some(a as u8)
    } else if let Authentication::Custom(u) = auth {
        Some(u)
    } else {
        None // never happens
    }
}

pub fn extract_atyp_from_byte(atyp_byte: &u8) -> AddressType {
    return match atyp_byte {
        0x01 => AddressType::IPv4,
        0x04 => AddressType::IPv6,
        0x03 => AddressType::DNS,
        _ => AddressType::IPv4,
    };
}
