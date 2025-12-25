# barelog
[![Cargo CI](https://github.com/s0uthview/barelog/actions/workflows/cargo.yml/badge.svg)](https://github.com/s0uthview/barelog/actions/workflows/cargo.yml)
A simple logger designed to work in `no-std` environments, such as embedded systems and in OS development.

Features:
- 📊 multiple log levels
- 🔌 pluggable subscribers
- 🛡️ thread safe API
- 📝 logging macros for each level
- 🎚️ configurable log level filtering
- 🌈 optional color output
- 🧩 alloc support

```rust
#![no_std]
#![no_main]

use barelog::{Level, info, error, Subscriber};
use core::fmt::{Arguments, Write};
use uart_16550::SerialPort;

static mut SERIAL1: Option<SerialPort> = None;

struct SerialLogger;

impl Subscriber for SerialLogger {
    fn log(&self, level: Level, args: Arguments) {
        unsafe {
            if let Some(serial) = SERIAL1.as_mut() {
                let _ = write!(serial, "[{}] {}\r\n", level.as_str(), args);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        SERIAL1 = Some(SerialPort::new(0x3F8));
        if let Some(serial) = SERIAL1.as_mut() {
            serial.init();
        }
    }

    barelog::set_max_level(Level::Info);
    barelog::set_subscriber(&SerialLogger).unwrap();

    info!("Hello from barelog over serial!");
    error!("This is an error message");

    loop {}
}
```
### Add crate to dependencies
In your `Cargo.toml`:
```toml
[dependencies]
barelog = { git = "https://github.com/s0uthview/barelog.git" }
```

Note: this repository is a work in progress and things may be buggy. I mainly made this for a personal project but feel free to use/contribute! <3