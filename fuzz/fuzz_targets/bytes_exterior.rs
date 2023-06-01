#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{Bytes};

// const N: usize = 10;

fuzz_target!(|data: (usize, usize)| {

    let (input, size) = data;
    // let mut buf = BytesMut::with_capacity(capacity);
    let mut buf = Bytes::from(&b"hello world"[..]);

    if size == 0 || size > 100000 {
        return
    }

    if input > buf.len() {
        return
    }

    for i in 0..size- 1 {

        buf.truncate(input);

    }



});