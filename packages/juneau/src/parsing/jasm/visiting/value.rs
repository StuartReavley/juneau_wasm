use antlr_rust::token::Token;
use crate::core::Visitor;
use crate::semantic::jasm::JasmValue;
use crate::parsing::parse_value;
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&ValueContextAll<'i>, JasmValue> for JasmParseVisitor {
    fn visit(&mut self, ctx:&ValueContextAll<'i>) -> JasmValue {
        use ValueContextAll::*;
        use JasmValue::*;
        match ctx {
            IntegerValueContext(ctx) => {
                let sign = ctx.sign.is_some();
                let value:i64 = parse_value(&ctx.v);
                I64(if sign {-value} else {value}) // note ideally we could do better than blindly give this an I64 type
            }
            FloatValueContext(ctx) => {
                let sign = ctx.sign.is_some();
                let value:f64 = parse_value(&ctx.v);
                F64(if sign {-value} else {value})
            },
            TrueValueContext(_) => Bool(true),
            FalseValueContext(_) => Bool(false),
            NullValueContext(_) => Null,
            StringValueContext(ctx) => {
                let value = parse_value(&ctx.v);
                String(value)
            },
            Error(_) => todo!(),
        }
    }
}
