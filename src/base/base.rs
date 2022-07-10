use std::collections::{HashMap, BTreeMap};

#[derive(Debug)]
#[warn(dead_code)]
pub struct StructA {
    pub name:&'static str
}

impl StructA {
    pub fn new(name: &'static str) -> Self {
        return StructA { name: name }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn test(&self) {
        // 普通
        let a = 1i32;
        println!("{}",a);

        // 可变
        let mut a1 = 2i32;
        println!("{}",a1);
        a1 = 3;
        println!("{}",a1);

        let mut mp:HashMap<i32, &str> = HashMap::new();
        mp.insert(1, "A");
        mp.insert(2, "B");
        mp.insert(3, "C");

        let mut ord_mp:BTreeMap<i32,&str> = BTreeMap::new();
        ord_mp.insert(1, "A");
        ord_mp.insert(2, "B");
        ord_mp.insert(3, "C");
        println!("{:?}",mp);
        println!("{:?}",ord_mp);
    }
}