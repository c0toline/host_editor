use std::{fs};
use std::io::{ErrorKind};
use app_core::{permissions};
use app_core::backup::backup_hosts;

#[test]
fn test_backup_destination_valid_directory(){
    // Directorio temporal de pruebas
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    // Ruta de backup dentro del directorio temporal
    let destination = temp_dir.path().join("hosts.bak");
    // Llamada a check_backup_destination
    permissions::check_backup_destination(&destination)
        .expect("No se pudo validar el directorio de destino");
}

#[test]
fn test_temp_file_is_removed() {
    // Directorio temporal de pruebas
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    // Ruta de backup dentro del directorio temporal
    let destination = temp_dir.path().join("hosts.bak");
    // Llamada a check_backup_destination
    let result = permissions::check_backup_destination(&destination);
    assert!(result.is_ok());
    let temp_file = fs::read_dir(temp_dir.path())
        .unwrap()
        .count();
    assert_eq!(temp_file, 0);
}

#[test]
fn test_backup_destination_non_existing_directory(){
    // Directorio temporal
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    // Destino inexistente
    let destination = temp_dir.path()
        .join("nonexistent")
        .join("hosts.bak");
    let result = permissions::check_backup_destination(&destination);
    let error = result.err().unwrap();
    assert_eq!(error.kind(), ErrorKind::NotFound);
}

#[test]
fn test_parent_folder_is_file() {
    // Directorio temporal
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    let parent = temp_dir.path().join("file.txt");
    fs::write(&parent, b"contenido de prueba")
        .expect("No se pudo crear el archivo de prueba");
    let destination = parent.join("hosts.bak");
    let result = permissions::check_backup_destination(&destination);
    assert!(result.is_err());

}

#[test]
fn test_backup_hosts_new_backup(){
    // Directorio temporal
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    let origin = temp_dir.path().join("hosts");
    let content = b"contenido de prueba";
    fs::write(&origin, content)
        .expect("No se pudo crear el archivo origen");
    let bytes_in_origin = content.len();
    let destination = temp_dir.path().join("hosts.bak");
    let bytes = backup_hosts(&origin, &destination, false)
        .expect("No se pudo crear el backup");
    assert_eq!(bytes, bytes_in_origin as u64);
    let original_content = fs::read(&origin)
        .expect("No se pudo leer el contenido origen");
    let backup_content = fs::read(&destination)
        .expect("No se pudo leer el contenido destino");
    assert_eq!(backup_content, original_content);
}

#[test]
fn test_backup_hosts_not_overwrites_existing_backup(){
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    let origin = temp_dir.path().join("hosts");
    let destination = temp_dir.path().join("hosts.bak");
    let original_content = b"contenido de prueba";
    let previous_content = b"backup anterior";
    fs::write(&origin, original_content)
        .expect("No se pudo crear el archivo origen");

    fs::write(&destination, previous_content)
        .expect("No se pudo crear el anterior fichero destino");

    let result = backup_hosts(&origin, &destination, false);
    let error = result.expect_err("Se esperaba un error porque el backup ya existe");
    assert_eq!(error.kind(), ErrorKind::AlreadyExists);
    let backup_content = fs::read(&destination)
        .expect("No se pudo leer el contenido desino");
    assert_eq!(backup_content, previous_content);
    let remaining_files = fs::read_dir(temp_dir.path())
        .expect("No se pudo leer el directorio temporal")
        .count();
    assert_eq!(remaining_files, 2);
}

#[test]
fn test_backup_hosts_origin_not_found() {
    let temp_dir = tempfile::tempdir().expect("No se pudo crear el directorio temporal");
    let origin = temp_dir.path().join("hosts");
    let destination = temp_dir.path().join("hosts.bak");
    let result = backup_hosts(&origin, &destination, false);
    let error = result.expect_err("Se esperaba un error porque el origen no existe");
    assert_eq!(error.kind(), ErrorKind::NotFound);
    assert!(!destination.exists());
}

#[test]
fn test_backup_hosts_overwrite_unsupported(){
    let temp_dir = tempfile::tempdir()
        .expect("No se pudo crear el directorio temporal");

    let origin = temp_dir.path().join("hosts");
    let original_content = b"contenido nuevo";

    fs::write(&origin, original_content)
        .expect("No se pudo crear el archivo origen");
    let destination = temp_dir.path().join("hosts.bak");
    let previous_content = b"contenido de prueba";
    fs::write(&destination, previous_content)
        .expect("No se pudo crear el fichero de backup anterior");
    let result = backup_hosts(&origin, &destination, true);
    let error = result.expect_err("Se esperaba un error ya que no esta creada la implementación");
    assert_eq!(error.kind(), ErrorKind::Unsupported);
    let backup_content = fs::read(&destination)
        .expect("No se pudo leer el backup anterior");

    assert_eq!(backup_content, previous_content);
}

#[test]
fn test_backup_hosts_destination_not_found(){
    let temp_dir = tempfile::tempdir()
        .expect("No se pudo crear el directorio temporal");
    let origin = temp_dir.path().join("hosts");
    let original_content = b"contenido nuevo";
    fs::write(&origin, original_content)
        .expect("No se pudo crear el archivo origen");
    let destination = temp_dir.path().join("nonexistent").join("hosts.bak");
    let result = backup_hosts(&origin, &destination, false);
    let error = result.expect_err("Se esperaba un error porque el destino no existe");
    assert_eq!(error.kind(), ErrorKind::NotFound);

}