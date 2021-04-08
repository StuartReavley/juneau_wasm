
pub trait VecSet<T> {
    fn set_insert(&mut self, value:T);
}

impl<T:Eq> VecSet<T> for Vec<T> {
    fn set_insert(&mut self, value:T) {
        if !self.contains(&value) {
            self.push(value);
        }
    }
}