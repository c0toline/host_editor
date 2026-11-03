use std::fs;
use std::io::ErrorKind;
use app_core::permissions;

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
    let file_on_parent = std::fs::write(&parent, b"contenido de prueba")
        .expect("No se pudo crear el archivo de prueba");
    let destination = parent.join("hosts.bak");
    let result = permissions::check_backup_destination(&destination);
    assert!(result.is_err());
    
}