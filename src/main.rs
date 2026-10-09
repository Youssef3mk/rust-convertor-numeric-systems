mod conv;
use conv::*;
fn main() {
    println!(
        "value is {}",
        to_hex(&String::from("1110101110110100011"), ENsys::BIN)
    );
}
