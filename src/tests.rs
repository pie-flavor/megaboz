use crate::*;

#[test]
fn load_minizork() {
    println!(
        "{}",
        ZMachine::from_file("minizork.z3").unwrap().get_zstring(WordAddress::from(ByteAddress(0xb106))).unwrap()
    );
}
