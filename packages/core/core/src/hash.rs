use std::{hash::{Hash, Hasher}, io::{Read, Write}};
use std::marker::PhantomData;
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Display};
use std::str::FromStr;
// use rand::random;
use serde::{Serialize, Deserialize};
use crate::{Base64able, Error};
use super::TypedEnum;


#[derive(Serialize, Deserialize)]
pub struct Hsh<S>(u64, PhantomData<S>);


impl<S> Hsh<S> {

    pub fn zero() -> Self {
        Self::new(0)
    }

    // DEPENDENCY NOT COMPATIBLE WITH WEBASSEMBLY -- NOTE WE CAN KEEP RAND crate in carogo.toml, it still compiles...
    // pub fn random() -> Self {
    //     Self::new(random())
    // }

    pub fn new(value:u64) -> Self {
        Self(value, PhantomData)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0u64
    }

    pub fn with_hsher_typed_value(value:&S, f:impl FnMut(&mut Hsher<S>)) -> Self where S:TypedEnum, S::Type:Hash {
        Self::with_hsher_typed(value.get_type(), f)
    }

    pub fn with_hsher_typed(typ:S::Type, mut f:impl FnMut(&mut Hsher<S>)) -> Self where S:TypedEnum, S::Type:Hash {
        Self::with_hsher(|hsher|{
            hsher.h(&typ);
            f(hsher);
        })
    }

    pub fn with_hsher(mut f:impl FnMut(&mut Hsher<S>)) -> Self {
        let mut hsher = Hsher::new();
        f(&mut hsher);
        hsher.finish()
    }
}


impl<S> Debug for Hsh<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_base_64())
    }
}
impl<S> Display for Hsh<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_base_64())
    }
}

impl<S> FromStr for Hsh<S> {
    type Err = Error;

    fn from_str(value:&str) -> Result<Self, Self::Err> {
        Ok(Hsh::new(u64::from_base_64(value.as_bytes())))
    }
}

impl<S> Hash for Hsh<S> {
    fn hash<H: std::hash::Hasher>(&self, state:&mut H) {
        state.write(&self.0.to_be_bytes())
    }
}

impl<S> Eq for Hsh<S> {}
impl<S> PartialEq for Hsh<S> {
    fn eq(&self, other:&Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<S> PartialOrd for Hsh<S> {
    fn partial_cmp(&self, other:&Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<S> Ord for Hsh<S> {
    fn cmp(&self, other:&Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S> Copy for Hsh<S> {}
impl<S> Clone for Hsh<S> {
    fn clone(&self) -> Self {
        Self::new(self.0)
    }
}



pub struct Hsher<S>(DefaultHasher, PhantomData<S>);


impl<S> Hsher<S> {
    pub fn new() -> Self {
        Self(DefaultHasher::new(), PhantomData)
    }

    pub fn finish(&self) -> Hsh<S> {
        Hsh::new(self.0.finish())
    }

    pub fn h<T:Hash>(&mut self, value:&T) {
        value.hash(&mut self.0);
    }
}