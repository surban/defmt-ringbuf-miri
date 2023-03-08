Hi!

I am writing a logger for an embedded system (STM32, but the details don't really matter here). The logger writes the log messages to a ring buffer in system memory (RAM), which can then be read at a later time.

I need the log messages to persist when the system is reset, because then they are of most interest (for example after a panic or watchdog reset). Hardware-wise this is not a problem because the embedded system does not clear its memory during reset. All I need to do here is to put the ring buffer into a linker section that is not initialized at program start.

However, in Rust there seems to be absolutely no valid way to read what it considers uninitialized memory. Even using `core::ptr::read_volatile` on "uninitialized" memory is considered undefined behavior.

I've published my code at https://github.com/surban/defmt-ringbuf-miri. This version can be run as a normal executable for testing, i.e. using `cargo run`. The code works fine, but running [MIRI](https://github.com/rust-lang/miri) on it produces the following error:

```
$ cargo +nightly miri run
Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)... done
   Compiling defmt-ringbuf-miri v0.2.0 (/data/surban/dev/defmt-ringbuf-miri)
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/surban/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/defmt-ringbuf-miri`
error: Undefined Behavior: using uninitialized data, but this operation requires initialized memory
  --> src/ring_buffer.rs:44:29
   |
44 |             let signature = (addr_of!((*ptr).signature) as *const u32).read_volatile();
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `ring_buffer::RingBuffer::<8192>::init` at src/ring_buffer.rs:44:29: 44:87
note: inside `main`
  --> src/main.rs:13:27
   |
13 |     let buffer = unsafe { RingBuffer::init(&mut BUFFER) };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
```

What can I do in Rust to read the contents of memory that is considered uninitialized without invoking undefined behavior?

Thanks for any ideas!
