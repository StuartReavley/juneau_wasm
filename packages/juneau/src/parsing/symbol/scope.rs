use crate::semantic::Variable;
use super::{SymbolSemantic, Symbols};


pub struct Scope<S:SymbolSemantic>(pub Vec<Symbols<S>>);

impl<S:SymbolSemantic> Scope<S> {
    pub fn new(globals:Symbols<S>) -> Self {
        Self(vec![globals, Symbols::new()])
    }

    pub fn push(&mut self) {
        self.0.push(Symbols::new());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn insert(&mut self, symbol:Variable<S>) {
        self.0.last_mut().unwrap().insert(symbol);
    }


    pub fn get_from<T>(&self, overload:T) -> &Variable<S>
    where S::Overload : From<T> {
        let overload = S::Overload::from(overload);
        self.get(&overload)
    }
    pub fn get(&self, overload:&S::Overload) -> &Variable<S> {
        for symbols in self.0.iter().rev() {
            if let Some(symbol) = symbols.get(&overload) {
                return symbol;
            }
        }
        panic!("Could not find symbol [{:?}]", overload);
    }
}
