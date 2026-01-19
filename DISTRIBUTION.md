# PortMan - Complete Distribution Package

## 📦 Package Contents

PortMan is now available as:

### 1. **CLI Application** (`portman`)
- Full-featured command-line interface
- Interactive TUI dashboard
- 2.0 MB optimized binary

### 2. **Web GUI Application** (`portman-gui`)
- Modern web-based interface
- Real-time port monitoring
- Responsive design
- 1.7 MB optimized binary

### 3. **.deb Package** (`portman_0.1.0.deb`)
- Easy one-command installation
- 1.1 MB package size
- Desktop shortcut for GUI
- Automatic symlink creation

## 🚀 Quick Start

### Install from .deb
```bash
sudo apt install ./deb-build/portman_0.1.0.deb
```

### Or use from source
```bash
cargo build --release
./target/release/portman --help
./target/release/portman-gui
```

## 📋 Features

### ✨ Web GUI (`portman-gui`)
- 🎨 Beautiful cyberpunk-themed interface
- 🔄 Auto-refresh every 2 seconds
- 📊 Real-time memory & CPU monitoring
- ⚔️ One-click process termination
- 📱 Responsive layout (desktop/tablet)
- Access at: http://127.0.0.1:5173

### 🎮 TUI Dashboard (`portman dashboard`)
- 🎯 Keyboard navigation (↑/k, ↓/j)
- ⚡ Fast terminal rendering
- 🎨 Color-coded output
- 📊 Real-time updates every 250ms

### 💻 CLI Commands
```bash
portman list                 # List all ports
portman check 3000          # Check specific port
portman kill 3000 --force   # Kill process
portman monitor "3000,5432" # Monitor multiple
portman info 3000           # Detailed info
```

## 📁 Directory Structure

```
PortMan/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── gui_main.rs          # Web GUI server
│   ├── port_monitor.rs      # Core logic
│   ├── gui.rs               # GUI backend
│   └── tui.rs               # TUI dashboard
├── src-tauri/dist/          # Web GUI frontend
│   ├── index.html
│   ├── styles.css
│   └── app.js
├── deb-build/
│   └── portman_0.1.0.deb    # Ready-to-install package
├── target/release/
│   ├── portman              # CLI + TUI binary
│   └── portman-gui          # Web GUI binary
├── Cargo.toml
├── README.md
└── INSTALL.md
```

## 🛠️ Building

### Build CLI + TUI
```bash
cargo build --release --bin portman
```

### Build Web GUI Server
```bash
cargo build --release --bin portman-gui
```

### Create .deb Package
```bash
cd deb-build
dpkg-deb --build portman_0.1.0
```

## 📦 Installation Methods

### Method 1: .deb Package (Recommended for Ubuntu/Debian)
```bash
sudo apt install ./deb-build/portman_0.1.0.deb
portman dashboard
portman-gui
```

### Method 2: From Binaries
```bash
cargo build --release
sudo cp target/release/portman /usr/local/bin/
sudo cp target/release/portman-gui /usr/local/bin/
portman dashboard
```

### Method 3: Development Setup
```bash
cargo build
./target/debug/portman list
./target/debug/portman-gui
```

## 🖥️ System Requirements

- **OS**: Linux (Ubuntu, Debian, Fedora, Arch, CentOS, etc.)
- **Architecture**: x86_64 (amd64)
- **Runtime**: No additional runtime needed
- **Permissions**: Read access to `/proc/net/tcp` (required)
- **Optional**: `sudo` for killing processes

## 🔧 Configuration

### Web GUI Port
Default port is 5173. To use a different port, modify `src/gui_main.rs`:
```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:YOUR_PORT").await?;
```

### Auto-refresh Interval
GUI refreshes every 2 seconds by default. Adjust in `src-tauri/dist/app.js`:
```javascript
refreshInterval = setInterval(refreshPorts, 2000); // Change 2000 to desired ms
```

## 🐛 Troubleshooting

### Port 5173 already in use
```bash
# Find and kill the process
portman kill 5173
# Or specify different port in source
```

### Permission Denied
```bash
# Use sudo for killing processes
sudo portman kill 3000
```

### Binary not found after install
```bash
# Create symlink manually
sudo ln -s /usr/local/bin/portman /usr/bin/portman
sudo ln -s /usr/local/bin/portman-gui /usr/bin/portman-gui
```

## 📝 File Sizes

- CLI Binary (`portman`): 2.0 MB
- GUI Binary (`portman-gui`): 1.7 MB
- .deb Package: 1.1 MB (compressed)
- Installed Size: ~8 MB

## 🎓 Usage Examples

### Find what's using port 3000
```bash
portman check 3000
```

### Kill Next.js dev server
```bash
portman kill 3000 --force
```

### Monitor multiple ports
```bash
portman monitor "3000,5432,8080"
```

### Open web dashboard
```bash
portman-gui
# Then open http://127.0.0.1:5173 in browser
```

### Launch TUI dashboard
```bash
portman dashboard
```

## 📄 License

MIT License - See LICENSE file

## 👤 Developer

**ZAKKYCRYPT**

## 🚀 Features At a Glance

| Feature | CLI | TUI | Web GUI |
|---------|-----|-----|--------|
| List Ports | ✓ | ✓ | ✓ |
| Check Port | ✓ | - | ✓ |
| Kill Process | ✓ | ✓ | ✓ |
| Monitor Ports | ✓ | - | ✓ |
| Port Details | ✓ | - | ✓ |
| Real-time Updates | - | ✓ | ✓ |
| Memory/CPU Info | ✓ | - | ✓ |
| GUI Interface | - | ✓ | ✓ |

## 🎯 Next Steps

1. **Install**: Use the .deb package or build from source
2. **Explore**: Try `portman dashboard` or `portman-gui`
3. **Automate**: Create scripts using the CLI
4. **Monitor**: Keep tabs on your dev servers

---

**Enjoy using PortMan!** 🎉
