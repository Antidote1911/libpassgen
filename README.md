[![Crates.io](https://img.shields.io/crates/v/libpassgen?style=flat-square)](https://crates.io/crates/libpassgen)
[![Crates.io](https://img.shields.io/crates/d/libpassgen?style=flat-square)](https://crates.io/crates/libpassgen)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/Antidote1911/libpassgen/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/Antidote1911/libpassgen/blob/master/LICENSE-APACHE)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![Cargo Build & Test](https://github.com/Antidote1911/libpassgen/actions/workflows/ci.yml/badge.svg)](https://github.com/Antidote1911/libpassgen/actions/workflows/ci.yml)
# libpassgen

Crate to generate pseudo-random passwords.  
This is the [Passgen](https://github.com/Antidote1911/passgen) core. A cli app to generate passwords.

## USAGE

Generate a 15 chars password with the given "pool" :
```rust
use libpassgen::*;

fn main() {
let mut pool = Pool::new();
pool.extend_from_string("123456789");
let password = generate_password(&pool, 15);
println!("{}",password);
}

```

Generate 100 passwords with 15 chars with the given "pool" :
```rust
use std::str::FromStr;
use libpassgen::*;

fn main() {
let mut pool = Pool::from_str("1234567").unwrap();
let vec_passwords = generate_n_passwords(&pool, 15, 100);
for n in 0..vec_passwords.len() {
    println!("{}",vec_passwords[n]);
	}
}
```

Have a look to [Passgen](https://github.com/Antidote1911/passgen) cli app for full example.
