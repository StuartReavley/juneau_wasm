use super::Semantic;


pub trait Type<S:Semantic> {
    fn as_function_type(&self) -> Option<&S::FunctionType>;
}

pub trait GetType<T> {
    fn get_type(&self) -> T;
}

impl<T, U:GetType<T>> GetType<Vec<T>> for Vec<U> {
    fn get_type(&self) -> Vec<T> {
        self.iter().map(|t|t.get_type()).collect()
    }
}

impl<T, U:GetType<T>> GetType<T> for Box<U> {
    fn get_type(&self) -> T {
        self.as_ref().get_type()
    }
}