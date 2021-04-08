use crate::core::Visitor;
use crate::semantic::Parameter;
use crate::semantic::jasm::Jasm;
use crate::parsing::{ParseVisitor, parse_name};
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&ParameterContextAll<'i>, Parameter<Jasm>> for JasmParseVisitor {
    fn visit(&mut self, ctx:&ParameterContextAll<'i>) -> Parameter<Jasm> {
        let name = parse_name(&ctx.name);
        let typ = self.parse(&ctx.t);
        let id = self.new_symbol(&name, typ.to_owned());
        Parameter {id, name, typ}
    }
}