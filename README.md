# Aero Traffic Control

A Rust CLI tool for intelligent window management using [AeroSpace](https://github.com/nikitabobko/AeroSpace) window manager on macOS.

## Overview

Aero Traffic Control provides smart application window cycling and focus management. When you specify an application, it will:

- **Focus existing windows**: If the app is already running but not focused, focus its first window
- **Cycle through windows**: If the app is already focused, cycle to the next window of that application
- **Launch applications**: If the app isn't running, launch it using macOS `open` command

## Prerequisites

- macOS
- [AeroSpace](https://github.com/nikitabobko/AeroSpace) window manager installed and running
- Rust toolchain (for building from source)

## Installation

```bash
git clone https://github.com/thomaschrstnsn/aero-traffic-control
cd aero-traffic-control
cargo install --path .
```

The binary will be available at `~/.cargo/bin`.

## Usage

```bash
aero-traffic-control <app-name>
```

**Examples:**

```bash
# Focus or cycle through Safari windows
aero-traffic-control Safari

# Focus or cycle through Terminal windows  
aero-traffic-control Terminal

# Focus or launch VS Code
aero-traffic-control "Visual Studio Code"
```

## How it Works

1. Lists all windows using AeroSpace's `list-windows` command
2. Filters windows by the specified application name
3. Checks if the application is currently focused
4. Takes appropriate action based on current state:
   - **App not focused**: Focus the first available window
   - **App focused**: Cycle to the next window (or wrap to first)
   - **App not running**: Launch the application

## Dependencies

- `serde` - JSON serialization/deserialization
- `tracing` - Structured logging
- `thiserror` - Error handling

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

