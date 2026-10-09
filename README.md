# Rust Numeric Systems Converter

A Rust tool and library designed to convert numbers between different numeric systems (Binary, Octal, Decimal, and Hexadecimal).

> **Note:** This is my very first project written in Rust, so it might not be perfect 

---

## 💡 Features

The local `conv` mod includes functions to convert numbers between various base systems:
- `to_bin`: Convert to Binary
- `to_oct`: Convert to Octal
- `to_dec`: Convert to Decimal
- `to_hex`: Convert to Hexadecimal

---

## 🚀 Usage

Each conversion function accepts a string reference (`&String`) containing the number to convert, and an enum (`conv::ENsys`) specifying its current numeric system.

### Example

```rust
use conv::*;

fn main() {
    // Convert a binary string to hexadecimal
    let result = to_hex(&String::from("1110101110110100011"), ENsys::BIN);

    println!("Value is {}", result);
}