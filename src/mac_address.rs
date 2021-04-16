use std::str::Chars;
use regex::Regex;

pub struct MacAddress
{
    pub address_array: [u8; 6]
}

impl MacAddress
{
    pub fn new() -> Self
    {
        MacAddress { address_array: [0, 0, 0, 0, 0, 0] as [u8; 6] }
    }

    pub fn from_array(mac: &[u8; 6]) -> Self
    {
        MacAddress { address_array: *mac }
    }

    pub fn from_string_slice(mac: &str) -> Option<Self>
    {
        if !MacAddress::is_valid_mac_address(&mac) {
            return None;
        }

        Some(MacAddress::string_slice_to_mac(&mac))
    }
}

impl MacAddress
{
    fn is_valid_mac_address(mac: &str) -> bool
    {
        let r: Regex = Regex::new(r"^(([0-9A-Fa-f]){2}:){5}([0-9A-Fa-f]){2}$").unwrap();

        r.is_match(mac)
    }

    fn string_slice_to_mac(mac: &str) -> Self
    {
        let octets: Vec<&str> = MacAddress::split_mac_by_colons(&mac);
        let mut result: [u8; 6] = [0, 0, 0, 0, 0, 0];
        let mut i: usize = 0;

        for octet in octets {
            result[i] = MacAddress::octet_chars_to_byte(&octet.chars());
            i += 1;
        }

        MacAddress::from_array(&result)
    }

    fn split_mac_by_colons(mac: &str) -> Vec<&str>
    {
        mac.split(':').collect::<Vec<&str>>()
    }

    fn octet_chars_to_byte(octet_chars: &Chars) -> u8
    {
        let first_half_byte: u8  = MacAddress::char_to_digit(&octet_chars.clone().nth(1).unwrap()).unwrap();
        let second_half_byte: u8 = MacAddress::char_to_digit(&octet_chars.clone().nth(0).unwrap()).unwrap() * 16;

        first_half_byte + second_half_byte
    }

    fn char_to_digit(hex: &char) -> Option<u8>
    {
        match hex {
            r @'0'..='9' | r @'A'..='F' | r @'a'..='f' => {
                Some(r.to_digit(16).unwrap() as u8)
            },
            _ => None
        }
    }
}

#[cfg(test)]
pub mod mac_address_test
{
    use std::u8;
    use std::collections::HashMap;

    use super::*;

    #[test]
    pub fn char_to_digit_test()
    {
        let char_hex_map: HashMap<char, u8> = [
                ('0', 0),
                ('1', 1),
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('A', 10),
                ('B', 11),
                ('C', 12),
                ('D', 13),
                ('E', 14),
                ('F', 15),
                ('a', 10),
                ('b', 11),
                ('c', 12),
                ('d', 13),
                ('e', 14),
                ('f', 15)
            ].iter().cloned().collect();

            for (key, value) in char_hex_map.iter() {
                assert_eq!(MacAddress::char_to_digit(key).unwrap(), *value);
            }
    }

    #[test]
    pub fn from_string_slice_integration_test()
    {
        let input: &str = "55:EE:75:BF:9C:A1";
        let expected_result: [u8; 6] = [0x55, 0xEE, 0x75, 0xBF, 0x9C, 0xA1];
        
        let result: MacAddress = MacAddress::from_string_slice(&input).unwrap();

        assert_eq!(result.address_array, expected_result)
    }
}