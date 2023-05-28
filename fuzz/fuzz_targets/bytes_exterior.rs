#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{BytesMut, BufMut};


fuzz_target!(|input: u16| {

    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16(input);
});