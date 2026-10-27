use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::hosts::HostEntry;

pub fn read_hosts(path: &Path) -> io::Result<Vec<HostEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries: Vec<HostEntry> = Vec::new();
    for (index, line) in reader.lines().enumerate() {


        if let Some(entry) = HostEntry::parse(index, &line?){
            entries.push(entry);
        };

    }
    Ok(entries)
}