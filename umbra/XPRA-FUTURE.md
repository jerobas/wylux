---

## 1) If I close the window on Windows, does it close the Linux app?

**Yes, in the normal case.** When you click the **X** on the forwarded window, Xpra sends the X11 “close” to that app (like clicking close locally). Most apps will exit.

But there are two important exceptions:

* **Apps that “close to tray” / background** (some Electron apps) may keep running.
* If the app ignores the close request, it’ll stay running.

### How to be sure (authoritative checks on Linux)

After closing on Windows, run on the VM:

```bash
pgrep -af cursor || echo "cursor not running"
```

And/or see if the window still exists:

```bash
xpra list-windows
```

If Cursor still shows there after you closed it on Windows, then it’s still running (either in background or the window is still mapped).

---

## 2) Daemon + start at boot (Linux)

### Is `--daemon=yes` the only daemon option?

For Xpra itself, **yes — that’s the built-in daemonization toggle** (`--daemon=yes/no`). You can also run it under a service manager (recommended).

### Best way to start with the OS: **systemd user service**

This is the cleanest: autostart, restart-on-crash, logs, etc.

Create:
`~/.config/systemd/user/xpra@:100.service`

```ini
[Unit]
Description=Xpra server on display %i
After=network.target

[Service]
Type=forking
ExecStart=/usr/bin/xpra start %i --bind-tcp=0.0.0.0:14500 --daemon=yes --exit-with-client=no --exit-with-windows=no --start-new-commands=yes --encoding=png --video=no --html=no --mdns=no --notifications=no
ExecStop=/usr/bin/xpra stop %i
Restart=on-failure

[Install]
WantedBy=default.target
```

Enable it:

```bash
systemctl --user daemon-reload
systemctl --user enable --now xpra@:100
```

If you want it to run even when you’re not logged in:

```bash
loginctl enable-linger $USER
```

That’s the “starts with OS” solution.

---

## 3) Do I really need `&` when launching apps?

If you launch like this:

```bash
DISPLAY=:100 cursor
```

your shell waits until Cursor exits. So **yes**, you either need `&` or some detaching method.

### Best alternatives (no hostage shell)

**Option A: `&` (simple)**

```bash
DISPLAY=:100 cursor &
```

**Option B: `nohup` (survives hangups)**

```bash
nohup env DISPLAY=:100 cursor >/dev/null 2>&1 &
```

**Option C: start it in your terminal multiplexer (tmux)**

```bash
tmux new -d "DISPLAY=:100 cursor"
```

If you want “fire and forget”, use **nohup**.

---

## 4) Performance: are you losing too much? How to go “max performance”?

### With your current safe config (`--encoding=png --video=no`)

You’re trading:

* ✅ stability (especially Electron)
* ✅ fewer codec bugs
  for:
* ❌ more bandwidth
* ❌ more CPU for large screen changes

For Cursor specifically (lots of UI redraw, text, scrolling), PNG can be *okay*, but it’s usually **not maximum performance**.

### How to test maximum performance properly

Do it in controlled steps:

#### Step 1 — confirm your client has GPU accel

On Windows client logs you earlier had “OpenGL is enabled” — that’s good.

#### Step 2 — enable video codecs on Linux server

Install the missing pieces you were warned about (Arch):

```bash
sudo pacman -S gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly
```

Then restart Xpra with video enabled and let it auto-pick:

```bash
xpra start :100 \
  --bind-tcp=0.0.0.0:14500 \
  --daemon=yes \
  --exit-with-client=no --exit-with-windows=no \
  --start-new-commands=yes \
  --encoding=auto \
  --html=no --mdns=no --notifications=no
```

#### Step 3 — if you see that WebP encoder crash again

Keep video, but ban webp by not using it (since your crash was webp-related). Easiest: prefer h264:

```bash
xpra start :100 ... --encoding=h264
```

#### Step 4 — measure

While using Cursor, watch:

```bash
top
```

and network usage (even `nload` helps):

```bash
sudo pacman -S nload
nload
```

If CPU is high and bandwidth is high → codec choice is wrong.
If CPU is low and bandwidth is reasonable → you’re in the sweet spot.

---

### My practical recommendation for Cursor

* Start stable with **PNG** while you validate workflow.
* Then move to **H.264** once GStreamer is installed.
* Avoid WebP until you confirm it’s fixed in your stack.

---