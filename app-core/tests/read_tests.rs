use app_core::read::read_hosts;
use std::path::Path;
use std::io::ErrorKind;

#[test]
fn test_read_hosts_valid_file(){
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hosts_example.txt");
    let hosts = read_hosts(&path)
        .expect("No se pudo leer el fichero");
}

#[test]
fn test_read_hosts_valid_number_of_entries(){
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hosts_example.txt");
    let hosts = read_hosts(&path)
        .expect("No se pudo leer el fichero");

    assert_eq!(hosts.len(), 4);
}

#[test]
fn test_read_nonexistent_file(){
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("nonexistent.txt");
    let hosts = read_hosts(&path);
    let error = hosts.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::NotFound);
}

#[test]
fn test_read_empty_file(){
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hosts_example2.txt");
    let hosts = read_hosts(&path)
        .expect("No se pudo leer el fichero");
    assert!(hosts.is_empty());
}

#[test]
fn test_oringin_line_number(){
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hosts_example.txt");

    let hosts = read_hosts(&path)
        .expect("No se pudo leer el fichero");
    assert_eq!(hosts[0].line_number(), 16);
    assert_eq!(hosts[1].line_number(), 17);
    assert_eq!(hosts[2].line_number(), 20);
    assert_eq!(hosts[3].line_number(), 21);

}