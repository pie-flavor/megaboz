use crate::*;

#[test]
fn read_zstring() {
    let dam_plaque = ZMachine::from_file("minizork.z3").unwrap().get_zstring(WordAddress::from(ByteAddress(0xb106))).unwrap();
    assert_eq!(dam_plaque, r#""Flood Control Dam #3 was constructed in 783 GUE with a grant of 37 million zorkmids from Lord Dimwit Flathead the Excessive. This impressive structure is composed of 370,000 cubic feet of concrete, is 256 feet tall and 193 feet wide.

The construction of FCD#3 took 112 days from ground breaking to dedication. It required a work force of 384 slaves, 34 slave drivers, and 12 engineers, 2345 bureaucrats, and nearly one million dead trees.

As you start your tour, notice the more interesting features of FCD#3. On your right..."#)
}

#[test]
fn read_abbrv() {
    let abbrv = ZMachine::from_file("minizork.z3").unwrap().get_abbrvd_zstring(ZStringAbbrv::new(20).unwrap());
    assert_eq!(abbrv, "through ")
}
