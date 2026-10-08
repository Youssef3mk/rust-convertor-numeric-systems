# rust-convertor-numaric-systems
rust tool for converto from and to diff numaric systems
it is firist project for me expected it is not perfect  
# usage 
use libutil;
fn main() {
    println!(
        "value is {}",
        libutil::to_hex(&String::from("1110101110110100011"), libutil::ENsys::BIN)
    );
}


 libutil :local lib contain func such to_hex ,to_dec,to_oct,to_bin
 have arguments   &string and  the current number system such  libutil::ENsys::BIN   