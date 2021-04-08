use std::collections::HashMap;
use std::iter::FromIterator;
use std::collections::hash_map::Entry::*;
use crate::core::FromRef;
use crate::semantic::{Semantic, Variable};
use super::SymbolSemantic;


#[derive(Clone)]
pub struct Symbols<S:SymbolSemantic>(pub HashMap<S::Overload, Vec<Variable<S>>>);


impl<S:SymbolSemantic> Default for Symbols<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S:SymbolSemantic> Symbols<S> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, symbol:Variable<S>) {
        let overload = S::Overload::from_ref(&symbol);
        match self.0.entry(overload) {
            Occupied(mut entry) => entry.get_mut().push(symbol),
            Vacant(entry) => {
                entry.insert(vec![symbol]);
            }
        };
    }

    pub fn get_from<T>(&self, overload:T) -> Option<&Variable<S>>
    where S::Overload: From<T> {
        let overload = S::Overload::from(overload);
        self.get(&overload)
    }
    pub fn get(&self, overload:&S::Overload) -> Option<&Variable<S>> {
        self.0.get(overload)
            .map(|symbols|match symbols.len() {
                0 => None,
                1 => Some(&symbols[0]),
                _ => panic!("ambiguous symbols")
            })
            .flatten()
    }
}



impl<S:SymbolSemantic> FromIterator<Variable<S>> for Symbols<S> {
    fn from_iter<I:IntoIterator<Item=Variable<S>>>(iter: I) -> Self {
        let mut symbols = Symbols::new();

        for symbol in iter {
            symbols.insert(symbol);
        }

        symbols
    }
}