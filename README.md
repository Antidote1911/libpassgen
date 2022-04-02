[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Cargo Build & Test](https://github.com/Antidote1911/libpassgen/actions/workflows/ci.yml/badge.svg)](https://github.com/Antidote1911/libpassgen/actions/workflows/ci.yml)

# libpassgen

Crate to generate pseudo-random passwords.  
This is the [Passgen](https://github.com/Antidote1911/passgen) core. A cli app to generate passwords.

## USAGE

Generate a 15 chars password with the given "pool" :
```rust
use libpassgen::generate_password;

fn main() {
let mut pool = Pool::from_str("1234567").unwrap();
let password = generate_password(&Pool(pool), 15);
println!(password);
}

```

Generate 100 passwords with 15 chars with the given "pool" :
```rust
use libpassgen::generate_n_passwords;

fn main() {
let mut pool = Pool::from_str("1234567").unwrap();
let vec_passwords = generate_n_passwords(&Pool(pool), 15, 100);
for n in 0..vec_passwords.len() {
        println!(pass_vec[n]);
    }
}
```

Have a look to [Passgen](https://github.com/Antidote1911/passgen) cli app for full example.
