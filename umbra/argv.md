# PortaQEMU — QEMU argv Builder Spec (Windows Guest, SSH Forwarding, Accel Detection) — v0.1

This document defines how `portaqemu up` must build the **QEMU command line** deterministically from `config/portaqemu.toml` for a **Windows guest**, including:

- argument composition rules
- networking with `hostfwd` for SSH and extra forwards
- acceleration selection (WHPX detection + TCG fallback)
- minimal device model for a usable Windows VM

This is the authoritative contract for v0.1.

---

## 1) Inputs

### 1.1 Config keys used by argv builder (v0.1)

From `portaqemu.toml`:

```toml
[vm]
name = "devvm"
disk = "%ROOT%/vm/disk.qcow2"
memory_mb = 4096
cpus = 4

[network]
ssh_host_port = 2222
forwards = [
  # { host = 8080, guest = 8080 }
]

[accel]
preferred = "auto"   # auto | whpx | tcg
```

### 1.2 Resolved paths

All paths must be variable-resolved before argv generation (see path variable spec). `vm.disk` must be an **absolute path** by the time argv is built.

---

## 2) Output

### 2.1 QEMU executable

`portaqemu` must locate `qemu-system-x86_64.exe` by (in order):

1. `config.qemu.path` (optional future key; not required in v0.1)
2. `<ROOT>\bin\qemu\qemu-system-x86_64.exe` (preferred portable layout)
3. `<ROOT>\bin\qemu-system-x86_64.exe`
4. PATH lookup (last resort)

If not found → exit code `3` (dependency missing).

---

## 3) Base VM Model (Windows Guest)

These are the default arguments for a usable Windows desktop VM.

### 3.1 Machine type

Use Q35:

* `-machine q35,<accel_fragment>`

Rationale: modern chipset, broadly compatible.

### 3.2 CPU model

* If accel is WHPX: `-cpu host`
* If accel is TCG: `-cpu qemu64`

### 3.3 Memory and CPUs

* `-m <vm.memory_mb>`
* `-smp <vm.cpus>`

### 3.4 RTC for Windows

Use localtime (Windows expects this commonly):

* `-rtc base=localtime`

### 3.5 Input usability

To avoid “mouse capture weirdness”:

* `-device qemu-xhci`
* `-device usb-tablet`

### 3.6 Display defaults

v0.1: rely on QEMU default UI output by not specifying display unless needed.

Optional (if you want determinism):

* `-display sdl`  (works well on Windows with bundled SDL)
* OR `-display gtk` (only if you ship GTK runtime)

**v0.1 contract:** display is not required to be configurable; it only must be usable.

---

## 4) Disk Attachment

### 4.1 Disk format

* If disk ends in `.qcow2` → `format=qcow2`
* If ends in `.raw` or `.img` → `format=raw`
* Otherwise:

  * either omit format (QEMU auto-detect)
  * OR implement a light `qemu-img info` probe (v0.2+)

### 4.2 Drive arguments

Attach as VirtIO for performance:

```
-drive file=<ABS_PATH>,if=virtio,format=<fmt>
```

**MVP notes:**

* Do not force `cache=none` on Windows host in v0.1 (can complicate permissions/perf).
* Do not require VirtIO drivers to be installed; that’s user image responsibility.

---

## 5) Networking & SSH Forwarding (MVP)

### 5.1 Network backend

Use user-mode networking with forwards:

* `-netdev user,id=n0,<hostfwd_rules...>`
* `-device virtio-net-pci,netdev=n0`

### 5.2 SSH forward (required)

Always include:

```
hostfwd=tcp:127.0.0.1:<ssh_host_port>-:22
```

Notes:

* Bind to `127.0.0.1` explicitly to avoid exposing ports on LAN.
* Validate `<ssh_host_port>` availability pre-launch (exit code `4` if in use).

### 5.3 Extra forwards (optional)

For each entry `{ host = H, guest = G }` in `network.forwards`, append:

```
hostfwd=tcp:127.0.0.1:H-:G
```

Validation rules:

* `host` must be 1–65535
* `guest` must be 1–65535
* host ports must be unique within the config
* each host port must be free on launch

### 5.4 Forward protocol constraints (v0.1)

* Only TCP forwards are supported in v0.1.
* UDP forwards are v0.2+.

---

## 6) Acceleration Detection & Fallback

### 6.1 Config modes

`accel.preferred` determines behavior:

* `whpx`: require WHPX (fail if not available)
* `tcg`: always use TCG
* `auto`: attempt WHPX, fallback to TCG if unavailable

### 6.2 How to detect whether WHPX is supported (fast-path)

`portaqemu` SHOULD implement a **capability probe**:

Run:

```
qemu-system-x86_64.exe -accel help
```

Parse stdout/stderr for available accelerators:

* If `whpx` appears → WHPX is likely supported.
* If not → assume no WHPX.

This probe is advisory; runtime may still fail, so fallback logic still applies.

### 6.3 Runtime fallback (authoritative)

Even if probe suggests WHPX, QEMU startup can still fail (feature disabled, policy, etc).

**v0.1 contract for `auto`:**

1. Try launch with WHPX
2. If QEMU exits quickly with an error matching “accel unavailable” patterns, retry once with TCG.

Recommended detection heuristics:

* QEMU process exits with non-zero code within a short grace window (e.g., 3 seconds)
* stderr contains any of these substrings (case-insensitive):

  * `whpx`
  * `WHPX:`
  * `failed to initialize whpx`
  * `acceleration` + `not available`
  * `no accelerator found`

If it matches → retry with TCG.

If it fails again → propagate failure (exit code `1` or `3` depending on cause).

### 6.4 Accel fragments

* WHPX:

  * `-accel whpx`
  * `-machine q35,accel=whpx`
* TCG:

  * `-accel tcg`
  * `-machine q35,accel=tcg`

**v0.1 recommendation:** choose one style consistently.
Preferred: use `-accel <type>` and keep `-machine q35` clean.

Example:

* `-machine q35 -accel whpx`
* `-machine q35 -accel tcg`

---

## 7) Process & State Integration

### 7.1 PID tracking

`portaqemu up` must store the spawned QEMU PID in `config/state.json`.

### 7.2 Stdout/stderr logging

All QEMU stdout/stderr must be redirected to:

* `<ROOT>\logs\qemu.log`

### 7.3 Avoid `-daemonize` in v0.1

v0.1 should not rely on `-daemonize` because:

* PID management becomes more complex
* debugging becomes harder

Instead:

* spawn QEMU normally
* keep a handle for graceful shutdown
* use process termination on `down`

---

## 8) The Canonical argv Layout (v0.1)

The argv builder must produce arguments in the following conceptual order:

1. Identity & resources
2. Accel & CPU
3. Devices (USB, net)
4. Disk
5. Networking rules
6. Optional extras

### 8.1 Example: AUTO accel, qcow2, ssh+1 forward

```text
qemu-system-x86_64.exe
  -name devvm
  -machine q35
  -accel whpx                (or tcg fallback)
  -cpu host                  (or qemu64 for tcg)
  -m 4096
  -smp 4
  -rtc base=localtime
  -device qemu-xhci
  -device usb-tablet
  -drive file=C:\...\disk.qcow2,if=virtio,format=qcow2
  -netdev user,id=n0,hostfwd=tcp:127.0.0.1:2222-:22,hostfwd=tcp:127.0.0.1:8080-:8080
  -device virtio-net-pci,netdev=n0
```

---

## 9) Validation & Error Mapping

### 9.1 Pre-launch validation

Before spawning QEMU, `portaqemu` must validate:

* disk exists and is readable
* requested host ports are free
* qemu exe exists and is runnable

Error → exit codes:

* config invalid → `2`
* missing disk/qemu → `3`
* port busy → `4`

### 9.2 Post-launch validation (readiness)

For `up` without `--no-wait`:

* Wait for TCP connect to `127.0.0.1:ssh_host_port`
* If timeout → return non-zero (generic error `1`) and include actionable log pointer

---

## 10) v0.2+ Extensions (Not in v0.1)

These are explicitly not required for MVP argv builder but are likely next:

* `-qmp` socket for structured control instead of PID-kill
* TAP/bridge networking (admin often required; keep optional)
* configurable display backend
* virtio-balloon / virtio-serial devices
* UDP forwarding
* probing disk format via `qemu-img info`

---

## Appendix A — Minimal Implementation Checklist

* [ ] Resolve `%ROOT%` variables → absolute paths
* [ ] Build forwards list with `127.0.0.1` binding
* [ ] Implement `-accel help` probe (optional but recommended)
* [ ] Implement runtime fallback: WHPX → TCG on accel failure
* [ ] Spawn QEMU with redirected logs
* [ ] Persist PID + metadata to state file
* [ ] Wait for SSH port readiness (unless `--no-wait`)
* [ ] Ensure `down` kills the recorded PID safely

---

```
::contentReference[oaicite:0]{index=0}
```