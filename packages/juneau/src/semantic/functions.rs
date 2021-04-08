use std::{iter::FromIterator, rc::Rc};
use std::collections::HashMap;
use crate::core::Id;
use super::{Name, Function, Semantic, Target};


#[derive(Clone, Debug)]
pub struct Functions<T:Target>(HashMap<Id, T::Function>);



impl<T:Target> Functions<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn get(&self, id:Id, name:&Name) -> T::Function {
        self.get_option(id).expect(&format!("function not found [{:?}] [{:?}]", id, name))
    }
    pub fn get_option(&self, id:Id) -> Option<T::Function> {
        self.0.get(&id).map(|f|f.to_owned())
    }
    pub fn insert(&mut self, id:Id, function:T::Function) {
        self.0.insert(id, function);
    }
    pub fn iter(&self) -> std::vec::IntoIter<T::Function> {
        self.0.iter().map(|(_, f)|f.to_owned()).collect::<Vec<_>>().into_iter()
    }
    pub fn union_self(&mut self, values:&Functions<T>) {
        for (id, function) in &values.0 {
            self.insert(*id, function.to_owned())
        }
    }
    pub fn union(a:&Functions<T>, b:&Functions<T>) -> Functions<T> {
        let mut result = Self::new();
        result.union_self(a);
        result.union_self(b);
        result
    }
}

impl<S:Semantic> Functions<S> {
    pub fn insert_function(&mut self, function:Rc<Function<S>>) {
        self.insert(function.id, function)
    }
}

impl<S:Semantic> From<Vec<Rc<Function<S>>>> for Functions<S> {
    fn from(values:Vec<Rc<Function<S>>>) -> Self {
        let mut functions = Functions::new();
        for function in values {
            functions.insert_function(function.to_owned());
        }
        functions
    }
}

impl<S:Semantic> FromIterator<Rc<Function<S>>> for Functions<S> {
    fn from_iter<I: IntoIterator<Item = Rc<Function<S>>>>(iterator:I) -> Self {
        let mut functions = Functions::new();

        for function in iterator {
            functions.insert_function(function)
        }

        functions
    }
}