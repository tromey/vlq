//! An implementation of the Base64-encoded
//! [VLQ](https://en.wikipedia.org/wiki/Variable-length_quantity)
//! encoding.  Note that there are several variants of VLQ.  This only
//! implements the variant used by [source
//! maps](https://github.com/mozilla/source-map).

#![deny(missing_docs)]

use std::io;

// Some constants defined by the spec.
const SHIFT: u8 = 5;
const MASK: u8 = (1 << SHIFT) - 1;
const CONTINUED: u8 = 1 << SHIFT;

/// An error that occurred while decoding.
#[derive(Debug)]
pub enum Error {
    /// The input contained an invalid byte.
    InvalidBase64(u8),
    /// An I/O error occurred while reading.
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

/// The result of decoding.
pub type Result<T> = std::result::Result<T, Error>;

// Decode a single base64 digit.
fn decode64(input: u8) -> Result<u8> {
    match input {
        b'A'...b'Z' => Ok(input - b'A'),
        b'a'...b'z' => Ok(input - b'a' + 26),
        b'0'...b'9' => Ok(input - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(Error::InvalidBase64(input))
    }
}

/// Decode a single VLQ value from the input, returning the value.
pub fn decode(input: &mut io::Read) -> Result<i64> {
    let mut accum = 0;
    let mut shift = 0;
    let mut keep_going = true;
    while keep_going {
        let mut byte = [0; 1];
        input.read_exact(&mut byte[..])?;
        let digit = decode64(byte[0])?;
        keep_going = (digit & CONTINUED) != 0;
        accum += ((digit & MASK) as u64) << shift;
        shift += SHIFT;
    }

    // The low bit holds the sign.
    let negate = (accum & 1) != 0;
    accum >>= 1;
    if negate {
        accum = accum.wrapping_neg();
    }

    Ok(accum as i64)
}

// Encode a single base64 digit.
fn encode64(value: u8) -> u8 {
    debug_assert!(value < 64);
    if value < 26 {
        value + b'A'
    } else if value < 52 {
        value - 26 + b'a'
    } else if value < 62 {
        value - 52 + b'0'
    } else if value == 62 {
        b'+'
    } else {
        assert!(value == 63);
        b'/'
    }
}

/// Encode a value as Base64 VLQ, sending it to the writer.
pub fn encode(value: i64, output: &mut io::Write) -> io::Result<()> {
    let signed = value < 0;
    let mut value = (value.wrapping_abs() as u64) << 1;
    if signed {
        value |= 1;
    }
    loop {
        let mut digit = value as u8 & MASK;
        value >>= SHIFT;
        if value > 0 {
            digit |= CONTINUED;
        }
        let bytes = [encode64(digit)];
        output.write_all(&bytes[..])?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    fn decode_tester_ok(input: &[u8], expect: i64) {
        let mut cursor = ::io::Cursor::new(input);
        match ::decode(&mut cursor) {
            Ok(x) => {
                assert_eq!(x, expect);
                assert_eq!(cursor.position(), input.len() as u64);
            },
            _ => assert!(false)
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
            match ::encode(val, &mut buf) {
                Ok(()) => assert!(buf.len() > 0),
                _ => assert!(false)
            }
            decode_tester_ok(&buf, val);
        }
    }
}
