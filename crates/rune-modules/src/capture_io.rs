//! I/O module capable of capturing what's been written to a buffer.
//!
//! ```
//! use rune::{Context, ContextError};
//! use rune_modules::capture_io::{self, CaptureIo};
//!
//! # fn main() -> Result<(), ContextError> {
//! let io = CaptureIo::new();
//!
//! let mut c = rune_modules::with_config(false)?;
//! c.install(&capture_io::module(&io)?)?;
//! # Ok(()) }
//! ```

use parking_lot::Mutex;
use rune::runtime::{Panic, Stack, VmError};
use rune::{ContextError, Module, Value};
use std::io::{self, Write};
use std::string::FromUtf8Error;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct CaptureIo {
    inner: Arc<Mutex<Vec<u8>>>,
}

impl CaptureIo {
    /// Construct a new capture.
    pub fn new() -> Self {
        Self::default()
    }

    /// Drain all captured I/O that has been written to output functions.
    pub fn drain(&self) -> Vec<u8> {
        let mut o = self.inner.lock();
        std::mem::take(&mut *o)
    }

    /// Drain all captured I/O that has been written to output functions into
    /// the given [Write].
    pub fn drain_into<O>(&self, mut out: O) -> io::Result<()>
    where
        O: Write,
    {
        let mut o = self.inner.lock();
        out.write_all(&*o)?;
        o.clear();
        Ok(())
    }

    /// Drain all captured I/O that has been written to output functions and try
    /// to decode as UTF-8.
    pub fn drain_utf8(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.drain())
    }
}

/// Provide a bunch of `std` functions that can be used during tests to capture output.
pub fn module(io: &CaptureIo) -> Result<Module, ContextError> {
    let mut module = Module::with_crate_item("std", &["io"]);

    let o = io.clone();

    module.function(&["print"], move |m: &str| {
        write!(o.inner.lock(), "{}", m).map_err(Panic::custom)
    })?;

    let o = io.clone();

    module.function(&["println"], move |m: &str| {
        writeln!(o.inner.lock(), "{}", m).map_err(Panic::custom)
    })?;

    let o = io.clone();

    module.raw_fn(&["dbg"], move |stack, args| {
        let mut o = o.inner.lock();
        dbg_impl(&mut *o, stack, args)
    })?;

    Ok(module)
}

fn dbg_impl<O>(o: &mut O, stack: &mut Stack, args: usize) -> Result<(), VmError>
where
    O: Write,
{
    for value in stack.drain(args)? {
        writeln!(o, "{:?}", value).map_err(VmError::panic)?;
    }

    stack.push(Value::Unit);
    Ok(())
}
