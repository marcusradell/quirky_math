use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Value {
    number: Cell<isize>,
    next: Option<Rc<RefCell<Value>>>,
}

impl Value {
    pub fn new_wrapped() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            next: None,
            number: Cell::new(0),
        }))
    }

    pub fn set_number(&self, number: isize) {
        self.number.set(number);
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Value>>) {
        self.next = Some(next);
    }

    pub fn get_total(&self) -> isize {
        match self.next.as_ref() {
            Some(value) => value.borrow_mut().get_total() + self.number.get(),
            None => self.number.get(),
        }
    }
}

pub fn interior_mutability_lab() {
    let mut lazy_register: HashMap<String, Rc<RefCell<Value>>> = HashMap::new();

    let a_value = Value::new_wrapped();

    a_value
        .borrow()
        .number
        .set(a_value.borrow().number.get() + 1);

    let b_value = Value::new_wrapped();
    b_value.borrow().set_number(5);

    a_value.borrow_mut().set_next(Rc::clone(&b_value));

    lazy_register.insert("a".to_string(), Rc::clone(&a_value));

    b_value.borrow().number.set(10);

    println!("{lazy_register:?}");

    // Total should be 10 + 1, but ends up being 6 + 1 due to the clone() call.
    let a_total = lazy_register.get("a").unwrap().borrow().get_total();

    println!("{a_total}");
}
