mod port_monitor;
mod tui;

use clap::{Parser, Subcommand};
use port_monitor::PortMonitor;

#[derive(Parser)]
#[command(name = "PortMan")]
#[command(about = "Monitor and manage ports on your system", long_about = None)]
#[command(before_help = "
 ____   ___  ____  _____ __  __    _    _   _ 
|  _ \\ / _ \\|  _ \\|_   _|  \\/  |  / \\  | \\ | |
| |_) | | | | |_) | | | | |\\/| | / _ \\ |  \\| |
|  __/| |_| |  _ <  | | | |  | |/ ___ \\| |\\  |
|_|    \\___/|_| \\_\\ |_| |_|  |_/_/   \\_\\_| \\_|

Developer: zakkycrypt
")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch interactive TUI dashboard
    #[command(about = "Launch the interactive dashboard for monitoring ports")]
    Dashboard,

    /// List all processes using a specific port
    #[command(about = "Check which process is using a specific port")]
    Check {
        /// Port number to check
        #[arg(value_name = "PORT")]
        port: u16,
    },

    /// List all ports currently in use
    #[command(about = "Show all ports in use")]
    List,

    /// Kill a process using a specific port
    #[command(about = "Kill the process using a specific port")]
    Kill {
        /// Port number to free
        #[arg(value_name = "PORT")]
        port: u16,

        /// Don't ask for confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Monitor ports in real-time
    #[command(about = "Monitor specific ports continuously")]
    Monitor {
        /// Ports to monitor (comma-separated)
        #[arg(value_name = "PORTS")]
        ports: String,

        /// Refresh interval in seconds
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Get detailed info about a port
    #[command(about = "Show detailed information about a port")]
    Info {
        /// Port number
        #[arg(value_name = "PORT")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dashboard => {
            tui::run_tui()?;
        }
        Commands::Check { port } => {
            let monitor = PortMonitor::new();
            monitor.check_port(port)?;
        }
        Commands::List => {
            let monitor = PortMonitor::new();
            monitor.list_all_ports()?;
        }
        Commands::Kill { port, force } => {
            let monitor = PortMonitor::new();
            monitor.kill_port(port, force)?;
        }
        Commands::Monitor { ports, interval } => {
            let monitor = PortMonitor::new();
            monitor.monitor_ports(&ports, interval).await?;
        }
        Commands::Info { port } => {
            let monitor = PortMonitor::new();
            monitor.port_info(port)?;
        }
    }

    Ok(())
}
