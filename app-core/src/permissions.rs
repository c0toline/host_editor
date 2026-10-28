use std::{fs, io};
use std::path::Path;
use tempfile;

pub fn check_backup_destination(destination: &Path) -> io::Result<()>{

    let parent_folder = match destination.parent() {
        Some(parent) => parent,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "La ruta no tiene directorio padre"
            ));
        }
    };

    let metadata = fs::metadata(parent_folder)?;
    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "La ruta no es un directorio válido"
        ));
    }

    let _checker = tempfile::NamedTempFile::new_in(parent_folder)?;

    drop(_checker);
    Ok(())
}