use crate::core::Visitor;

mod visitor;
pub use visitor::*;

mod expression;
pub use expression::*;

mod function;
pub use function::*;

mod implementation;
pub use implementation::*;

mod parameter;
pub use parameter::*;

mod statement;
pub use statement::*;

mod typ;
pub use typ::*;

mod value;
pub use value::*;


pub fn render_jasm<'s, S>(jasm:&'s S) -> String
where JasmRenderVisitor: Visitor<&'s S, String>
{
    let mut visitor = JasmRenderVisitor::new();
    visitor.visit(jasm)
}