xpra start :100 \
  --bind-tcp=0.0.0.0:14500 \
  --daemon=yes \
  --exit-with-client=no \
  --exit-with-windows=no \
  --start-new-commands=yes \
  --encoding=png --video=no \
  --html=no --mdns=no --notifications=no

“How do I move windows if Ctrl+Alt+drag doesn’t work?”

This is the subtle part — and the answer depends on keyboard layout + Windows quirks, not Xpra itself.

Why Ctrl+Alt+drag fails for you

You’re on:

Windows client

PT keyboard layout

Possibly AltGr (which = Ctrl+Alt internally)

So:

Ctrl + Alt is being eaten by the OS / layout

Xpra never receives it.

That’s why nothing happens.

✅ Correct solution: change the Xpra move modifier

You do not need decorations.
You just need a different modifier.

Pick a modifier that Windows won’t steal

Good choices:

Meta (Windows key)

Shift

Control alone

Start the server with a custom modifier

Example (Windows key + drag to move):

xpra start :100 \
  --bind-tcp=127.0.0.1:14500 \
  --daemon=yes \
  --window-move-modifier=meta \
  --window-resize-modifier=meta


Now:

Win + left-click drag → move window

Win + right-click drag → resize window

No decorations.
No WM.
No black desktop.
No Electron weirdness.

🔁 If the server is already running

You can also set this client-side (works in most builds):

xpra_cmd attach tcp://localhost:14500/ --window-move-modifier=meta


If that doesn’t apply immediately, restart the server once with the flag.

❌ Why decorations feel ugly (and you’re right)

They are server-side

They don’t match Windows

Electron apps already draw their own UI

You get double chrome

You’re absolutely right to avoid them.

⚠️ Important rule when running WM-less

To keep things stable:

❌ Don’t minimize from the Windows taskbar

❌ Don’t use Windows snap (Win+Arrow)

✅ Move via modifier + drag

✅ Resize via modifier + right-drag

Xpra is managing geometry — Windows snapping breaks that contract.

✅ TL;DR

Yes, you can:

Start Xpra with no apps

Attach clients anytime

Start apps later

To move windows without decorations:

Ctrl+Alt doesn’t work because of PT / AltGr

Use:

--window-move-modifier=meta


Then move with Win + drag



🟢 xpra

The main binary (both server and client).

xpra start

xpra start-desktop

xpra stop

xpra list

xpra start-child

xpra attach

On Linux, this is the command.

🟢 xpra_cmd

Same as xpra, but console subsystem on Windows.

Shows logs in PowerShell

Does not hide stdout

This is the one you want in terminals

Think:

xpra_cmd = xpra, but Windows-friendly

🟡 xpra-launcher

GUI helper.

Stores connection profiles

Starts xpra attach under the hood

No unique functionality

Good UX, bad for learning

You lose nothing by using it — it’s just a wrapper.

🟡 xpra-proxy

Used when:

multiple clients

authentication gateways

load balancing

WAN setups

You do not need this right now.

🟡 xpra-proxy_cmd

Same as above, but console version.

Ignore for now.

🟡 xpra-shadow

Very different mode.

Shares an existing real X11 desktop

Similar to VNC

No app persistence

No per-app control

You explicitly do not want this.