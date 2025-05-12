use std::ops::{Deref};

use circular_buffer::{CircularBuffer, Error};


fn main() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());

    buffer.write('2').unwrap();
    let _first = buffer[0];
    println!("first: {:?}", _first);

    buffer.make_contiguous();
    let inner_ref = buffer.deref();

    // let mut inner_ref2 = buffer.deref_mut();

    println!("inner ref: {:?}", inner_ref);
}