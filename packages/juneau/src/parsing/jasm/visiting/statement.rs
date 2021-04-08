use crate::core::{Visitor, Visits, concat};
use crate::semantic::{Variable, GetType};
use crate::semantic::jasm::{JasmStatement, JasmType, JasmExpression, Block};
use crate::parsing::{ParseVisitor, parse_name, parse_string};
use crate::parsing::jasm::JasmParseVisitor;
use crate::parsing::jasm::gen::*;


impl<'i> Visitor<&StatementContextAll<'i>, JasmStatement> for JasmParseVisitor {
    fn visit(&mut self, ctx:&StatementContextAll<'i>) -> JasmStatement {
        use JasmStatement::*;
        use StatementContextAll::*;
        match ctx {
            IfStatementContext(ctx) => {
                let (thens, els) = visit_if(self, &ctx.iff.as_ref().unwrap());
                If(thens, els)
            },
            DeclarationStatementContext(ctx) => {
                let typ = self.parse(&ctx.t);
                let name = parse_name(&ctx.name);
                let id = self.new_symbol(&name, typ.to_owned());
                Declaration(Variable {id, name, typ})
            },
            AssignStatementContext(ctx) => {
                let variable = self.parse(&ctx.left);
                let expression = self.parse(&ctx.right);
                if variable.get_type() != expression.get_type() && expression.get_type() != JasmType::Void.into_reference() {
                    println!("VAR {:?}", variable.get_type());
                    println!("EXP {:?}", expression.get_type());
                    println!("VAR2 {:?}", variable);
                    println!("EXPS {:?}", expression);
                    panic!("assign types are not equal")
                } else {
                    Assign(variable, expression)
                }
            }
            WhileStatementContext(ctx) => {
                let condition = self.parse(&ctx.condition);
                let body = self.parse(&ctx.body);
                While(condition, body)
            },
            FunctionStatementContext(ctx) => Function(self.parse(&ctx.func())),
            StructDefinitionStatementContext(ctx) => {
                let name = parse_name(&ctx.name);
                let parameters = self.parses(ctx.parameter_all());
                StructDefinition(self.new_struct(name, parameters))
            },
            ExpressionStatementContext(ctx) => {
                let expression = self.parse(&ctx.expr);
                Expression(expression)
            },
            ReturnStatementContext(ctx) => {
                let expression = self.parse_option(&ctx.expr);
                Return(expression)
            },
            Error(_) => todo!()
        }
    }
}



fn visit_if(visitor:&mut JasmParseVisitor, ctx:&If_statementContextAll) -> (Vec<(JasmExpression, Block)>, Block) {
    use JasmStatement::If;
    let condition = visitor.parse(&ctx.condition);
    let then = visitor.parse(&ctx.then);
    let thens = vec![(condition, then)];
    match &ctx.els_if {
        Some(ctx) => {
            let (thens2, els) = visit_if(visitor, &ctx);
            (concat(thens, thens2), els)
        },
        None => (thens, visitor.parse_or(&ctx.els, Block::default()))
    }
}