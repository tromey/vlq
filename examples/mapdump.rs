extern crate vlq;

use std::env;
use std::io::stdin;
use vlq::decode;

fn main() {
    let arg = if env::args().count() > 1 {
        env::args().nth(1).unwrap()
    } else {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(error) => panic!("error: {}", error),
        }
    };
    let mut line = 0;
    let mut orig_line = 0;
    let mut orig_column = 0;
    let mut source_number = 0;
    let mut name_number = 0;

    for group in arg.split(';') {
        println!("================\nLine {}", line);

        if group.is_empty() {
            line += 1;
            continue;
        }

        let mut column = 0;
        for segment in group.split(',') {
            let bytes = segment.as_bytes();
            let mut slice = &bytes[..];
            let input = &mut slice;

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
