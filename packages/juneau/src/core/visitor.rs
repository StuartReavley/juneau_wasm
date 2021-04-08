

pub trait Visitor<S, T> {
    fn visit(&mut self, s:S) -> T;
    fn visits_into(&mut self, s:Vec<S>) -> Vec<T> {
        s.into_iter().map(|s|self.visit(s)).collect::<Vec<_>>()
    }
}

pub trait Visits<'s, S, T> {
    fn visits(&mut self, s:&'s Vec<S>) -> Vec<T> where Self: Visitor<&'s S, T> {
        s.iter().map(|s|self.visit(s)).collect::<Vec<_>>()
    }
}

impl<'s, S:'s, T, V:Visitor<&'s S, T>> Visits<'s, S, T> for V {}

pub trait VisitorWith<S, U, T> {
    fn visit_with(&mut self, s:S, u:U) -> T;
}