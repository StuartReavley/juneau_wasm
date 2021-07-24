use std::fmt::Debug;
use juneau_core::RcRefCell;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Id {
    pub value: u64
}


impl From<u64> for Id {
    fn from(value: u64) -> Id {
        Id {value}
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub trait IdContext {
    fn new_id(&mut self) -> Id;
}

#[derive(Debug)]
pub struct IdProvider {
    next_id: u64
}


impl IdProvider {
    pub fn new(next_id: u64) -> Self {
        Self {next_id}
    }

    pub fn new_cell(next_id:u64) -> RcRefCell<Self> {
        RcRefCell::new(Self::new(next_id))
    }

    pub fn increment_to(&mut self, value:u64) {
        if value < self.next_id {
            //panic!("cannot set value below next_id");
        }
        else {
            self.next_id = value;
        }
    }
}

impl IdContext for IdProvider {
    fn new_id(&mut self) -> Id {
        let id = Id::from(self.next_id);
        self.next_id += 1;
        id
    }
}

impl IdContext for RcRefCell<IdProvider> {
    fn new_id(&mut self) -> Id {
        self.borrow_mut().new_id()
    }
}
