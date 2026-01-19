use colored::Colorize;
use nix::unistd::Pid as NixPid;
use nix::sys::signal::{kill, Signal};
use std::fs;
use std::io::{self, BufRead, Write};
use std::time::Duration;
use sysinfo::{Pid, System};
use tokio::time::sleep;

pub struct PortMonitor {
    system: System,
}

impl PortMonitor {
    pub fn new() -> Self {
        PortMonitor {
            system: System::new_all(),
        }
    }

    /// Check which process is using a specific port
    pub fn check_port(&self, port: u16) -> anyhow::Result<()> {
        let processes = self.find_processes_using_port(port)?;

        if processes.is_empty() {
            println!(
                "{}",
                format!("✓ Port {} is {} in use", port, "not".green()).green()
            );
        } else {
            println!("{}", format!("✗ Port {} is {} in use:", port, "in".red()).red());
            for (pid, name) in &processes {
                println!("  {} (PID: {})", name.cyan(), pid);
            }
        }

        Ok(())
    }

    /// List all ports in use
    pub fn list_all_ports(&self) -> anyhow::Result<()> {
        println!("{}", "=== Active Ports ===".bold());

        let proc_net_file = "/proc/net/tcp";
        if !std::path::Path::new(proc_net_file).exists() {
            return Err(anyhow::anyhow!("Unable to read port information on this system"));
        }

        let file = fs::File::open(proc_net_file)?;
        let reader = io::BufReader::new(file);

        let mut ports = Vec::new();

        for line in reader.lines().skip(1) {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 3 {
                if let Ok(port) = parse_port(&parts[1]) {
                    if let Ok(state) = parts[3].parse::<u32>() {
                        if state == 10 {
                            // LISTEN state
                            ports.push(port);
                        }
                    }
                }
            }
        }

        ports.sort();
        ports.dedup();

        if ports.is_empty() {
            println!("{}", "No active listening ports found".yellow());
        } else {
            for port in ports {
                if let Ok(processes) = self.find_processes_using_port(port) {
                    if !processes.is_empty() {
                        let process_info = processes
                            .iter()
                            .map(|(_, name)| name.clone())
                            .collect::<Vec<_>>()
                            .join(", ");
                        println!("  {} → {}", port.to_string().cyan(), process_info);
                    } else {
                        println!("  {} → {}", port.to_string().cyan(), "unknown process".dimmed());
                    }
                }
            }
        }

        Ok(())
    }

    /// Kill a process using a specific port
    pub fn kill_port(&self, port: u16, force: bool) -> anyhow::Result<()> {
        let processes = self.find_processes_using_port(port)?;

        if processes.is_empty() {
            println!("No process found using port {}", port);
            return Ok(());
        }

        println!("Process(es) using port {}:", port.to_string().yellow());
        for (pid, name) in &processes {
            println!("  {} (PID: {})", name.cyan(), pid);
        }

        if !force {
            print!("\n{} ", "Kill these processes?".yellow());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y")
                && !input.trim().eq_ignore_ascii_case("yes")
            {
                println!("{}", "Cancelled".dimmed());
                return Ok(());
            }
        }

        let mut success = 0;
        for (pid, name) in processes {
            match kill(
                NixPid::from_raw(pid as i32),
                Signal::SIGTERM,
            ) {
                Ok(_) => {
                    println!(
                        "{}",
                        format!("✓ Killed {} (PID: {})", name, pid).green()
                    );
                    success += 1;
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!("✗ Failed to kill {} (PID: {}): {}", name, pid, e).red()
                    );
                }
            }
        }

        if success > 0 {
            println!(
                "{}",
                format!("\n✓ Successfully killed {} process(es)", success).green()
            );
        }

        Ok(())
    }

    /// Monitor ports in real-time
    pub async fn monitor_ports(&self, ports_str: &str, interval: u64) -> anyhow::Result<()> {
        let ports: Vec<u16> = ports_str
            .split(',')
            .filter_map(|p| p.trim().parse::<u16>().ok())
            .collect();

        if ports.is_empty() {
            return Err(anyhow::anyhow!("No valid ports provided"));
        }

        println!(
            "{}",
            format!("Monitoring ports: {}", ports_str).bold()
        );
        println!("Press Ctrl+C to stop\n");

        loop {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
            println!("{}", format!("=== Port Monitor [{}] ===", chrono::Local::now().format("%H:%M:%S")).bold());

            for port in &ports {
                match self.find_processes_using_port(*port) {
                    Ok(processes) => {
                        if processes.is_empty() {
                            println!("Port {}: {} (no process)", port.to_string().cyan(), "free".green());
                        } else {
                            let process_info = processes
                                .iter()
                                .map(|(pid, name)| format!("{} ({})", name, pid))
                                .collect::<Vec<_>>()
                                .join(", ");
                            println!("Port {}: {} {}", port.to_string().cyan(), "in use".red(), process_info);
                        }
                    }
                    Err(_) => {
                        println!("Port {}: {}", port.to_string().cyan(), "error".yellow());
                    }
                }
            }

            sleep(Duration::from_secs(interval)).await;
        }
    }

    /// Get detailed info about a port
    pub fn port_info(&self, port: u16) -> anyhow::Result<()> {
        println!("{}", format!("=== Port {} Information ===", port).bold());

        match self.find_processes_using_port(port) {
            Ok(processes) => {
                if processes.is_empty() {
                    println!("Status: {} (not in use)", "free".green());
                } else {
                    println!("Status: {} (in use)", "busy".red());
                    println!("\nProcesses:");
                    for (pid, name) in &processes {
                        if let Some(process) = self.system.process(Pid::from_u32(*pid as u32)) {
                            println!("\n  Process: {} (PID: {})", name.cyan(), pid);
                            println!("  Memory: {:.2} MB", process.memory() as f64 / 1024.0);
                            println!("  CPU Usage: {:.2}%", process.cpu_usage());
                            if let Some(cmd) = process.cmd().get(0) {
                                println!("  Command: {}", cmd.dimmed());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        Ok(())
    }

    fn find_processes_using_port(&self, port: u16) -> anyhow::Result<Vec<(u32, String)>> {
        let mut result = Vec::new();

        // Use ss command to find listening sockets with PIDs
        if let Ok(output) = std::process::Command::new("ss")
            .args(&["-tlnp"])
            .output() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                for line in stdout.lines() {
                    // Look for lines with our port number
                    let port_str = format!(":{}", port);
                    if line.contains(&port_str) && line.contains("LISTEN") && line.contains("pid=") {
                        // Extract PID from pid=XXXX
                        if let Some(pid_part) = line.split("pid=").nth(1) {
                            if let Some(pid_str) = pid_part.split(',').next() {
                                if let Ok(pid) = pid_str.parse::<u32>() {
                                    if let Some(process) = self.system.process(Pid::from_u32(pid)) {
                                        result.push((pid, process.name().to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
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
