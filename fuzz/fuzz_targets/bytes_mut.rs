#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{BytesMut, BufMut};

// const N: usize = 10;

fuzz_target!(|data: (u16, usize, usize)| {

    let (input, capacity, size) = data;
    let mut buf = BytesMut::with_capacity(capacity);

    if size == 0 || size > 100000 {
        return
    }

    // capacity must be greater than size
    if input >= buf.len() as u16 {
        return
    }

    for i in 0..size- 1 {

        buf.put_u16(input);

    }

});