#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{BytesMut, BufMut};

const N: usize = 10;

fuzz_target!(|data: (u16, i32)| {

    let (input, capacity) = data;
    let mut buf = BytesMut::with_capacity(capacity);

    buf.put(&b"hello world"[..]);

    for i in 0..N - 1 {

        buf.put_u16(input);
    }

    let a = buf.split();
    // assert_eq!(a, b"hello world\x04\xD2"[..]);




});