use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Value {
    pub number: Cell<isize>,
    pub next: Option<Rc<RefCell<Value>>>,
}

impl Value {
    pub fn get_total(&self) -> isize {
        match self.next.as_ref() {
            Some(value) => value.borrow_mut().get_total() + self.number.get(),
            None => self.number.get(),
        }
    }
}

pub fn interior_mutability_lab() {
    let mut lazy_register: HashMap<String, Rc<RefCell<Value>>> = HashMap::new();

    let a_value = Rc::new(RefCell::new(Value {
        number: Cell::new(0),
        next: None,
    }));

    a_value
        .borrow()
        .number
        .set(a_value.borrow().number.get() + 1);

    let b_value = Rc::new(RefCell::new(Value {
        next: None,
        number: Cell::new(5),
    }));

    a_value.borrow_mut().next = Some(Rc::clone(&b_value));

    lazy_register.insert("a".to_string(), Rc::clone(&a_value));

    b_value.borrow().number.set(10);

    println!("{lazy_register:?}");

    // Total should be 10 + 1, but ends up being 6 + 1 due to the clone() call.
    let a_total = lazy_register.get("a").unwrap().borrow().get_total();

    println!("{a_total}");
}
