

#[derive(Default, Debug)]
pub struct Stack<T>(Vec<T>);


impl<T> Stack<T> {
    pub fn new()-> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value:T) {
        self.0.push(value)
    }
    pub fn pop(&mut self) -> T {
        self.0.pop().unwrap()
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.0.last_mut().unwrap()
    }
    pub fn get(&self) -> &T {
        self.0.last().unwrap()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn any(&self) -> bool {
        self.len() > 0
    }
    pub fn is_single(&self) -> bool {
        self.len() == 1
    }
    pub fn push_default(&mut self) where T:Default {
        self.push(T::default())
    }
}


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