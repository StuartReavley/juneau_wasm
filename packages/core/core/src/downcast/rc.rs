use std::{marker::Unsize, rc::Rc};
use std::cell::RefCell;
use crate::RcRefCell;
use super::Downcast;




pub trait UpcastRc<U> {
    fn upcast<T:?Sized>(self) -> Rc<T> where U: Unsize<T>;
}

impl<U:'static> UpcastRc<U> for Rc<U> {
    fn upcast<T:?Sized>(self) -> Rc<T> where U: Unsize<T> {
        self
    }
}


pub trait DowncastRc<T:?Sized> {
    fn downcast<U:'static>(self) -> Option<Rc<U>> where U:Unsize<T>;
    fn downcast_cell<U:'static>(self) -> Option<RcRefCell<U>> where RefCell<U>:Unsize<T>;
}

impl<T:Downcast + ?Sized> DowncastRc<T> for Rc<T> {
    fn downcast<U:'static>(self) -> Option<Rc<U>> where U:Unsize<T> {
        self.into_any_rc().downcast::<U>().ok()
    }

    fn downcast_cell<U:'static>(self) -> Option<RcRefCell<U>> where RefCell<U>:Unsize<T> {
        self.downcast::<RefCell<U>>().map(RcRefCell::from_rc)
    }
}



#[cfg(test)]
mod test {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};
    use crate::downcast::{Downcast, DowncastRc};
    use crate::downcast::rc::UpcastRc;

    trait Animal:Downcast {}


    struct Cat {
        pub ear_count: isize
    }
    impl Animal for Cat {}

    struct Dog {
        pub tail_count: isize
    }
    impl Animal for Dog {}


    #[test]
    fn test_upcast_rc() {
        let cat = Rc::new(Cat{ear_count: 2});
        let animal = cat.upcast::<dyn Animal>();
        let cat = animal.clone().downcast::<Cat>();
        let cat = cat.unwrap();
        assert_eq!(cat.ear_count, 2);

        let dog = animal.clone().downcast::<Dog>();
        assert_eq!(dog.is_none(), true);
    }

    #[test]
    fn test_downcast_rc() {
        let cat = Rc::new(Cat{ear_count: 2});
        let animal = cat as Rc<dyn Animal>;
        let cat = animal.clone().downcast::<Cat>();
        let cat = cat.unwrap();
        assert_eq!(cat.ear_count, 2);

        let dog = animal.clone().downcast::<Dog>();
        assert_eq!(dog.is_none(), true);
    }

    #[test]
    fn test_downcast_rc_collection() {
        let mut animals = HashMap::<String, Rc<dyn Animal>>::new();
        animals.insert("cat".into(), Rc::new(Cat{ear_count: 3}));
        animals.insert("dog".into(), Rc::new(Dog{tail_count: 1}));

        let cat = animals.get("cat").and_then(|animal|animal.clone().downcast::<Cat>());
        let cat = cat.unwrap();
        assert_eq!(cat.ear_count, 3);

        let dog = animals.get("cat").and_then(|animal|animal.clone().downcast::<Dog>());
        assert_eq!(dog.is_none(), true);
    }
}