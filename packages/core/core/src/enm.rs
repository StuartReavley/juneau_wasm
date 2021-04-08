

pub trait TypedEnum {
    type Type;
    fn get_type(&self) -> Self::Type;
}