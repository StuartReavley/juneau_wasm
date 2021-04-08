use crate::core::Visitor;
use crate::semantic::jasm::JasmType;
use crate::parsing::{ParseVisitor, parse_string};
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&TypContextAll<'i>, JasmType> for JasmParseVisitor {
    fn visit(&mut self, ctx:&TypContextAll<'i>) -> JasmType {
        use JasmType::*;
        use TypContextAll::*;
        match ctx {
            ValueTypeContext(ctx) => {
                let name = parse_string(&ctx.name);
                match name.parse::<JasmType>() {
                    Ok(typ) => typ,
                    Err(_) => self.scope.get_from((&name.into(), None)).typ.to_owned()
                }
            },
            PointerTypeContext(ctx) => self.parse(&ctx.t).into_reference(),
            Error(_) => panic!()
        }
    }
}