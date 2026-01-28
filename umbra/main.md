---

# PortaQEMU Specification

## Versioned Product & Architecture Contract

---

## Purpose

**PortaQEMU** is a **portable, no-admin** tool that installs and runs a **QEMU-based Windows VM** from a user-writable directory, exposes **SSH for VS Code Remote-SSH**, and integrates with **Windows Terminal** via a reversible profile fragment.

The project prioritizes:

* portability
* determinism
* minimal coupling to the host system
* future extensibility (tray UI, host↔guest integration)

---

## Target User

Developers on Windows **without administrator privileges** who want a reproducible, terminal-first VM workflow.

---

## Non-Negotiable Constraints

* No admin privileges required at install or runtime
* All state stored in user-writable locations
* No modification of global Windows Terminal settings
* Must work without WHPX (fallback to TCG)
* CLI is the source of truth (UI is optional / layered)

---

# Version Roadmap

---

# v0.1 — MVP (Foundational Runtime)

> **Goal:** A reliable, portable VM runtime with SSH + Terminal integration.

---

## Features (v0.1)

### F1. Portable Install Layout

* Default root:

  ```
  %LOCALAPPDATA%\PortaQEMU\
  ```
* Overrideable via `--root` or environment variable

```
PortaQEMU/
  bin/        # portaqemu.exe, qemu runtime
  vm/         # disk images
  config/     # toml, ssh, state
  logs/       # launcher + qemu logs
  terminal/   # generated artifacts
```

---

### F2. Config-Driven VM Launch

* Single config file:

  ```
  config/portaqemu.toml
  ```
* No hardcoded paths or ports
* QEMU argv is derived exclusively from config

---

### F3. Path Variable Resolution (MVP-critical)

All config paths **MUST support variables**.

#### Supported variables (v0.1)

| Variable | Meaning                  |
| -------- | ------------------------ |
| `%ROOT%` | PortaQEMU root directory |

#### Rules

* Variables are resolved **before validation**
* Unresolved variables → config error
* Resolved paths are normalized to absolute paths
* Paths may point **outside** the root directory

#### Example

```toml
disk = "%ROOT%/vm/disk.qcow2"
identity_file = "%ROOT%/config/ssh/id_ed25519"
```

> This is a **contract**. Future versions may add more variables, but `%ROOT%` is mandatory.

---

### F4. VM Lifecycle Management (CLI)

Supported commands:

* `up`
* `down`
* `status`

State tracking:

* PID + metadata stored in:

  ```
  config/state.json
  ```

Logs:

* `logs/launcher.log`
* `logs/qemu.log`

---

### F5. Networking (MVP)

* QEMU user-mode networking
* Mandatory SSH forward:

  ```
  localhost:<ssh_host_port> → guest:22
  ```
* Optional additional forwards
* Pre-launch port conflict detection

---

### F6. SSH + VS Code Remote-SSH Support

* SSH identity stored under:

  ```
  config/ssh/
  ```
* Commands:

  * print SSH command
  * execute SSH
  * print VS Code SSH config block
  * optionally install/remove into **default** SSH config only

> Non-standard SSH locations are **out of scope** for MVP.

---

### F7. Windows Terminal Integration (Fragment-Based)

* Install/remove Terminal **profile fragment**
* No editing of `settings.json`
* Stable GUID per VM
* Modes:

  * `ssh`
  * `up_attach`

---

### F8. Autostart Control (CLI-only)

#### Commands

```bash
portaqemu enable
portaqemu disable
portaqemu autostart status
```

#### Behavior

* Uses **Startup folder shortcut** (HKCU, no registry writes)
* Enables:

  ```
  portaqemu up --no-wait
  ```
* Disable removes only the shortcut it owns

> Tray UI is **explicitly not part of v0.1**.

---

### F9. Diagnostics (`doctor`)

Checks:

* QEMU binary presence
* Disk image presence
* Acceleration availability (WHPX / fallback)
* Port conflicts
* SSH key presence
* Terminal fragment location/status
* Autostart status

Outputs:

* Human checklist
* JSON diagnostics for tooling

---

## v0.1 Configuration Contract

### `config/portaqemu.toml`

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

---

## v0.1 CLI Commands (Summary)

```
portaqemu init
portaqemu up [--attach|--no-wait]
portaqemu down
portaqemu status
portaqemu ssh [--exec]
portaqemu terminal install|remove|path
portaqemu vscode print|install|remove
portaqemu enable|disable|autostart status
portaqemu doctor
```

---

## v0.1 Explicitly Out of Scope

* Guest agent
* Host↔guest RPC
* Tray UI
* File sharing / clipboard
* VM image creation
* Multiple SSH config locations
* `%USERPROFILE%` / `%ENV%` variables

---

# v0.2 — Usability & Environment Awareness

> **Goal:** Reduce friction in real-world setups.

Planned additions:

### v0.2 Features

* Support **multiple SSH config locations**

  * configurable path
  * VS Code portable setups
* Additional path variables:

  * `%USERPROFILE%`
  * `%CONFIG%`
* Tray application (read-only status + start/stop)
* Autostart via registry (optional alternative)
* SSH readiness via handshake (not just TCP)
* Better error classification in `doctor`

---

# v0.3 — Host ↔ Guest Integration Layer

> **Goal:** Make the VM feel “native”.

Planned additions:

* Dedicated integration channel (vsock or TCP)
* Action registry (host + guest)
* Event-based messaging
* Clipboard sync
* Open file / open VS Code from guest
* Structured JSON protocol
* Plugin-style extension points

---

# Design Principles (Non-Versioned)

* CLI is authoritative
* All mutations are reversible
* Config is declarative
* State is explicit and inspectable
* No silent magic

---

## End of Specification

---