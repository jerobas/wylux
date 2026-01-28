# PortaQEMU

Portable QEMU VM manager for Windows - A no-admin tool for running QEMU-based VMs with SSH and Windows Terminal integration.

## Features

### v0.1 MVP
- ✅ Portable install layout (no admin required)
- ✅ Config-driven VM launch
- ✅ Path variable resolution (%ROOT%)
- ✅ VM lifecycle management (up/down/status)
- ✅ QEMU networking with SSH forwarding
- ✅ SSH + VS Code Remote-SSH support
- ✅ Windows Terminal fragment integration
- ✅ Autostart control
- ✅ Diagnostics (doctor command)

### v0.3 Integration Layer
- ✅ Protocol layer (JSON over vsock/TCP)
- ✅ Function registry pattern
- ✅ Connection manager
- ✅ Event bus for pub/sub messaging

## Installation

```bash
cargo build --release
```

## Usage

### Initialize

```bash
portaqemu init
```

### Start VM

```bash
portaqemu up
```

### Stop VM

```bash
portaqemu down
```

### Status

```bash
portaqemu status
```

### Terminal Integration

```bash
portaqemu terminal install
portaqemu terminal remove
portaqemu terminal path
```

### VS Code SSH Config

```bash
portaqemu vscode print
portaqemu vscode install
portaqemu vscode remove
```

### Autostart

```bash
portaqemu enable
portaqemu disable
portaqemu autostart status
```

### Diagnostics

```bash
portaqemu doctor
```

## Configuration

Configuration is stored in `config/portaqemu.toml`:

```toml
[vm]
name = "devvm"
disk = "%ROOT%/vm/disk.qcow2"
memory_mb = 4096
cpus = 4

[network]
ssh_host_port = 2222
forwards = []

[accel]
preferred = "auto"   # auto | whpx | tcg

[terminal]
profile_name = "PortaQEMU Dev VM"
icon = "%ROOT%/bin/icon.ico"
mode = "ssh"         # ssh | up_attach

[vscode]
ssh_user = "dev"
identity_file = "%ROOT%/config/ssh/id_ed25519"
```

## Architecture

The project follows a modular architecture:

- **CLI Layer**: Command parsing and routing
- **Config System**: TOML loading, variable resolution, validation
- **QEMU Subsystem**: Binary location, acceleration detection, argv building, spawning
- **State Management**: VM state persistence, locking
- **Terminal Integration**: Windows Terminal fragment generation
- **VS Code Integration**: SSH config management
- **Autostart**: Startup folder management
- **Doctor**: Diagnostics and health checks
- **Integration Layer**: Protocol, registry, connection manager, event bus (v0.3)

## License

MIT OR Apache-2.0
