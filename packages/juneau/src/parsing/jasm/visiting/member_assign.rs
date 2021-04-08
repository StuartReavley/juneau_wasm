use crate::core::{Visitor, VisitorWith};
use crate::core::Id;
use crate::semantic::Parameter;
use crate::semantic::jasm::{JasmExpression, JasmType, Member};
use crate::parsing::{ParseVisitor, parse_name};
use crate::parsing::jasm::{JasmParseVisitor, gen::*};


impl<'i> VisitorWith<&MemberAssignContextAll<'i>, Id, Member> for JasmParseVisitor {
    fn visit_with(&mut self, ctx:&MemberAssignContextAll<'i>, struct_id:Id) -> Member {
        let name = parse_name(&ctx.name);
        let expression = self.parse(&ctx.expr);
        let Parameter {id, ..} = self.get_parameter(struct_id, &name);
        Member {id: *id, name, expression}
    }
}