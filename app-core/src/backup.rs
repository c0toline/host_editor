use std::fs;
use std::io::{self, Error, ErrorKind, BufReader, Write};
use std::path::Path;
use tempfile;

#[derive(Debug, PartialEq, Eq)]
pub enum BackupStatus {
    NotFound,
    Empty,
    SameSize,
    DifferentSize,
}

pub fn check_backup_status(origin: &Path, destination: &Path) -> io::Result<BackupStatus>  {
    let origin_metadata = fs::metadata(origin)?;
    if !origin_metadata.is_file() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "El origen no es un archivo regular",
        ));
    }

    match destination.metadata() {
        Ok(metadata) if metadata.is_file() =>{
            if metadata.len() == 0 {
                Ok(BackupStatus::Empty)
            } else if metadata.len() == origin_metadata.len() {
                Ok(BackupStatus::SameSize)
            } else {
                Ok(BackupStatus::DifferentSize)
            }
        },
        Ok(_) => Err(Error::new(
            ErrorKind::InvalidInput,
            "El destino no es un fichero regular")),
        Err(error) if error.kind() == ErrorKind::NotFound =>{
            Ok(BackupStatus::NotFound)
        },
        Err(error) => Err(error),
    }
}

pub fn backup_hosts(origin: &Path, destination: &Path, overwrite: bool) -> io::Result<u64>{
    /*let bytes =  fs::copy(origin, destination)?;
    Ok(Some(bytes))*/
    if overwrite {
        return Err(io::Error::new(
            ErrorKind::Unsupported,
            "La sobreescrituro esta pendiente de implementación"
        ));
    }
    let mut origin_file = BufReader::new(fs::File::open(origin)?);
    let metadata = origin_file.get_ref().metadata()?;
    if !metadata.is_file() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "No se ha indicado un fichero origen válido"
        ))
    }

    let destination_parent_folder = destination.parent()
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "La ruta destino no tiene directorio padre"))?;

    let mut temp_destination_file = tempfile::NamedTempFile::new_in(destination_parent_folder)?;
    let bytes = io::copy(&mut origin_file, &mut temp_destination_file)?;
    temp_destination_file.flush()?;
    temp_destination_file.as_file().sync_all()?;
    temp_destination_file.persist_noclobber(destination)?;
    Ok(bytes)
}