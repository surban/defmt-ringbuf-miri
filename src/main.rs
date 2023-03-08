use core::mem::MaybeUninit;

mod ring_buffer;

pub use ring_buffer::{RingBuf, RingBuffer};

// On the embedded system this would be placed in a linker section
// that is not initialized at start.
// #[link_section = ".uninit"]
static mut BUFFER: MaybeUninit<RingBuffer<8192>> = MaybeUninit::uninit();

fn main() {
    let buffer = unsafe { RingBuffer::init(&mut BUFFER) };

    let mut data = [0; 16];

    assert_eq!(buffer.read(&mut data), (0, false));

    buffer.write(&[1, 2, 3, 4]);

    assert_eq!(buffer.read(&mut data), (4, false));
    assert_eq!(&data[..4], &[1, 2, 3, 4]);
}
