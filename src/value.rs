use std::{cell::Cell, collections::HashMap};

#[derive(PartialEq, Debug, Clone)]
pub struct Value {
    pub number: Cell<isize>,
    pub next: Option<Box<Value>>,
}

impl Value {
    pub fn get_total(&self) -> isize {
        match self.next.as_ref() {
            Some(value) => value.get_total() + self.number.get(),
            None => self.number.get(),
        }
    }
}

pub fn interior_mutability_lab() {
    let mut lazy_register: HashMap<String, &Box<Value>> = HashMap::new();

    let mut a_value = Box::new(Value {
        number: Cell::new(0),
        next: None,
    });

    a_value.number.set(a_value.number.get() + 1);

    let b_value = Box::new(Value {
        next: None,
        number: Cell::new(5),
    });

    // Clone needs to be replaced to make things work.
    a_value.next = Some(b_value.clone());

    b_value.number.set(10);

    lazy_register.insert("a".to_string(), &a_value);

    println!("{lazy_register:?}");

    // Total should be 10 + 1, but ends up being 6 + 1 due to the clone() call.
    let a_total = lazy_register.get("a").unwrap().get_total();

    println!("{a_total}");
}
