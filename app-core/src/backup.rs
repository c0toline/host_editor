use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

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
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
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

pub fn backup_hosts(origin: &Path, destination: &Path) -> io::Result<Option<u64>>{
    let bytes =  fs::copy(origin, destination)?;
    Ok(Some(bytes))


}