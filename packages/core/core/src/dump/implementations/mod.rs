use crate::Dump;

mod cow;
mod hash_map;
mod primitive;
mod tuple;
mod phantom;



impl<T:Dump + ?Sized> Dump for std::rc::Rc<T> {
    fn dump(&self) -> String {
        self.as_ref().dump()
    }
}
impl<T:Dump + ?Sized> Dump for Box<T> {
    fn dump(&self) -> String {
        self.as_ref().dump()
    }
}
impl Dump for () {
    fn dump(&self) -> String {
        String::from("()")
    }
}


impl<T:Dump + ?Sized> Dump for std::cell::RefCell<T> {
    fn dump(&self) -> String {
        format!("RefCell {{ value: {} }}", self.borrow().dump())
    }
}

impl<T:Dump> Dump for Option<T> {
    fn dump(&self) -> String {
        match self {
            Some(value) => format!("Some({})", value.dump()),
            None => "None".into()
        }
    }
}

impl<T:Dump> Dump for Vec<T> {
    fn dump(&self) -> String {
        format!("[{}]", self.iter().map(|value|value.dump()).collect::<Vec<_>>().join(", "))
    }
}