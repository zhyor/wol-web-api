use regex::Regex;
use std::net::IpAddr;
use std::net::UdpSocket;
use std::str::FromStr;

fn parse_mac_address(mac: &str) -> Option<[u8; 6]> {
    let re =
        Regex::new(r"^(([0-9A-Fa-f]{2}[-]){5}|([0-9A-Fa-f]{2}[:]){5})[0-9A-Fa-f]{2}$").unwrap();
    if !re.is_match(mac) {
        return None;
    }
    let re = Regex::new(r"[:-]").unwrap();
    let parts: Vec<&str> = re.split(mac).collect();
    let mut bytes = [0u8; 6];
    for (i, part) in parts.iter().enumerate() {
        bytes[i] = u8::from_str_radix(part, 16).ok()?;
    }
    Some(bytes)
}

fn create_magic_packet(mac: &[u8; 6]) -> [u8; 102] {
    let mut packet = [0xFF; 102];

    for i in 1..=16 {
        packet[i * 6..(i + 1) * 6].copy_from_slice(mac);
    }
    packet
}

pub fn wake_on_lan(mac: &str) -> Option<()> {
    let mac_bytes = match parse_mac_address(mac) {
        Some(v) => v,
        None => return None,
    };
    let magic_packet = create_magic_packet(&mac_bytes);
    let broadcast_ip = IpAddr::from_str("255.255.255.255").unwrap();
    let port = 9;
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket.set_broadcast(true).unwrap();
    socket.send_to(&magic_packet, (broadcast_ip, port)).unwrap();
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_format() {
        let mut result = parse_mac_address("00:11-22-33:44-55");
        match result {
            Some(v) => panic!("{:?}", v),
            None => println!("OK"),
        }
        result = parse_mac_address("00:11:22:33:44:55:123");
        match result {
            Some(v) => panic!("{:?}", v),
            None => println!("OK"),
        }
        result = parse_mac_address("00-11-22-33-44-55-12");
        match result {
            Some(v) => panic!("{:?}", v),
            None => println!("OK"),
        }
        result = parse_mac_address("00-11:22-33-44-55");
        match result {
            Some(v) => panic!("{:?}", v),
            None => println!("OK"),
        }
        result = parse_mac_address("00:11:22:33:44:55");
        match result {
            Some(v) => println!("OK: {:?}", v),
            None => panic!("Err"),
        }
        result = parse_mac_address("00-11-22-33-44-55");
        match result {
            Some(v) => println!("OK: {:?}", v),
            None => panic!("Err"),
        }
    }

    #[test]
    fn test_magic_packet(){
        let mac = "6C:1F:F7:0C:A1:4F";
        let expected= [
            255, 255, 255, 255, 255, 255, 108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79,
            108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79,
            108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79,
            108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79,
            108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79,
            108, 31, 247, 12, 161, 79, 108, 31, 247, 12, 161, 79,
        ];
        let bytes= parse_mac_address(mac).unwrap();
        let output = create_magic_packet(&bytes);
        assert_eq!(output,expected);
    }
}
