# PortaQEMU — Rust crate/module layout for `portaqemu.exe` (v0.1)

This is a **build-ready** Rust project layout and module contract for the CLI binary **`portaqemu.exe`**.
It is designed to support the v0.1 spec (portable QEMU VM + Terminal fragment + SSH + autostart + doctor), while keeping v0.2/v0.3 extensible.

---

## 0) Workspace / repo layout

Recommended repo layout (single binary crate, plus optional internal libs later):

```

portaqemu/
Cargo.toml
Cargo.lock
README.md
SPEC.md
src/
main.rs
lib.rs
cli/
mod.rs
commands/
mod.rs
init.rs
up.rs
down.rs
status.rs
ssh.rs
terminal.rs
vscode.rs
autostart.rs
doctor.rs
config/
mod.rs
schema.rs
load.rs
vars.rs
validate.rs
paths/
mod.rs
root.rs
terminal_fragments.rs
vscode.rs
ssh.rs
qemu/
mod.rs
locate.rs
accel.rs
argv.rs
spawn.rs
probe.rs
state/
mod.rs
model.rs
io.rs
lock.rs
terminal/
mod.rs
fragment.rs
guid.rs
vscode/
mod.rs
ssh_config.rs
autostart/
mod.rs
startup_folder.rs
shortcut.rs
doctor/
mod.rs
checks.rs
report.rs
output/
mod.rs
json.rs
human.rs
util/
mod.rs
fs_atomic.rs
net.rs
process.rs
time.rs
hashing.rs
tests/
config_vars.rs
terminal_fragment.rs
qemu_argv.rs

```

**Why this shape:** commands are thin; shared logic lives in `config/`, `qemu/`, `terminal/`, `state/`, etc. That keeps “feature additions” from turning into a giant `main.rs`.

---

## 1) Crates to use (v0.1)

Suggested dependencies:

- CLI / parsing: `clap` (derive)
- Serialization: `serde`, `serde_json`, `toml`
- Errors: `thiserror` (typed errors) + `anyhow` (top-level context) OR just `thiserror`
- Logging: `tracing`, `tracing-subscriber`
- GUIDs: `uuid` (for v5 UUID)
- Hashing: `sha2` (for qemu args hash)
- Windows integration:
  - `windows` crate (COM shortcuts) **or** a small helper crate for `.lnk` creation
  - if you want simplest v0.1: create a `.cmd` file in Startup folder (no COM), see `autostart/`

You can keep windows-only code behind `cfg(windows)`.

---

## 2) `main.rs` and `lib.rs` responsibilities

### `src/main.rs`
- Parses CLI args (clap)
- Initializes logging + output mode
- Calls `cli::run(args)` and maps errors → exit codes

### `src/lib.rs`
- Exposes `cli::run` + core structs for integration testing

---

## 3) CLI layer (`src/cli/`)

### `cli/mod.rs`
- Defines `Cli` struct (clap)
- Defines `OutputMode` (human/json)
- `run(cli: Cli) -> Result<ExitOutcome, AppError>`

### `cli/commands/*`
Each file implements a single command handler:

- `init.rs`: `handle_init(ctx, args)`
- `up.rs`: `handle_up(ctx, args)`
- `terminal.rs`: `handle_terminal(ctx, subcmd)`
- etc.

**Pattern:**
- command handler is “application glue”
- it calls reusable services from other modules

---

## 4) Core context object

Create a central `AppContext` so every command gets consistent access to paths/config/output/logging.

Example:

```rust
pub struct AppContext {
  pub root: PathBuf,
  pub config_path: PathBuf,
  pub output: OutputMode,
  pub now: DateTime<Utc>,
}
```

This is built once in `cli::run`.

---

## 5) Config system (`src/config/`)

### `schema.rs`

Typed config structs mirroring TOML:

```rust
#[derive(Deserialize)]
pub struct Config {
  pub vm: VmConfig,
  pub network: NetworkConfig,
  pub accel: AccelConfig,
  pub terminal: TerminalConfig,
  pub vscode: VscodeConfig,
}
```

### `vars.rs` (path variables)

* `resolve_vars(input: &str, root: &Path) -> Result<String, ConfigError>`
* v0.1 supports only `%ROOT%`
* add “reject unresolved markers” logic here

### `load.rs`

* reads TOML
* resolves variables on path fields
* returns `ResolvedConfig` where paths are `PathBuf` and absolute

### `validate.rs`

* config validation rules
* errors with actionable messages

---

## 6) QEMU subsystem (`src/qemu/`)

### `locate.rs`

* find `qemu-system-x86_64.exe` in:

  1. (future) config override
  2. `<root>/bin/qemu/...`
  3. `<root>/bin/...`
  4. PATH

### `accel.rs`

* detection/probing for WHPX
* implements `detect_available_accels(qemu_path) -> AccelAvailability`
* implements `choose_accel(preferred, availability) -> AccelPlan`

### `argv.rs` (your spec)

* `build_argv(cfg: &ResolvedConfig, accel: AccelChoice) -> Vec<OsString>`
* includes:

  * q35
  * `-accel whpx|tcg`
  * `-cpu host|qemu64`
  * `-m`, `-smp`
  * disk attachment
  * `-netdev user,...hostfwd=...`
  * devices (usb-tablet, virtio-net)

### `spawn.rs`

* spawns QEMU
* redirects stdout/stderr to `logs/qemu.log`
* returns a `RunningVm { pid, child_handle? }`

### `probe.rs`

* quick failure detection for accel fallback:

  * if accel=whpx fails fast with matching stderr, retry tcg (auto mode)

---

## 7) State system (`src/state/`)

### `model.rs`

Defines the persisted state:

```rust
#[derive(Serialize, Deserialize)]
pub struct VmState {
  pub running: bool,
  pub qemu_pid: Option<u32>,
  pub started_at: Option<String>,
  pub qemu_args_hash: Option<String>,
  pub last_error: Option<String>,
}
```

### `io.rs`

* `load_state(path) -> VmState`
* `save_state_atomic(path, &VmState)`

### `lock.rs`

Single-instance guard to prevent concurrent `up`/`down`:

* file lock in `config/` (simple lockfile strategy v0.1)

---

## 8) Terminal integration (`src/terminal/`)

### `guid.rs`

* stable UUIDv5 generation:

  * namespace constant (PortaQEMU)
  * name = `vm.name`

### `fragment.rs`

* builds fragment JSON (serde_json)
* writes atomically to the fragment directory
* remove deletes only our file

Terminal path logic lives in `paths/terminal_fragments.rs`.

---

## 9) VS Code / SSH config (`src/vscode/`)

### `ssh_config.rs`

* `print_block(cfg) -> String`
* `install_block(default_ssh_config_path, block)` with BEGIN/END markers
* `remove_block(...)`

**v0.1 limitation:** only default `~/.ssh/config` is supported.
(v0.2 expands discovery/overrides.)

---

## 10) Autostart (`src/autostart/`)

### `startup_folder.rs`

* computes per-user Startup folder path (Windows-known folder)
* v0.1 recommended approach:

**Simplest reliable option for v0.1:**
Write a small `.cmd` into Startup folder:

* `PortaQEMU (devvm).cmd`
* contains:

  ```
  "%ROOT%\bin\portaqemu.exe" up --no-wait
  ```

This avoids COM shortcut complexity in MVP.

### `shortcut.rs` (optional)

Later replace `.cmd` with `.lnk` using `windows` crate.

---

## 11) Doctor (`src/doctor/`)

### `checks.rs`

Individual check functions returning structured results:

```rust
pub struct CheckResult {
  pub id: &'static str,
  pub status: CheckStatus, // Pass/Warn/Fail
  pub message: String,
  pub hint: Option<String>,
}
```

### `report.rs`

* formats results for human and JSON
* `doctor` command collects + prints

---

## 12) Output formatting (`src/output/`)

Single interface so all commands can do:

```rust
ctx.emit(Outcome::Status { ... })
```

* `json.rs` -> structured JSON
* `human.rs` -> friendly console output

This prevents every command from re-implementing `--json`.

---

## 13) Utilities (`src/util/`)

* `fs_atomic.rs`: write temp + rename
* `net.rs`: check port availability; wait for TCP readiness
* `process.rs`: process liveness check, kill tree (best-effort)
* `hashing.rs`: sha256 of argv for `qemu_args_hash`

---

## 14) Tests (important, cheap wins)

Add unit/integration tests for the riskiest contracts:

* `tests/config_vars.rs`

  * `%ROOT%` resolution
  * unresolved marker errors

* `tests/terminal_fragment.rs`

  * stable GUID from vm name
  * fragment JSON matches schema

* `tests/qemu_argv.rs`

  * argv contains correct `hostfwd` rules
  * accel switching changes cpu model
  * disk format selection (.qcow2 vs .raw)

---

## 15) Minimal “happy path” execution flow (v0.1)

### `portaqemu up` does:

1. load config → resolve vars → validate
2. locate qemu
3. select accel:

   * preferred=auto: probe + fallback
4. build argv
5. spawn qemu + log redirection
6. persist state (pid + args hash)
7. if waiting: poll `localhost:ssh_host_port` readiness

---

[1]: https://cursor.com/docs/agent/modes?utm_source=chatgpt.com "Modes | Cursor Docs"
[2]: https://openai.com/index/introducing-codex/?utm_source=chatgpt.com "Introducing Codex"