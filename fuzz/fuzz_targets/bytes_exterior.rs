#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{BytesMut, BufMut};

// const N: usize = 10;

fuzz_target!(|data: (u16, usize, usize)| {

    let (input, capacity, size) = data;
    let mut buf = BytesMut::with_capacity(capacity);

    // buf.put(&b"hello world"[..]);

    if size == 0 || size > 100000 {
        return
    }

    for i in 0..size- 1 {

        buf.put_u16(input);

    }



});