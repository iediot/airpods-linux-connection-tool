<<<<<<< HEAD
# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.
=======
<h1 align="center">AirPods Linux Connection Tool</h1>
>>>>>>> ef91d5e (Update README.md)

## Recommended IDE Setup

<<<<<<< HEAD
[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
=======
<p align="center">
  <img src="https://img.shields.io/badge/Linux-grey?logo=linux&logoColor=white">
  <img src="https://img.shields.io/badge/Rust-orange?logo=rust&logoColor=white">
  <img src="https://img.shields.io/badge/Tauri-blue?logo=tauri&logoColor=white">
  <img src="https://img.shields.io/badge/Svelte-red?logo=svelte&logoColor=white">
</p>

---

### How it works

1. Runs silently in the background
2. Continuously scans for AirPods in **pairing mode** (hold the button on the back of the case)
3. When detected, a popup appears in the center of the screen
4. Click **Connect** to pair, trust, and connect — or **Ignore** to dismiss (30s cooldown)

### Requirements

- Linux with BlueZ (any modern distro)
- Bluetooth adapter
- `bluez` and `dbus` packages

### Build from source

```bash
git clone https://github.com/iediot/airpods-linux-connection-tool.git
cd airpods-linux-connection-tool
npm install
npm run tauri build
```

The binary will be at `src-tauri/target/release/airpods-linux-helper`.

### Install

```bash
# Copy binary
mkdir -p ~/.local/bin
cp src-tauri/target/release/airpods-linux-helper ~/.local/bin/

# Create systemd service
mkdir -p ~/.config/systemd/user
cat > ~/.config/systemd/user/airpods-helper.service << 'EOF'
[Unit]
Description=AirPods Connection Helper
After=bluetooth.target graphical-session.target
PartOf=graphical-session.target

[Service]
ExecStart=%h/.local/bin/airpods-linux-helper
Restart=on-failure
RestartSec=5
Environment=DISPLAY=:0

[Install]
WantedBy=graphical-session.target
EOF

# Enable and start
systemctl --user daemon-reload
systemctl --user enable airpods-helper.service
systemctl --user start airpods-helper.service
```

### Usage

| Action | What happens |
|---|---|
| **AirPods enter pairing mode** | Popup appears automatically |
| **Click Connect** | Pairs, trusts, and connects your AirPods |
| **Click Ignore** | Dismisses popup, 30s cooldown before next scan |

### Useful commands

```bash
# Check status
systemctl --user status airpods-helper.service

# View logs
journalctl --user -u airpods-helper.service -f

# Stop
systemctl --user stop airpods-helper.service

# Disable autostart
systemctl --user disable airpods-helper.service
```
>>>>>>> ef91d5e (Update README.md)
