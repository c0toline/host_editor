use std::net::{IpAddr};

#[derive(Debug)]
pub struct HostEntry {
    line_number: usize,
    ip: IpAddr,
    hostnames: Vec<String>,
    description: String,
    enabled: bool
}

impl HostEntry {
    pub fn parse(index: usize, line: &str) -> Option<Self> {
        let raw_line_content = line;
        let raw_line_trimmed_content = raw_line_content.trim();
        let is_enabled = !raw_line_trimmed_content.starts_with("#");
        let line_number = index + 1;
        let line_string = raw_line_trimmed_content
            .strip_prefix("#")
            .unwrap_or(raw_line_trimmed_content);
        let (content, description) = match line_string.split_once("#"){
            Some((content, description)) => (content, description.trim().to_string()),
            None => (line_string, "".to_string())
        };

        let mut parts = content.split_whitespace();
        let str_ip = match parts.next(){
            Some(ip) => ip,
            None => return None,
        };

        let ip = match str_ip.parse::<IpAddr>() {
            Ok(ip) => ip,
            Err(_) => return None
        };

        let hostnames = parts
            .map(|host| host.to_string())
            .collect::<Vec<String>>();

        if hostnames.is_empty() {
            return None;
        }

        let entry = HostEntry{
            line_number,
            ip,
            hostnames,
            description,
            enabled: is_enabled
        };

        Some(entry)

    }
    
    pub fn line_number(&self) -> usize { self.line_number }
}

#[cfg(test)]
#[path = "host_tests.rs"]
mod tests;