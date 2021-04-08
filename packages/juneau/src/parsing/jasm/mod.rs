use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
use std::cell::Ref;
use std::ops::Deref;


use antlr_rust::{common_token_stream::CommonTokenStream, token_stream::TokenStream};
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::InputStream;
use antlr_rust::error_strategy::BailErrorStrategy;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::atn_config_set::ATNConfigSet;
use antlr_rust::dfa::DFA;
use antlr_rust::errors::ANTLRError;
use antlr_rust::parser::Parser;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use crate::core::Visitor;
use crate::semantic::Function;
use crate::semantic::jasm::{Jasm, JasmStatement, JasmExpression, JasmType};
use crate::parsing::ParseVisitor;
use gen::{JasmLexer, JasmParser, JasmParserContextType};


mod gen;

pub mod test;

mod overload;
pub use overload::*;

mod semantic;
pub use semantic::*;

mod visiting;
pub use visiting::*;

mod visitor;
pub use visitor::*;

use self::gen::FuncContextExt;



// impl<'i, I:TokenStream<'i>, H> From<JasmParser<'i,I,H>> for Result<Rc<BaseParserRuleContext<'i, FuncContextExt<'i>>>, ANTLRError> 
// where <I as TokenStream<'i>>::TF : CommonTokenFactory {
//     fn from(_: JasmParser<'i,I,H>) -> Self {
//         todo!()
//     }
// }

pub fn parse_jasm_function(visitor:&mut JasmParseVisitor, source:&str) -> Rc<Function<Jasm>> {
    let token_factory = CommonTokenFactory::default();
    let lexer = JasmLexer::new_with_token_factory(InputStream::new(source.into()), &token_factory);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = JasmParser::with_strategy(token_source, BailErrorStrategy::new());
    let result = parser.func().expect("parsed unsuccessfully");
    let function = visitor.parse(&Some(result));
    function
}


pub fn parse_jasm_statement(visitor:&mut JasmParseVisitor, source:&str) -> JasmStatement {
    let token_factory = CommonTokenFactory::default();
    let lexer = JasmLexer::new_with_token_factory(InputStream::new(source.into()), &token_factory);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = JasmParser::with_strategy(token_source, BailErrorStrategy::new());
    let result = parser.statement().expect("parsed unsuccessfully");
    let statement = visitor.parse(&Some(result));
    statement
}

pub fn parse_jasm_expression(visitor:&mut JasmParseVisitor, source:&str) -> JasmExpression {
    let token_factory = CommonTokenFactory::default();
    let lexer = JasmLexer::new_with_token_factory(InputStream::new(source.into()), &token_factory);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = JasmParser::with_strategy(token_source, BailErrorStrategy::new());
    let result = parser.expression().expect("parsed unsuccessfully");
    let expression = visitor.parse(&Some(result));
    expression
}


// pub fn parse_jasm_expression<'v>(visitor:&mut JasmParseContext<'v>, source:&str) -> JasmExpression {
//     parse_jasm2(visitor, source, |parser|parser.expression())
// }

// pub fn parse_jasm2<'v, S, A:Visitable<'v, JasmParseContext<'v>, S>>(visitor:&mut JasmParseContext<'v>, source:&str, f:impl Fn(JasmParser<CommonTokenStream<JasmLexer<InputStream<&str>>>, BailErrorStrategy<JasmParserContextType>>) -> A) -> S {
//     let token_factory = CommonTokenFactory::default();
//     let lexer = JasmLexer::new_with_token_factory(InputStream::new(source.into()), &token_factory);
//     let token_source = CommonTokenStream::new(lexer);
//     let mut parser = JasmParser::with_strategy(token_source, BailErrorStrategy::new());
//     let result = f(parser);//.expression();
//     visitor.visit(&result)
// }