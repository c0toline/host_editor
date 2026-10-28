use app_core::{read,paths};
use app_core::backup::{check_backup_status, BackupStatus, backup_hosts};
use app_core::permissions::check_backup_destination;
use std::{io};
use std::io::Write;
use std::path::Path;

fn backup_decision() -> io::Result<bool> {

    loop {
        print!("Desea realizar una copia de seguridad del fichero (y/n): ");
        io::stdout().flush()?;
        let mut decision_input = String::new();
        io::stdin()
            .read_line(&mut decision_input)?;

        let decision: char = match decision_input.trim().parse::<char>(){
            Ok(decision) => decision,
            Err(error) => {
                eprintln!("Entrada invalida: {}", error);
                continue;
            }
        };
        if decision == 'y' {
            return Ok(true)

        } else if decision == 'n' {
            return Ok(false)
        } else {
            println!("Opción invalida");
            continue;
        }
    }

}

fn main() -> std::io::Result<()>{
    let path = paths::default_hosts_path();
    let backup_path = paths::default_backup_path();
    if path != backup_path {
        match check_backup_status(&path, &backup_path)?{
            BackupStatus::Empty => { println!("Se encontro un backup vacío"); },
            BackupStatus::SameSize => { println!("Ambos ficheros tienen el mismo tamaño"); },
            BackupStatus::DifferentSize => { println!("Los ficheros hosts discrepan en tamaño"); },
            BackupStatus::NotFound => { println!("No se encontró el fichero de backup"); },
        }
    }
    match backup_decision() {
        Ok(true) => {
            check_backup_destination(&backup_path)?;
            let bytes = backup_hosts(&path, &backup_path)?;
            println!("Backup realizado: {:?} bytes", bytes);
        },
        Ok(false) => {
            println!("Backup omitido");
        },
        Err(error) => {
            eprintln!("Error al indicar la decision {}", error)
        }
    };
    let hosts = read::read_hosts(&path)?;
    println!("{:#?}", hosts);
    Ok(())

}


