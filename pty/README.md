# lunel-pty

A terminal session manager that provides JSON-based terminal emulation over stdin/stdout.

## Building

### Desktop (macOS/Linux/Windows)

```bash
cargo build --release
```

### iOS (Real Device)

Requires Xcode and iOS SDK:

```bash
cargo build --target aarch64-apple-ios --release
```

### iOS (Simulator)

```bash
cargo build --target aarch64-apple-ios-sim --release
```

## Usage

```bash
./lunel-pty
```

Send JSON commands via stdin. See `protocol.rs` for command/event definitions.

## iOS Notes

- The iOS build requires `UIKit` and `CoreGraphics` frameworks
- Use `xcrun` to compile with the iOS SDK
- For real devices, code signing is required
- Simulator builds can be tested using the iOS Simulator
