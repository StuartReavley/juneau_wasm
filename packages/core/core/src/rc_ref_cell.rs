use std::{cell::{Ref, RefCell, RefMut}, marker::Unsize, rc::Rc};
use crate::{Dump, downcast::Downcast};





#[derive(Debug)]
pub struct RcRefCell<T:?Sized>(pub Rc<RefCell<T>>);


impl<T> RcRefCell<T> {
    pub fn new(value:T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}


impl<T:?Sized> RcRefCell<T> {

    pub fn ptr_eq(a:&RcRefCell<T>, b:&RcRefCell<T>) -> bool {
        Rc::ptr_eq(&a.0, &b.0)
    }

    pub fn from_rc(value:Rc<RefCell<T>>) -> Self {
        Self(value)
    }

    pub fn borrow(&self) -> Ref<'_, T>{
        match self.0.as_ref().try_borrow() {
            Ok(r) => r,
            Err(e) => panic!("borrow panic")
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        match self.0.as_ref().try_borrow_mut() {
            Ok(r) => r,
            Err(e) => panic!("borrow mut panic")
        }
    }
}

impl<T:?Sized> RcRefCell<T> where RefCell<T>: Downcast {
    pub fn downcast<U:'static>(self) -> Option<RcRefCell<U>> {
        self.0.into_any_rc().downcast::<RefCell<U>>().ok().map(|x|RcRefCell(x))
    }
}

impl<T> RcRefCell<T> {
    pub fn upcast<U:?Sized>(self) -> Rc<U> where RefCell<T>: Unsize<U> {
        self.0
    }

    pub fn upcast_cell<U:?Sized>(self) -> RcRefCell<U> where T: Unsize<U> {
        RcRefCell::from_rc(self.0.clone() as Rc<RefCell<U>>)
    }
}




impl<T:Dump+?Sized> Dump for RcRefCell<T> {
    fn dump(&self) -> String {
        self.0.dump()
    }
}

impl<T:?Sized> Clone for RcRefCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}




// impl<T: ?Sized> Deref for Rc<T> {
//     type Target = T;

//     #[inline(always)]
//     fn deref(&self) -> &T {
//         &self.inner().value
//     }
// }


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{RcRefCell, downcast::{Downcast, DowncastRc}};



    trait Animal:Downcast {}


    struct Cat {
        pub ear_count: isize
    }
    impl Animal for RefCell<Cat> {}
    
    struct Dog {
        pub tail_count: isize
    }
    impl Animal for RefCell<Dog> {}


    #[test]
    fn test1() {
        let cat = RcRefCell::new(Cat{ear_count: 1});
        let animal = cat.clone().upcast::<dyn Animal>();
        let cat = animal.downcast_cell::<Cat>();

        assert_eq!(cat.is_some(), true)
    }
}