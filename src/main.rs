#[allow(unused_imports)]
pub mod io;
use crate::io::io::*;

/*
   Program output will be:
   -=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    literal
    -1
    64
    flame egg
    1
    more valid line
    using a new
    delimiter
     and ignoring
     whitespaces
   -=-=-=-=-=-=-=-=-=-=-=-=-=-=-
*/
fn main() {
    // Create a new stream from `&str`
    let mut stream: IStream =
        IStream::from("string literal please 64 flame egg \nonly 1 more valid line");

    // skips the next token in the buffer, "string"
    stream.skip(1);

    // second_str contains "literal"
    let second_str: String = stream.next().unwrap_or_else(|| panic!("bonk"));
    println!("{second_str}");

    /*
     * `stream.buf` now contains everything from "please" to the end of the buffer
     */

    // `inv_i32` is an invalid call, because the next key in the buffer
    // is "please", which is not a valid int. It will default to -1
    // because of the call to `unwrap_or`
    let inv_i32: i32 = stream.next().unwrap_or(-1);
    println!("{inv_i32}");

    /*
     * `stream.buf` now contains everything from "64" to the end of the buffer
     */

    // first_i32 contains "64"
    let first_i32: i32 = stream.next().unwrap_or(-1);
    println!("{first_i32}");

    /*
     * `stream.buf` now contains everything from "flame" to the end of the buffer
     */

    // `to_end_of_line` contains everything in the stream until the next end line
    // (or end of the buffer, if that is the case). In this case, it contains
    // "flame egg"
    let to_end_of_line: String = stream.next_line().unwrap_or_else(|| panic!("bonk"));
    println!("{to_end_of_line}");

    /*
     * `stream.buf` now contains everything after the "\n"
     */

    // `next_i32` reads the buffer until a value is found that matches the
    // necessary type (i32). In this case, it will skip "only" and hold "1"
    let next_i32: i32 = stream.next_valid().unwrap_or_else(|| panic!("bonk"));
    println!("{next_i32}");

    /*
     * `stream.buf` now contains "more valid line"
     */

    // `remaining` now holds the rest of the buffer.
    // stream is now useless, as `stream.buf`` is empty
    let remaining: String = stream.flush();
    println!("{remaining}");

    let mut new_stream: IStream =
        IStream::from("using a new; delimiter\n and ignoring\n whitespaces");

    // `new_stream` will now use `;` as its delimiter, ignoring spaces.
    new_stream.use_delim(';');

    // `new_stream` will now ignore end line characters as delimiters.
    new_stream.ignore_endl(true);

    // `first` will contain everything until the first delimiter
    // so, first contains "using a new"
    let first: String = new_stream.next().unwrap_or_else(|| panic!("bonk"));
    println!("{first}");

    println!("{}", new_stream.next::<String>().unwrap());
}
