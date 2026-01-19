```
 ____   ___  ____  _____ __  __    _    _   _ 
|  _ \ / _ \|  _ \|_   _|  \/  |  / \  | \ | |
| |_) | | | | |_) | | | | |\/| | / _ \ |  \| |
|  __/| |_| |  _ <  | | | |  | |/ ___ \| |\  |
|_|    \___/|_| \_\ |_| |_|  |_/_/   \_\_| \_|
```

**Developer**: zakkycrypt

# PortMan - Port Monitor & Manager

A powerful Rust CLI tool to monitor and manage ports on your Linux system. Easily find which processes are using specific ports and kill them when needed.

## Features

- 🎨 **Interactive Dashboard** - Beautiful TUI interface for real-time monitoring
- 🔍 **Check Port Usage** - See which process is using a specific port
- 📊 **List All Ports** - View all ports currently in use
- ⚔️ **Kill Processes** - Terminate processes using a specific port
- 👁️ **Real-time Monitoring** - Monitor multiple ports continuously
- ℹ️ **Detailed Info** - Get detailed process information for a port

## Installation

### Prerequisites
- Rust 1.70 or later
- Linux system (tested on Ubuntu, Fedora, Arch)

### Build from Source

```bash
cd PortMan
cargo build --release
```

The compiled binary will be at `target/release/portman`.

### Add to PATH (Optional)

```bash
sudo cp target/release/portman /usr/local/bin/
```

## Usage

### Launch Interactive Dashboard (Recommended!)

```bash
portman dashboard
```

This opens a beautiful interactive TUI interface where you can:
- **↑/k** or **↓/j** - Navigate through ports
- **d** - Kill selected process
- **r** - Refresh port list
- **q** or **Esc** - Quit

### Check a Specific Port

```bash
portman check 3000
```

Output:
```
✓ Port 3000 is in use:
  next-dev (PID: 1234)
```

### List All Active Ports

```bash
portman list
```

Output:
```
=== Active Ports ===
  3000 → next-dev
  5432 → postgres
  8080 → node
```

### Kill Process Using a Port

```bash
# With confirmation
portman kill 3000

# Force kill without confirmation
portman kill 3000 --force
```

Output:
```
Process(es) using port 3000:
  next-dev (PID: 1234)

Kill these processes? y
✓ Killed next-dev (PID: 1234)

✓ Successfully killed 1 process(es)
```

### Monitor Ports in Real-time

```bash
# Monitor a single port
portman monitor 3000

# Monitor multiple ports
portman monitor "3000,5432,8080"

# Custom refresh interval (default: 2 seconds)
portman monitor 3000 --interval 1
```

### Get Detailed Port Information

```bash
portman info 3000
```

Output:
```
=== Port 3000 Information ===
Status: busy (in use)

Processes:
  Process: next-dev (PID: 1234)
  Memory: 125.45 MB
  CPU Usage: 2.34%
  Command: /usr/bin/node
```

## Common Examples

### Kill Next.js Dev Server

```bash
# Find and kill the Next.js dev server on port 3000
portman kill 3000 --force
```

### Find What's Using Port 5432 (PostgreSQL)

```bash
portman check 5432
```

### Monitor All Your Dev Servers

```bash
portman monitor "3000,5432,8080"
```

## Requirements

This tool requires read access to `/proc/net/tcp` and `/proc` filesystem, which is available on all Linux systems by default.

## Troubleshooting

### Permission Denied

If you get permission errors when killing processes, you may need to run with `sudo`:

```bash
sudo portman kill 3000
```

### Port Not Found

If a port shows as free but you believe a process is using it:
- The process might be listening on a different interface (check with `netstat -tlnp`)
- The port might be in TIME_WAIT state

## Building for Distribution

```bash
cargo build --release
```

This creates an optimized binary with maximum performance.

## License

MIT

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
