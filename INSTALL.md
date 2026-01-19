# PortMan .deb Package Installation

## Installation

### Option 1: Using dpkg (Recommended)

```bash
cd deb-build
sudo dpkg -i portman_0.1.0.deb
```

### Option 2: Using apt

```bash
cd deb-build
sudo apt install ./portman_0.1.0.deb
```

## After Installation

Once installed, PortMan will be available system-wide:

### Launch the TUI Dashboard
```bash
portman dashboard
```

### Launch the Web GUI
```bash
portman-gui
```
Then open http://127.0.0.1:5173 in your browser.

### Use the CLI
```bash
# List all active ports
portman list

# Check a specific port
portman check 3000

# Kill a process on a port
portman kill 3000 --force

# Monitor multiple ports
portman monitor "3000,5432,8080"
```

## Uninstallation

```bash
sudo apt remove portman
```

or

```bash
sudo dpkg -r portman
```

## Package Contents

- `/usr/local/bin/portman` - CLI application with TUI dashboard
- `/usr/local/bin/portman-gui` - Web GUI server
- `/usr/share/applications/portman-gui.desktop` - Desktop shortcut for GUI

## Requirements

- Linux system (Ubuntu, Debian, Fedora, Arch, etc.)
- Read access to `/proc/net/tcp`
- Optional: `sudo` for killing processes

## Troubleshooting

### Permission Denied when killing processes

You may need to run with `sudo`:
```bash
sudo portman kill 3000
```

### Port 5173 already in use

If port 5173 is in use, you can modify the GUI to use a different port by editing the source.

## Support

For issues or feature requests, visit the GitHub repository.
