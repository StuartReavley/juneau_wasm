use crate::core::{Visitor, Visits};
use crate::semantic::{Name, GetType, Variable, Parameter, Type};
use crate::semantic::jasm::{JasmExpression, JasmType};
use crate::parsing::{ParseVisitor, parse_name, parse_string};
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&ExpressionContextAll<'i>, JasmExpression> for JasmParseVisitor {
    fn visit(&mut self, ctx:&ExpressionContextAll<'i>) -> JasmExpression {
        use JasmExpression::*;
        use ExpressionContextAll::*;
        match ctx {
            ConstantExpressionContext(ctx) => {
                let value = self.parse(&ctx.v);
                Constant(value)
            }
            ParenExpressionContext(ctx) => self.parse(&ctx.expr),
            UnaryExpressionContext(ctx) => {
                let operator = parse_name(&ctx.operator);
                let expression = self.parse(&ctx.expr);
                visit_invocation(self, operator, vec![expression])
            },
            BinaryExpressionContext(ctx) => {
                let operator = parse_name(&ctx.operator);
                let left = self.parse(&ctx.left);
                let right = self.parse(&ctx.right);
                visit_invocation(self, operator, vec![left, right])
            },
            VariableExpressionContext(ctx) => {
                let name = parse_name(&ctx.name);
                let Variable {id, typ, ..} = self.scope.get_from((&name, None));
                Var(Variable::new(*id, &name, typ))
            }
            StructAccessExpressionContext(ctx) => {
                let object = self.parse_box(&ctx.object);
                let name = parse_name(&ctx.name);
                let typ = object.get_type();
                match typ {
                    JasmType::Struct(struct_id, _) => {
                        let Parameter {id, typ, ..} = self.get_parameter(struct_id, &name);
                        StructAccess(object, *id, name, typ.to_owned())
                    },
                    _ => panic!()
                }
            }
            ArrayAccessExpressionContext(ctx) => {
                let object = self.parse_box(&ctx.left);
                let index = self.parse_box(&ctx.right);
                ArrayAccess(object, index)
            },
            InvocationExpressionContext(ctx) => {
                let name = parse_name(&ctx.function);
                let arguments = self.parses(ctx.expression_all());
                visit_invocation(self, name, arguments)
            },
            CastExpressionContext(ctx) => {
                let expression = self.parse_box(&ctx.expr);
                let typ = self.parse(&ctx.t);
                Cast(expression, typ)
            },
            ReferenceExpressionContext(ctx) => {
                let expression = self.parse_box(&ctx.expr);
                Reference(expression)
            },
            DereferenceExpressionContext(ctx) => {
                let expression = self.parse_box(&ctx.expr);
                Dereference(expression)
            },
            Error(_) => todo!(),
        }
    }
}

pub fn visit_invocation(visitor:&JasmParseVisitor, name:Name, arguments:Vec<JasmExpression>) -> JasmExpression {
    let Variable {id, typ, ..} = visitor.scope.get_from((&name, Some(arguments.get_type())));
    let typ = typ.as_function_type().expect("function doesn't have function type");
    JasmExpression::Invocation(*id, name, arguments, *typ.retrn.to_owned())
}
