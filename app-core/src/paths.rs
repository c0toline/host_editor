use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub fn default_hosts_path() -> PathBuf{
    PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts")
}

#[cfg(target_os = "windows")]
pub fn default_backup_path() -> PathBuf {
    PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts").with_extension("back")
}

#[cfg(target_os = "linux")]
pub fn default_hosts_path() -> PathBuf {
    PathBuf::from(r"/etc/hosts")
}

#[cfg(target_os = "linux")]
pub fn default_backup_path() -> PathBuf {
    PathBuf::from(r"/etc/backup").with_extension("back")
}