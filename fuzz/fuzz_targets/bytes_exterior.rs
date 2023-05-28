#![no_main]
use libfuzzer_sys::fuzz_target;


use bytes::{BytesMut, BufMut};


fuzz_target!(|input: &[u8]| {

    let mut buf = BytesMut::with_capacity(input);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);
});