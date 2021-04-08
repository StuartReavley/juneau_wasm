use std::{iter::FromIterator, rc::Rc};
use std::collections::{HashMap, HashSet};
use juneau_core::hash_set;
use crate::core::Id;
use crate::semantic::Target;
use super::{Function, Functions, Semantic};


pub struct Module<T:Target> {
    pub main_id: Option<Id>,
    pub library_ids: HashSet<Id>,
    pub functions: Functions<T>
}

impl<T:Target> Module<T> {
    pub fn new_with_main(main_id:Id, main:T::Function) -> Self {
        Self::new_with_functions(main_id, main, Functions::new())
    }

    pub fn new_with_library(library_ids:HashSet<Id>, functions:Functions<T>) -> Self {
        Self::new(None, library_ids, functions)
    }

    pub fn new_with_functions(main_id:Id, main:T::Function, mut functions:Functions<T>) -> Self {
        functions.insert(main_id, main);
        let library_ids = hash_set!{main_id};
        let main_id = Some(main_id);
        Self::new(main_id, library_ids, functions)
    }

    pub fn new(main_id:Option<Id>, library_ids:HashSet<Id>, functions:Functions<T>) -> Self {
        Self {main_id, library_ids, functions}
    }

    pub fn get_main(&self) -> T::Function {
        self.get_function(self.main_id.unwrap())
    }

    pub fn get_function(&self, id:Id) -> T::Function {
        self.functions.get(id, &"".into())
    }
}

impl<T:Semantic> Module<T> {
    pub fn new_library(functions:Functions<T>) -> Self {
        let library_ids = functions.iter().map(|f|f.id).collect();
         Self::new_with_library(library_ids, functions)
    }
}


// impl<S:Semantic> FromIterator<Rc<Function<S>>> for Module<S> {
//     fn from_iter<I:IntoIterator<Item = Rc<Function<S>>>>(iterator:I) -> Self {
//         let mut functions = Functions::new();

//         for function in iterator {
//             functions.insert_function(function)
//         }

//         functions
//     }
// }
