use crate::*;

#[test]
fn load_minizork() {
    println!(
        "{}",
        ZMachine::from_file("minizork.z3").unwrap().get_abbrvd_zstring(ZStringAbbrv::new(20).unwrap())
    );
}
