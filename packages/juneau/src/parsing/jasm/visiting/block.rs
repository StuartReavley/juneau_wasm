use crate::core::Visitor;
use crate::semantic::jasm::Block;
use crate::parsing::ParseVisitor;
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&BlockContextAll<'i>, Block> for JasmParseVisitor {
    fn visit(&mut self, ctx:&BlockContextAll<'i>) -> Block {
        Block(self.parses(ctx.statement_all()))
    }
}