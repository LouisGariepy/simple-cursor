# `simple-cursor`

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/simple-cursor">
    <img alt="Crates.io version" src="https://img.shields.io/crates/v/simple-cursor.svg?style=flat-square"/>
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/simple-cursor/latest/simple_cursor/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/simple-cursor?style=flat-square"/>
  </a>
  
  <!-- Dependencies -->
  <a href="https://deps.rs/repo/github/LouisGariepy/simple-cursor">
    <img alt="Crates.io version" src="https://deps.rs/repo/github/LouisGariepy/simple-cursor/status.svg?style=flat-square"/>
  </a>

  <!-- no_std -->
  <picture>
    <img alt="no_std compatible" src="https://img.shields.io/badge/no__std-compatible-light_green?style=flat-square"/>
  </picture>

 
  <!-- License -->
  <a href="https://github.com/LouisGariepy/simple-cursor#License">
    <img alt="License" src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square"/>
  </a>
</div>

A super simple `#[no_std]`-compatible character cursor implementation geared towards lexers/tokenizers. The implementation is inspired by the one used in `rustc` and should be performant enough to handle pretty much anything you could throw at it.


# Basic use
The following examples showcases the basic features of `simple_cursor`. Please refer to the `Cursor` [docs](https://docs.rs/simple-cursor/latest/simple_cursor/struct.Cursor.html) for more info.

```rust
use simple_cursor::Cursor;

// Create the input string and the cursor.
let input = "123 foobar竜<!>";
let mut cursor = Cursor::new(input);

// "123"
let number_start = cursor.byte_pos();
cursor.skip_while(|c| c.is_ascii_digit());
let number_end = cursor.byte_pos();

// Some(' ')
let whitespace = cursor.bump();

// "foobar"
let ident_start = cursor.byte_pos();
cursor.skip_while(|c| c.is_ascii_alphabetic());
let ident_end = cursor.byte_pos();

// "竜<!>"
let rest_start = ident_end;
let rest_end = input.len();

assert_eq!("123", &input[number_start..number_end]);
assert_eq!(Some(' '), whitespace);
assert_eq!("foobar", &input[ident_start..ident_end]);
assert_eq!("竜<!>", &input[rest_start..rest_end]);
```