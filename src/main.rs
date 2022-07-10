mod base;

use base::base::StructA;

fn main() {
    let a = StructA::new("A");
    a.test();
    a.get_name();
}
