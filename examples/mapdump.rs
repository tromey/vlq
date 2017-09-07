extern crate vlq;

use std::env;
use std::io::Read;
use vlq::decode;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let mut line = 0;
    let mut orig_line = 0;
    let mut orig_column = 0;
    let mut source_number = 0;
    let mut name_number = 0;

    for group in arg.split(';') {
        println!("================\nLine {}", line);

        let mut column = 0;
        for segment in group.split(',') {
            let bytes = segment.as_bytes();
            let mut slice = &bytes[..];
            let input: &mut Read = &mut slice;

            let col_delta = decode(input).expect("column needed");
            column += col_delta;
            println!("   column {}", column);

            match decode(input) {
                Err(_) => {}
                Ok(s) => {
                    source_number += s;
                    println!("   source #{}", source_number);

                    let line_delta = decode(input).unwrap();
                    orig_line += line_delta;
                    println!("   orig line {}", orig_line);

                    let col_delta = decode(input).unwrap();
                    orig_column += col_delta;
                    println!("   orig column {}", orig_column);

                    match decode(input) {
                        Err(_) => {},
                        Ok(n) => {
                            name_number += n;
                            println!("   name #{}", name_number);
                        }
                    }
                }
            };

            println!("");
        }

        println!("");
        line += 1;
    }
}

