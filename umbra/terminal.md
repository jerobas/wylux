# PortaQEMU — Windows Terminal Fragment Spec (v0.1)

This document defines **how PortaQEMU installs a Windows Terminal profile** safely and reversibly using **JSON Fragment Extensions**, including:

- where fragments must be written
- how the fragment JSON must be shaped
- how PortaQEMU discovers the correct location
- how install/remove works without touching `settings.json`

This spec is part of **v0.1 (MVP)**.

---

## 1. Background

Windows Terminal supports **JSON Fragment Extensions**: small JSON files dropped into a specific directory. Terminal loads them automatically and merges them into its settings. Microsoft documents the fragment location and structure here. :contentReference[oaicite:0]{index=0}

PortaQEMU MUST:
- never modify user `settings.json`
- be reversible by deleting only the files it created
- work regardless of Terminal distribution (Store / unpackaged)

---

## 2. Fragment Location Contract

### 2.1 Primary Fragment Root (per-user, recommended)

PortaQEMU MUST write fragments to:

```

%LOCALAPPDATA%\Microsoft\Windows Terminal\Fragments{app-name}{file-name}.json

```

This is the documented fragment extension directory. :contentReference[oaicite:1]{index=1}

### 2.2 Directory creation

If any part of the fragment directory path does not exist, PortaQEMU MUST create it. :contentReference[oaicite:2]{index=2}

### 2.3 Naming

- `{app-name}` MUST be stable and unique to this project:
  - `PortaQEMU` (exact casing recommended)
- `{file-name}.json` SHOULD include the VM name to avoid collisions:
  - `devvm.json`
  - `workvm.json`

Example:
```
%LOCALAPPDATA%\Microsoft\Windows Terminal\Fragments\PortaQEMU\devvm.json
```

---

## 3. Fragment JSON Structure

### 3.1 Top-level schema

A fragment file is a JSON document containing **snippets** that Terminal merges.

For PortaQEMU v0.1, the fragment MUST define a profile under:
- `profiles.list` (preferred)
- and MAY also include color schemes later (v0.2+)

**Required:**
- `profiles`
  - `list`: array of profiles

### 3.2 Profile object requirements (PortaQEMU contract)

Each PortaQEMU-generated profile MUST include:

| Field | Required | Notes |
|------|----------|------|
| `guid` | ✅ | MUST be stable across installs for same VM |
| `name` | ✅ | Shown in Terminal UI |
| `commandline` | ✅ | What Terminal runs |
| `startingDirectory` | ❌ | Optional (e.g. `%USERPROFILE%`) |
| `icon` | ❌ | Optional (path to `.ico` or `.png`) |

### 3.3 `guid` stability requirements

PortaQEMU MUST use a stable GUID per VM. Recommended method:

- Compute GUID as UUIDv5 of:
  - namespace = fixed PortaQEMU namespace GUID
  - name = `vm.name` (from config)
- This ensures:
  - re-install does not create duplicates
  - `remove` can reliably identify “our” profile

### 3.4 `source` field (important)

The Terminal uses the `"source"` field to indicate origin. When profiles come from a fragment, they may appear as a “fragment extension” source in UI/diagnostics. :contentReference[oaicite:3]{index=3}

PortaQEMU SHOULD include:

```json
"source": "PortaQEMU"
````

This is not strictly required by Terminal, but is valuable for identification and debugging.

---

## 4. Commandline Modes (v0.1)

PortaQEMU supports two terminal modes controlled by config:

### 4.1 Mode A: `terminal.mode = "ssh"` (recommended for MVP)

Terminal profile runs SSH directly:

* commandline:

  * uses `ssh.exe` available on modern Windows
  * connects to `localhost:<ssh_host_port>`

Example:

```json
"commandline": "ssh -p 2222 dev@localhost"
```

### 4.2 Mode B: `terminal.mode = "up_attach"`

Terminal profile starts VM (if needed) and attaches:

Example:

```json
"commandline": "\"%ROOT%\\\\bin\\\\portaqemu.exe\" up --attach"
```

Notes:

* `%ROOT%` is resolved by PortaQEMU when generating the fragment (do not leave `%ROOT%` in JSON)
* Paths in JSON must be Windows-safe (escape backslashes or quote carefully)

---

## 5. Fragment File Templates

### 5.1 Template — SSH mode

```json
{
  "profiles": {
    "list": [
      {
        "guid": "{PUT-STABLE-GUID-HERE}",
        "name": "PortaQEMU: devvm",
        "commandline": "ssh -p 2222 dev@localhost",
        "icon": "C:\\\\Users\\\\<user>\\\\AppData\\\\Local\\\\PortaQEMU\\\\bin\\\\icon.ico",
        "startingDirectory": "%USERPROFILE%",
        "source": "PortaQEMU"
      }
    ]
  }
}
```

### 5.2 Template — up_attach mode

```json
{
  "profiles": {
    "list": [
      {
        "guid": "{PUT-STABLE-GUID-HERE}",
        "name": "PortaQEMU: devvm",
        "commandline": "\"C:\\\\Users\\\\<user>\\\\AppData\\\\Local\\\\PortaQEMU\\\\bin\\\\portaqemu.exe\" up --attach",
        "icon": "C:\\\\Users\\\\<user>\\\\AppData\\\\Local\\\\PortaQEMU\\\\bin\\\\icon.ico",
        "startingDirectory": "%USERPROFILE%",
        "source": "PortaQEMU"
      }
    ]
  }
}
```

---

## 6. Discovery Rules (v0.1)

PortaQEMU needs to determine where to write fragment files.

### 6.1 Root discovery algorithm (MUST)

1. Read `%LOCALAPPDATA%` (Windows environment variable)
2. Compose:

   ```
   <LOCALAPPDATA>\Microsoft\Windows Terminal\Fragments
   ```
3. Use that as the fragment root directory (create if missing). ([Microsoft Learn][1])

### 6.2 Terminal installation detection (MVP stance)

PortaQEMU v0.1 does **not** need to detect which Terminal distribution is installed, because fragments are loaded from the fragment root regardless of settings.json path. ([Microsoft Learn][1])

However, `doctor` SHOULD warn if Windows Terminal is not installed / not launchable (best-effort).

---

## 7. CLI Subcommands Contract

### 7.1 `portaqemu terminal path`

Print the computed fragment root and the exact file path it will use for this VM.

Output example:

* Human:

  ```
  Fragment root: C:\Users\me\AppData\Local\Microsoft\Windows Terminal\Fragments
  Fragment file: C:\Users\me\AppData\Local\Microsoft\Windows Terminal\Fragments\PortaQEMU\devvm.json
  ```
* JSON (`--json`):

  ```json
  {
    "fragment_root": "C:\\Users\\me\\AppData\\Local\\Microsoft\\Windows Terminal\\Fragments",
    "fragment_file": "C:\\Users\\me\\AppData\\Local\\Microsoft\\Windows Terminal\\Fragments\\PortaQEMU\\devvm.json"
  }
  ```

### 7.2 `portaqemu terminal install`

MUST:

* ensure fragment directory exists
* generate stable GUID
* render fragment JSON (ssh/up_attach based on config)
* write atomically:

  * write to temp file
  * rename/replace target file

Idempotency:

* running install twice must produce same file contents (aside from whitespace formatting)

### 7.3 `portaqemu terminal remove`

MUST:

* delete only the specific fragment file it owns:

  * `%LOCALAPPDATA%\Microsoft\Windows Terminal\Fragments\PortaQEMU\<vm.name>.json`
* if missing, return success with “already removed” warning

---

## 8. Validation & Doctor Checks (v0.1)

`portaqemu doctor` MUST include:

* [PASS/WARN/FAIL] Fragment root exists or is creatable
* [PASS/WARN/FAIL] Fragment file exists (if installed)
* [PASS/WARN/FAIL] Fragment JSON parses
* [PASS/WARN/FAIL] Profile GUID is stable (compare to expected)

Optional best-effort:

* attempt to locate `wt.exe` in PATH and show its path
* warn if not found (Terminal may still exist via Windows app alias)

---

## 9. Security & Safety Notes

* Fragment installation MUST not write anywhere outside LocalAppData.
* Fragment removal MUST not delete other app directories.
* `commandline` MUST quote paths safely to prevent injection when root path contains spaces.

---

## References

* Microsoft Learn: JSON fragment extensions in Windows Terminal ([Microsoft Learn][1])
* Fragment vs settings path discussion (MS employee) ([Stack Overflow][2])
* `source` field indicates fragment/dynamic origin ([Super User][3])

---

[1]: https://learn.microsoft.com/en-us/windows/terminal/json-fragment-extensions?utm_source=chatgpt.com "JSON fragment extensions in Windows Terminal"
[2]: https://stackoverflow.com/questions/72036145/where-should-i-generate-windows-terminal-json-fragment-extensions?utm_source=chatgpt.com "Where should I generate Windows Terminal JSON ..."
[3]: https://superuser.com/questions/1663138/git-bash-not-using-the-source-value-specified-in-windows-terminal-settings?utm_source=chatgpt.com "Git Bash not using the source value specified in windows ..."