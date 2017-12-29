extern crate vlq;

use vlq::{decode, encode};

fn decode_tester_ok(input: &[u8], expect: i64) {
    let mut input = input.iter().cloned();
    match decode(&mut input) {
        Ok(x) => {
            assert_eq!(x, expect);
            assert!(input.next().is_none());
        }
        _ => assert!(false),
    }
}

#[test]
fn test_decode() {
    decode_tester_ok("A".as_bytes(), 0);
    decode_tester_ok("B".as_bytes(), 0);
    decode_tester_ok("C".as_bytes(), 1);
    decode_tester_ok("D".as_bytes(), -1);
}

#[test]
fn test_roundtrip() {
    for val in -512..512 {
        let mut buf = Vec::<u8>::new();
        match encode(val, &mut buf) {
            Ok(()) => assert!(buf.len() > 0),
            _ => assert!(false),
        }
        decode_tester_ok(&buf, val);
    }
}

