use std::cmp::Ordering;


pub trait IteratorExtensions {
    type Item;

    fn sort_by<F:FnMut(&Self::Item, &Self::Item) -> Ordering>(self, compare:F) -> Vec<Self::Item>;
}


impl<I:Iterator> IteratorExtensions for I {
    type Item = I::Item;

    fn sort_by<F:FnMut(&Self::Item, &Self::Item) -> Ordering>(self, compare:F) -> Vec<Self::Item> {
        let mut values = self.collect::<Vec<_>>();
        values.sort_by(compare);
        values
    }
}