use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::hosts::HostEntry;

#[test]
fn test_parse_host_disable(){
    let host = "# 1.1.1.1 unhost soy.unhost.com # Host de test 1";
    let index: usize = 0;
    let result = HostEntry::parse(index, host);

    assert!(result.is_some());

    let entry = result.expect("Se esperaba una entrada válida");
    assert_eq!(entry.line_number, 1);
    assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
    assert_eq!(entry.hostnames, vec!["unhost".to_string(), "soy.unhost.com".to_string()]);
    assert_eq!(entry.description, "Host de test 1");
    assert_eq!(entry.enabled, false);
}

#[test]
fn test_parse_host_disable_without_description(){
    let host = "# 1.1.1.1 unhost soy.unhost.com";
    let index: usize = 1;
    let result = HostEntry::parse(index, host);
    assert!(result.is_some());

    let entry = result.expect("Se esperaba una entrada válida");
    assert_eq!(entry.line_number, 2);
    assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
    assert_eq!(entry.hostnames, vec!["unhost".to_string(), "soy.unhost.com".to_string()]);
    assert_eq!(entry.description, "");
    assert_eq!(entry.enabled, false);
}

#[test]
fn test_parse_host_enable(){
    let host = " 1.1.1.1 unhost soy.unhost.com # Soy un host habilitado";
    let index: usize = 2;
    let result = HostEntry::parse(index, host);
    assert!(result.is_some());

    let entry = result.expect("Se esperaba una entrada válida");
    assert_eq!(entry.line_number, 3);
    assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
    assert_eq!(entry.hostnames, vec!["unhost".to_string(), "soy.unhost.com".to_string()]);
    assert_eq!(entry.description, "Soy un host habilitado");
    assert_eq!(entry.enabled, true);
}

#[test]
fn test_parse_host_enable_without_description(){
    let host = " 1.1.1.1 unhost soy.unhost.com";
    let index: usize = 3;
    let result = HostEntry::parse(index, host);
    assert!(result.is_some());

    let entry = result.expect("Se esperaba una entrada válida");
    assert_eq!(entry.line_number, 4);
    assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)));
    assert_eq!(entry.hostnames, vec!["unhost".to_string(), "soy.unhost.com".to_string()]);
    assert_eq!(entry.description, "");
    assert_eq!(entry.enabled, true);
}

#[test]
fn test_parse_host_with_invalid_ip(){
    let host = "999.999.999.999 servidor servidor.local # Soy una linea con una ip inválida";
    let index: usize = 4;
    let result = HostEntry::parse(index, host);
    assert!(result.is_none());

}

#[test]
fn test_parse_host_without_hostname(){
    let host = "1.1.1.1 # No tengo hostname";
    let index: usize = 5;
    let result = HostEntry::parse(index, host);
    assert!(result.is_none());
}

#[test]
fn test_parse_host_with_ipvsix(){
    let host = "::1 localhost # IPV6 hacia localhost";
    let index: usize = 6;
    let result = HostEntry::parse(index, host);
    assert!(result.is_some());

    let entry = result.expect("Se esperaba una entrada válida");
    assert_eq!(entry.line_number, 7);
    assert_eq!(entry.ip, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
    assert_eq!(entry.hostnames, vec!["localhost".to_string()]);
    assert_eq!(entry.description, "IPV6 hacia localhost");
    assert_eq!(entry.enabled, true);
}

#[test]
fn test_parse_host_stating_with_whitespace(){
    let host = "   # 8.8.8.8 google google.com # Destino google";
    let index: usize = 7;
    let result = HostEntry::parse(index, host);
    assert!(result.is_some());

    let entry = result.expect("Se esperaba una entrada valida");
    assert_eq!(entry.line_number, 8);
    assert_eq!(entry.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
    assert_eq!(entry.hostnames, vec!["google".to_string(), "google.com".to_string()]);
    assert_eq!(entry.description, "Destino google");
    assert_eq!(entry.enabled, false);
}

#[test]
fn test_parse_comment(){
    let host = "# No soy un host soy host soy un comentario";
    let index: usize = 8;
    assert!(HostEntry::parse(index, host).is_none());
}

#[test]
fn test_parse_empty_line(){
    let host = " ";
    let index: usize = 9;
    assert!(HostEntry::parse(index, host).is_none());
}
