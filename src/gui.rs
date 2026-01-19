use sysinfo::{Pid, System};
use std::fs;
use std::io::BufRead;
use serde::{Deserialize, Serialize};

use crate::port_monitor::PortMonitor;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub process_name: String,
    pub pid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortDetails {
    pub port: u16,
    pub process_name: String,
    pub pid: u32,
    pub memory_mb: f64,
    pub cpu_usage: f32,
}

pub struct GuiBackend {
    monitor: PortMonitor,
}

impl GuiBackend {
    pub fn new() -> Self {
        GuiBackend {
            monitor: PortMonitor::new(),
        }
    }

    pub fn get_all_ports(&self) -> anyhow::Result<Vec<PortInfo>> {
        let proc_net_file = "/proc/net/tcp";
        if !std::path::Path::new(proc_net_file).exists() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(proc_net_file)?;
        let reader = std::io::BufReader::new(file);
        let mut port_pids = Vec::new();

        for line in reader.lines().skip(1) {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 3 {
                if let Ok(port) = parse_port(&parts[1]) {
                    if let Ok(state) = parts[3].parse::<u32>() {
                        if state == 10 {
                            // LISTEN state
                            if let Ok(inode) = parts[9].parse::<u64>() {
                                if let Some(pid) = find_pid_by_inode(inode) {
                                    port_pids.push((port, pid));
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut system = System::new_all();
        let mut result = Vec::new();
        for (port, pid) in port_pids {
            if let Some(process) = system.process(Pid::from_u32(pid)) {
                result.push(PortInfo {
                    port,
                    process_name: process.name().to_string(),
                    pid,
                });
            }
        }

        result.sort_by_key(|p| p.port);
        Ok(result)
    }

    pub fn kill_process(&self, port: u16) -> anyhow::Result<String> {
        match self.monitor.kill_port(port, true) {
            Ok(_) => Ok(format!("Successfully killed process on port {}", port)),
            Err(e) => Err(anyhow::anyhow!("Failed to kill process: {}", e)),
        }
    }

    pub fn get_port_details(&self, port: u16) -> anyhow::Result<Option<PortDetails>> {
        let ports = self.get_all_ports()?;
        
        if let Some(port_info) = ports.iter().find(|p| p.port == port) {
            let mut system = System::new_all();
            if let Some(process) = system.process(Pid::from_u32(port_info.pid)) {
                return Ok(Some(PortDetails {
                    port: port_info.port,
                    process_name: port_info.process_name.clone(),
                    pid: port_info.pid,
                    memory_mb: process.memory() as f64 / 1024.0,
                    cpu_usage: process.cpu_usage(),
                }));
            }
        }
        
        Ok(None)
    }
}

fn parse_port(s: &str) -> anyhow::Result<u16> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 2 {
        Ok(u16::from_str_radix(parts[1], 16)?)
    } else {
        Err(anyhow::anyhow!("Invalid port format"))
    }
}

fn find_pid_by_inode(inode: u64) -> Option<u32> {
    // First try the fd directory method
    let fd_dir = "/proc";
    if let Ok(entries) = fs::read_dir(fd_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if let Ok(pid) = dir_name.to_string_lossy().parse::<u32>() {
                        let fd_path = path.join("fd");
                        // Check if we can read this directory
                        if let Ok(fds) = fs::read_dir(&fd_path) {
                            for fd_entry in fds.flatten() {
                                let fd_path = fd_entry.path();
                                if let Ok(target) = fs::read_link(&fd_path) {
                                    if let Some(target_str) = target.to_str() {
                                        if target_str.contains(&format!("socket:[{}]", inode)) {
                                            return Some(pid);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: try using ss command
    if let Ok(output) = std::process::Command::new("ss")
        .args(&["-tlnp"])
        .output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines() {
                if line.contains(&format!("{}:", inode)) || 
                   (line.contains("socket") && line.contains(&inode.to_string())) {
                    // Extract PID from the line
                    if let Some(pid_part) = line.split_whitespace().find(|s| s.contains('/')) {
                        if let Some(pid_str) = pid_part.split('/').next() {
                            if let Ok(pid) = pid_str.parse::<u32>() {
                                return Some(pid);
                            }
                        }
                    }
                }
            }
        }
    }
    
    None
}
