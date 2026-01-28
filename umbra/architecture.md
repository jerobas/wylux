### **Protocol Layer**
```
Message Format (JSON over vsock):
{
  "id": "uuid-here",           // For matching request/response
  "type": "call" | "response" | "event",
  "function": "open_vscode",   // Function name
  "args": {...},               // JSON arguments
  "result": "...",             // For responses
  "error": "..."               // If error occurred
}
```

### **Core Components**

**1. Registry Pattern (both sides)**
```
FunctionRegistry {
  - HashMap<String, Box<dyn Fn(Args) -> Result>>
  - register("function_name", handler)
  - call("function_name", args) -> Result
}
```

**2. Connection Manager**
```
ConnectionManager {
  - VsockStream (persistent connection)
  - Sender/Receiver channels
  - pending_requests: HashMap<UUID, oneshot::Sender>
  - async event loop that:
    * Reads messages from vsock
    * Routes "call" messages to FunctionRegistry
    * Routes "response" messages to pending_requests
    * Routes "event" messages to EventBus
}
```

**3. Event Bus (both sides)**
```
EventBus {
  - listeners: HashMap<String, Vec<Callback>>
  - on("event_name", callback)
  - emit("event_name", data)
  - (emits get serialized and sent to other side)
}
```

**Usage Pattern**

- Host side (expose Windows functions):

```rust
// Register functions
registry.register("open_vscode", |args| {
    let path = args["path"].as_str()?;
    Command::new("code").arg(path).spawn()?;
    Ok("opened".into())
});

registry.register("get_clipboard", |_args| {
    let text = get_windows_clipboard()?;
    Ok(text.into())
});

// Listen for events from guest
events.on("file_saved", |data| {
    println!("Guest saved file: {}", data);
});
```

- Guest side (call host functions):

```rust
// Call host function
let result = connection.call("open_vscode", json!({
    "path": "/home/user/project"
})).await?;

// Emit event to host
connection.emit("file_saved", json!({
    "path": "/home/user/file.txt"
})).await?;

// Listen for host events
events.on("clipboard_changed", |data| {
    println!("Host clipboard: {}", data);
});
```

**Why This Architecture Works**

Adding new functions = just one registry.register() call
Bidirectional = both sides have identical Registry/EventBus, just different functions registered
Async-ready = naturally handles stdout streaming later (just emit events for each line)
Type-safe = Rust's type system + serde catches errors at compile time
Single persistent connection = efficient, low latency

For your "capture output later" goal, you'd just change a function from:

```rust
rustOk(output.into())  // Return full string
```

to:

```rust
// Stream each line as event
for line in child.stdout.lines() {
    connection.emit("process_stdout", json!({"line": line})).await?;
}
```

This architecture scales from simple string returns to full bidirectional streaming naturally. Make sense?