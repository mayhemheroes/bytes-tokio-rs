#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{BytesMut, BufMut};

const N: usize = 10;

fuzz_target!(|input: u16| {


    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);

    for i in 0..N - 1 {

        buf.put_u16(input);
    }



});