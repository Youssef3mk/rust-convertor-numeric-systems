use libutil;
fn main() {
    println!(
        "value is {}",
        libutil::to_hex(&String::from("1110101110110100011"), libutil::ENsys::BIN)
    );
}
