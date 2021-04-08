use std::rc::Rc;
use crate::core::Visitor;
use crate::semantic::{Function, Implementation, FunctionType, GetType};
use crate::semantic::jasm::Jasm;
use crate::parsing::{ParseVisitor, parse_name};
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&FuncContextAll<'i>, Rc<Function<Jasm>>> for JasmParseVisitor {
    fn visit(&mut self, ctx:&FuncContextAll<'i>) -> Rc<Function<Jasm>> {
        self.scope.push();
        let name = parse_name(&ctx.name);
        let parameters = self.parses(ctx.parameter_all());
        let retrn_typ = self.parse(&ctx.t);
        let implementation = self.parse(&ctx.implementation);
        let implementation = Implementation::new((implementation, retrn_typ));
        self.scope.pop();
        let return_type = implementation.get_type(); // note this won't support recursion
        let parameter_types = parameters.get_type();
        let function_type = FunctionType::new(parameter_types, return_type);
        let id = self.new_symbol(&name, function_type.into());
        Function::new(id, name, parameters, implementation)
    }
}