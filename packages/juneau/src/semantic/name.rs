use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    rc::Rc,
};

pub struct Name(Rc<String>);

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <String as Debug>::fmt(&self.0, f)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <String as Display>::fmt(&self.0, f)
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for Name {}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl PartialEq<&Name> for Name {
    fn eq(&self, other: &&Name) -> bool {
        self.0.eq(&other.0)
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(Rc::new(value.to_owned()))
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}

impl<'n> From<&'n Name> for &'n str {
    fn from(value: &'n Name) -> Self {
        value.0.as_ref()
    }
}

impl<'n> From<&'n Name> for String {
    fn from(value: &'n Name) -> Self {
        value.0.as_str().to_owned()
    }
}
