use std::borrow::Borrow;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;
use crate::core::Visitor;
use crate::semantic::Name;
use antlr_rust::{token::{CommonToken, GenericToken}, token_factory::{CommonTokenFactory, TokenFactory}};
use antlr_rust::token::Token;



pub trait ParseVisitor {

    fn parses<S, T>(&mut self, s:Vec<Rc<S>>) -> Vec<T> where for<'s> Self: Visitor<&'s S, T> {
        s.into_iter().map(|s|self.visit(s.as_ref())).collect()
    }
    fn parse_box<S, T>(&mut self, s:&Option<Rc<S>>) -> Box<T> where for<'s> Self: Visitor<&'s S, T> {
        Box::new(self.parse(s))
    }

    fn parse_or<S, T>(&mut self, s:&Option<Rc<S>>, default:T) -> T where for<'s> Self: Visitor<&'s S, T> {
        s.as_ref().map(|_|self.parse(s)).unwrap_or(default)
    }

    fn parse_option<S, T>(&mut self, s:&Option<Rc<S>>) -> Option<T> where for<'s> Self: Visitor<&'s S, T> {
        s.as_ref().map(|_|self.parse(s))
    }

    fn parse<S, T>(&mut self, s:&Option<Rc<S>>) -> T where for<'s> Self: Visitor<&'s S, T> {
        self.visit(s.as_ref().unwrap())
    }
}



pub fn parse_value<S, T>(ctx:&Option<Box<GenericToken<S>>>) -> T where
    S: Debug + Borrow<str>,
    T: FromStr,    
    <T as FromStr>::Err: Debug {
    ctx.as_ref().unwrap().get_text().parse::<T>().unwrap()
}

pub fn parse_name<S:Debug + Borrow<str>>(ctx:&Option<Box<GenericToken<S>>>) -> Name {
    parse_string(ctx).into()
}

pub fn parse_string<S:Debug + Borrow<str>>(ctx:&Option<Box<GenericToken<S>>>) -> String {
    ctx.as_ref().unwrap().get_text().to_owned()
}





// pub trait StringVisitor<'s, S:'s>: Visitor<&'s S, String> {
//     fn visit_value<F>(&mut self, s:&'s S) -> F where
//     F: FromStr,    
//     <F as FromStr>::Err: Debug {
//         F::from_str(&self.visit(s)).unwrap()
//     }
// }
// // impl<'v, C, Ctx:Visitable<'v, C, String>> StringVisitable<'v, C> for Ctx {}
// impl<'s, S:'s, V:Visitor<&'s S, String>> StringVisitor<'s, S> for V {}

// impl<S:Debug+Borrow<str>, V> Visitor<Option<Box<GenericToken<S>>>, String> for V {



// new...
// impl<'s, S:'s, T, V:VisitReference<S, T>> Visitor<&Rc<S>, T> for V {
//     fn visit(&mut self, s:&Rc<S>) -> T {
//         self.visit(s.as_ref())
//     }
// }

// new
// impl<'s, S:'s, T, V:VisitReference<S, T>> Visitor<&Option<S>, T> for V {
//     fn visit(&mut self, s:&Option<S>) -> T {
//         self.visit(s.as_ref().unwrap())
//     }
// }

// new
// impl<S:Debug+Borrow<str>, V> Visitor<Option<Box<GenericToken<S>>>, String> for V {
//     fn visit(&mut self, s:Option<Box<GenericToken<S>>>) -> String {
//         s.unwrap().get_text().to_owned()
//     }
// }
