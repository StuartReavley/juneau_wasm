// Generated from /home/stuart/Documents/juneau.rust/src/parsing/jasm/Jasm.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::token_source::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::jasmlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const WHITESPACE:isize=1; 
		pub const LET:isize=2; 
		pub const IF:isize=3; 
		pub const ELSE:isize=4; 
		pub const TRUE:isize=5; 
		pub const FALSE:isize=6; 
		pub const WHILE:isize=7; 
		pub const NULL:isize=8; 
		pub const FUNCTION:isize=9; 
		pub const STRUCT:isize=10; 
		pub const AS:isize=11; 
		pub const RETURN:isize=12; 
		pub const SHIFTLEFT:isize=13; 
		pub const SHIFTRIGHT:isize=14; 
		pub const AMPERSAND:isize=15; 
		pub const DOT:isize=16; 
		pub const EXCLAMATION:isize=17; 
		pub const PLUS:isize=18; 
		pub const MINUS:isize=19; 
		pub const ASTERISK:isize=20; 
		pub const DIVISION:isize=21; 
		pub const PERCENT:isize=22; 
		pub const ASSIGN:isize=23; 
		pub const EQUAL:isize=24; 
		pub const NOTEQUAL:isize=25; 
		pub const GREATER:isize=26; 
		pub const GREATEREQUAL:isize=27; 
		pub const LESS:isize=28; 
		pub const LESSEQUAL:isize=29; 
		pub const AND:isize=30; 
		pub const OR:isize=31; 
		pub const XOR:isize=32; 
		pub const LEFTPAREN:isize=33; 
		pub const RIGHTPAREN:isize=34; 
		pub const LEFTBRACE:isize=35; 
		pub const RIGHTBRACE:isize=36; 
		pub const LEFTSQUARE:isize=37; 
		pub const RIGHTSQUARE:isize=38; 
		pub const SEMICOLON:isize=39; 
		pub const COMMA:isize=40; 
		pub const COLON:isize=41; 
		pub const QUESTIONMARK:isize=42; 
		pub const DOUBLEQUOTE:isize=43; 
		pub const INTEGER:isize=44; 
		pub const DECIMAL:isize=45; 
		pub const IDENTIFIER:isize=46; 
		pub const TEXT:isize=47;
	pub const RULE_statement:usize = 0; 
	pub const RULE_if_statement:usize = 1; 
	pub const RULE_block:usize = 2; 
	pub const RULE_func:usize = 3; 
	pub const RULE_expression:usize = 4; 
	pub const RULE_value:usize = 5; 
	pub const RULE_parameter:usize = 6; 
	pub const RULE_typ:usize = 7; 
	pub const RULE_memberAssign:usize = 8;
	pub const ruleNames: [&'static str; 9] =  [
		"statement", "if_statement", "block", "func", "expression", "value", "parameter", 
		"typ", "memberAssign"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;43] = [
		None, None, Some("'let'"), Some("'if'"), Some("'else'"), Some("'true'"), 
		Some("'false'"), Some("'while'"), Some("'null'"), Some("'function'"), 
		Some("'struct'"), Some("'as'"), Some("'return'"), Some("'<<'"), Some("'>>'"), 
		Some("'&'"), Some("'.'"), Some("'!'"), Some("'+'"), Some("'-'"), Some("'*'"), 
		Some("'/'"), Some("'%'"), Some("'='"), Some("'=='"), Some("'!='"), Some("'>'"), 
		Some("'>='"), Some("'<'"), Some("'<='"), Some("'and'"), Some("'or'"), 
		Some("'xor'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), 
		Some("']'"), Some("';'"), Some("','"), Some("':'"), Some("'?'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;48]  = [
		None, Some("WHITESPACE"), Some("LET"), Some("IF"), Some("ELSE"), Some("TRUE"), 
		Some("FALSE"), Some("WHILE"), Some("NULL"), Some("FUNCTION"), Some("STRUCT"), 
		Some("AS"), Some("RETURN"), Some("SHIFTLEFT"), Some("SHIFTRIGHT"), Some("AMPERSAND"), 
		Some("DOT"), Some("EXCLAMATION"), Some("PLUS"), Some("MINUS"), Some("ASTERISK"), 
		Some("DIVISION"), Some("PERCENT"), Some("ASSIGN"), Some("EQUAL"), Some("NOTEQUAL"), 
		Some("GREATER"), Some("GREATEREQUAL"), Some("LESS"), Some("LESSEQUAL"), 
		Some("AND"), Some("OR"), Some("XOR"), Some("LEFTPAREN"), Some("RIGHTPAREN"), 
		Some("LEFTBRACE"), Some("RIGHTBRACE"), Some("LEFTSQUARE"), Some("RIGHTSQUARE"), 
		Some("SEMICOLON"), Some("COMMA"), Some("COLON"), Some("QUESTIONMARK"), 
		Some("DOUBLEQUOTE"), Some("INTEGER"), Some("DECIMAL"), Some("IDENTIFIER"), 
		Some("TEXT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,JasmParserExt, I, JasmParserContextType , dyn JasmListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type JasmTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, JasmParserContextType , dyn JasmListener<'input> + 'a>;

/// Parser for Jasm grammar
pub struct JasmParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				JasmParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> JasmParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> JasmParser<'input, I, DefaultErrorStrategy<'input,JasmParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for JasmParser
pub trait JasmParserContext<'input>:
	for<'x> Listenable<dyn JasmListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=JasmParserContextType>
{}

impl<'input> JasmParserContext<'input> for TerminalNode<'input,JasmParserContextType> {}
impl<'input> JasmParserContext<'input> for ErrorNode<'input,JasmParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn JasmParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn JasmListener<'input> + 'input{}

pub struct JasmParserContextType;
antlr_rust::type_id!{JasmParserContextType}

impl<'input> ParserNodeType<'input> for JasmParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn JasmParserContext<'input> + 'input;
	  type Visitor = dyn ParseTreeVisitor<'input, Self> + 'input;
}

impl<'input, I, H> Deref for JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct JasmParserExt{
}

impl JasmParserExt{
}


impl<'input> TokenAware<'input> for JasmParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for JasmParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for JasmParserExt{
	fn get_grammar_file_name(&self) -> & str{ "Jasm.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn JasmParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					4 => JasmParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> JasmParser<'input, I, DefaultErrorStrategy<'input,JasmParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 5)
				}
				1=>{
					recog.precpred(None, 4)
				}
				2=>{
					recog.precpred(None, 3)
				}
				3=>{
					recog.precpred(None, 2)
				}
				4=>{
					recog.precpred(None, 12)
				}
				5=>{
					recog.precpred(None, 8)
				}
				6=>{
					recog.precpred(None, 6)
				}
			_ => true
		}
	}
}
//------------------- statement ----------------
#[derive(Debug)]
pub enum StatementContextAll<'input>{
	WhileStatementContext(WhileStatementContext<'input>),
	StructDefinitionStatementContext(StructDefinitionStatementContext<'input>),
	DeclarationStatementContext(DeclarationStatementContext<'input>),
	FunctionStatementContext(FunctionStatementContext<'input>),
	AssignStatementContext(AssignStatementContext<'input>),
	ExpressionStatementContext(ExpressionStatementContext<'input>),
	IfStatementContext(IfStatementContext<'input>),
	ReturnStatementContext(ReturnStatementContext<'input>),
Error(StatementContext<'input>)
}
antlr_rust::type_id!{StatementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for StatementContextAll<'input>{}

impl<'input> JasmParserContext<'input> for StatementContextAll<'input>{}

impl<'input> Deref for StatementContextAll<'input>{
	type Target = dyn StatementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StatementContextAll::*;
		match self{
			WhileStatementContext(inner) => inner,
			StructDefinitionStatementContext(inner) => inner,
			DeclarationStatementContext(inner) => inner,
			FunctionStatementContext(inner) => inner,
			AssignStatementContext(inner) => inner,
			ExpressionStatementContext(inner) => inner,
			IfStatementContext(inner) => inner,
			ReturnStatementContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for StatementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for StatementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::type_id!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
		StatementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StatementContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<StatementContextExt<'input>>{


}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

pub type WhileStatementContext<'input> = BaseParserRuleContext<'input,WhileStatementContextExt<'input>>;

pub trait WhileStatementContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token WHILE
	/// Returns `None` if there is no child corresponding to token WHILE
	fn WHILE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(WHILE, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WhileStatementContextAttrs<'input> for WhileStatementContext<'input>{}

pub struct WhileStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub condition: Option<Rc<ExpressionContextAll<'input>>>,
	pub body: Option<Rc<BlockContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{WhileStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for WhileStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for WhileStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_whileStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhileStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for WhileStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for WhileStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for WhileStatementContext<'input> {}

impl<'input> WhileStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::WhileStatementContext(
				BaseParserRuleContext::copy_from(ctx,WhileStatementContextExt{
        			condition:None, body:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructDefinitionStatementContext<'input> = BaseParserRuleContext<'input,StructDefinitionStatementContextExt<'input>>;

pub trait StructDefinitionStatementContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRUCT
	/// Returns `None` if there is no child corresponding to token STRUCT
	fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(STRUCT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTBRACE
	/// Returns `None` if there is no child corresponding to token LEFTBRACE
	fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LEFTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
	/// Returns `None` if there is no child corresponding to token RIGHTBRACE
	fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(RIGHTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
	fn parameter_all(&self) ->  Vec<Rc<ParameterContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn parameter(&self, i: usize) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JasmParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> StructDefinitionStatementContextAttrs<'input> for StructDefinitionStatementContext<'input>{}

pub struct StructDefinitionStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub name: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{StructDefinitionStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for StructDefinitionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for StructDefinitionStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_structDefinitionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructDefinitionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for StructDefinitionStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for StructDefinitionStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for StructDefinitionStatementContext<'input> {}

impl<'input> StructDefinitionStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::StructDefinitionStatementContext(
				BaseParserRuleContext::copy_from(ctx,StructDefinitionStatementContextExt{
					name:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DeclarationStatementContext<'input> = BaseParserRuleContext<'input,DeclarationStatementContextExt<'input>>;

pub trait DeclarationStatementContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LET
	/// Returns `None` if there is no child corresponding to token LET
	fn LET(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COLON
	/// Returns `None` if there is no child corresponding to token COLON
	fn COLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(COLON, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
	fn typ(&self) -> Option<Rc<TypContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DeclarationStatementContextAttrs<'input> for DeclarationStatementContext<'input>{}

pub struct DeclarationStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub name: Option<TokenType<'input>>,
	pub t: Option<Rc<TypContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{DeclarationStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for DeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for DeclarationStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for DeclarationStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for DeclarationStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for DeclarationStatementContext<'input> {}

impl<'input> DeclarationStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::DeclarationStatementContext(
				BaseParserRuleContext::copy_from(ctx,DeclarationStatementContextExt{
					name:None, 
        			t:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FunctionStatementContext<'input> = BaseParserRuleContext<'input,FunctionStatementContextExt<'input>>;

pub trait FunctionStatementContextAttrs<'input>: JasmParserContext<'input>{
	fn func(&self) -> Option<Rc<FuncContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> FunctionStatementContextAttrs<'input> for FunctionStatementContext<'input>{}

pub struct FunctionStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{FunctionStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for FunctionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for FunctionStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for FunctionStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for FunctionStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for FunctionStatementContext<'input> {}

impl<'input> FunctionStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::FunctionStatementContext(
				BaseParserRuleContext::copy_from(ctx,FunctionStatementContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AssignStatementContext<'input> = BaseParserRuleContext<'input,AssignStatementContextExt<'input>>;

pub trait AssignStatementContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(ASSIGN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> AssignStatementContextAttrs<'input> for AssignStatementContext<'input>{}

pub struct AssignStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub left: Option<Rc<ExpressionContextAll<'input>>>,
	pub right: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{AssignStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for AssignStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for AssignStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assignStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for AssignStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for AssignStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for AssignStatementContext<'input> {}

impl<'input> AssignStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::AssignStatementContext(
				BaseParserRuleContext::copy_from(ctx,AssignStatementContextExt{
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionStatementContext<'input> = BaseParserRuleContext<'input,ExpressionStatementContextExt<'input>>;

pub trait ExpressionStatementContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpressionStatementContextAttrs<'input> for ExpressionStatementContext<'input>{}

pub struct ExpressionStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ExpressionStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for ExpressionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ExpressionStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for ExpressionStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for ExpressionStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for ExpressionStatementContext<'input> {}

impl<'input> ExpressionStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::ExpressionStatementContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionStatementContextExt{
        			expr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IfStatementContext<'input> = BaseParserRuleContext<'input,IfStatementContextExt<'input>>;

pub trait IfStatementContextAttrs<'input>: JasmParserContext<'input>{
	fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IfStatementContextAttrs<'input> for IfStatementContext<'input>{}

pub struct IfStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub iff: Option<Rc<If_statementContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{IfStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for IfStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for IfStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ifStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for IfStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for IfStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for IfStatementContext<'input> {}

impl<'input> IfStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::IfStatementContext(
				BaseParserRuleContext::copy_from(ctx,IfStatementContextExt{
        			iff:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ReturnStatementContext<'input> = BaseParserRuleContext<'input,ReturnStatementContextExt<'input>>;

pub trait ReturnStatementContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token RETURN
	/// Returns `None` if there is no child corresponding to token RETURN
	fn RETURN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(RETURN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ReturnStatementContextAttrs<'input> for ReturnStatementContext<'input>{}

pub struct ReturnStatementContextExt<'input>{
	base:StatementContextExt<'input>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ReturnStatementContextExt<'a>}

impl<'input> JasmParserContext<'input> for ReturnStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ReturnStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_returnStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for ReturnStatementContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for ReturnStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for ReturnStatementContext<'input> {}

impl<'input> ReturnStatementContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::ReturnStatementContext(
				BaseParserRuleContext::copy_from(ctx,ReturnStatementContextExt{
        			expr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(57);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				1 =>{
					let tmp = ExpressionStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule expression*/
					recog.base.set_state(18);
					let tmp = recog.expression_rec(0)?;
					if let StatementContextAll::ExpressionStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.expr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(19);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = IfStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule if_statement*/
					recog.base.set_state(21);
					let tmp = recog.if_statement()?;
					if let StatementContextAll::IfStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.iff = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				3 =>{
					let tmp = WhileStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(22);
					recog.base.match_token(WHILE,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(23);
					let tmp = recog.expression_rec(0)?;
					if let StatementContextAll::WhileStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.condition = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					/*InvokeRule block*/
					recog.base.set_state(24);
					let tmp = recog.block()?;
					if let StatementContextAll::WhileStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.body = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				4 =>{
					let tmp = FunctionStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					/*InvokeRule func*/
					recog.base.set_state(26);
					recog.func()?;

					}
				}
			,
				5 =>{
					let tmp = StructDefinitionStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(27);
					recog.base.match_token(STRUCT,&mut recog.err_handler)?;

					recog.base.set_state(28);
					let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
					if let StatementContextAll::StructDefinitionStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(29);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(38);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IDENTIFIER {
						{
						/*InvokeRule parameter*/
						recog.base.set_state(30);
						recog.parameter()?;

						recog.base.set_state(35);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(31);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule parameter*/
							recog.base.set_state(32);
							recog.parameter()?;

							}
							}
							recog.base.set_state(37);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(40);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = DeclarationStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(41);
					recog.base.match_token(LET,&mut recog.err_handler)?;

					recog.base.set_state(42);
					let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
					if let StatementContextAll::DeclarationStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(43);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule typ*/
					recog.base.set_state(44);
					let tmp = recog.typ()?;
					if let StatementContextAll::DeclarationStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.t = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(45);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = AssignStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					/*InvokeRule expression*/
					recog.base.set_state(47);
					let tmp = recog.expression_rec(0)?;
					if let StatementContextAll::AssignStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.left = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(48);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(49);
					let tmp = recog.expression_rec(0)?;
					if let StatementContextAll::AssignStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
					ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(50);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = ReturnStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(52);
					recog.base.match_token(RETURN,&mut recog.err_handler)?;

					recog.base.set_state(54);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << TRUE) | (1usize << FALSE) | (1usize << NULL) | (1usize << AMPERSAND) | (1usize << EXCLAMATION) | (1usize << MINUS) | (1usize << ASTERISK) | (1usize << LEFTPAREN) | (1usize << DOUBLEQUOTE) | (1usize << INTEGER) | (1usize << DECIMAL) | (1usize << IDENTIFIER))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(53);
						let tmp = recog.expression_rec(0)?;
						if let StatementContextAll::ReturnStatementContext(ctx) = cast_mut::<_,StatementContextAll >(&mut _localctx){
						ctx.expr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(56);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- if_statement ----------------
pub type If_statementContextAll<'input> = If_statementContext<'input>;


pub type If_statementContext<'input> = BaseParserRuleContext<'input,If_statementContextExt<'input>>;

#[derive(Clone)]
pub struct If_statementContextExt<'input>{
	pub condition: Option<Rc<ExpressionContextAll<'input>>>,
	pub then: Option<Rc<BlockContextAll<'input>>>,
	pub els_if: Option<Rc<If_statementContextAll<'input>>>,
	pub els: Option<Rc<BlockContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for If_statementContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for If_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_if_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for If_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_statement }
}
antlr_rust::type_id!{If_statementContextExt<'a>}

impl<'input> If_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_statementContextExt{
				condition: None, then: None, els_if: None, els: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait If_statementContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<If_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block_all(&self) ->  Vec<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}
fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> If_statementContextAttrs<'input> for If_statementContext<'input>{}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_statement(&mut self,)
	-> Result<Rc<If_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_if_statement);
        let mut _localctx: Rc<If_statementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(59);
			recog.base.match_token(IF,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(60);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,If_statementContext >(&mut _localctx).condition = Some(tmp.clone());
			  

			/*InvokeRule block*/
			recog.base.set_state(61);
			let tmp = recog.block()?;
			 cast_mut::<_,If_statementContext >(&mut _localctx).then = Some(tmp.clone());
			  

			recog.base.set_state(67);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ELSE {
				{
				recog.base.set_state(62);
				recog.base.match_token(ELSE,&mut recog.err_handler)?;

				recog.base.set_state(65);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 IF 
					=> {
						{
						/*InvokeRule if_statement*/
						recog.base.set_state(63);
						let tmp = recog.if_statement()?;
						 cast_mut::<_,If_statementContext >(&mut _localctx).els_if = Some(tmp.clone());
						  

						}
					}

				 LEFTBRACE 
					=> {
						{
						{
						/*InvokeRule block*/
						recog.base.set_state(64);
						let tmp = recog.block()?;
						 cast_mut::<_,If_statementContext >(&mut _localctx).els = Some(tmp.clone());
						  

						}
						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for BlockContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::type_id!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(69);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(73);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LET) | (1usize << IF) | (1usize << TRUE) | (1usize << FALSE) | (1usize << WHILE) | (1usize << NULL) | (1usize << FUNCTION) | (1usize << STRUCT) | (1usize << RETURN) | (1usize << AMPERSAND) | (1usize << EXCLAMATION) | (1usize << MINUS) | (1usize << ASTERISK) | (1usize << LEFTPAREN) | (1usize << DOUBLEQUOTE) | (1usize << INTEGER) | (1usize << DECIMAL) | (1usize << IDENTIFIER))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(70);
				recog.statement()?;

				}
				}
				recog.base.set_state(75);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(76);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func ----------------
pub type FuncContextAll<'input> = FuncContext<'input>;


pub type FuncContext<'input> = BaseParserRuleContext<'input,FuncContextExt<'input>>;

#[derive(Clone)]
pub struct FuncContextExt<'input>{
	pub name: Option<TokenType<'input>>,
	pub t: Option<Rc<TypContextAll<'input>>>,
	pub implementation: Option<Rc<BlockContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for FuncContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for FuncContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_func(self);
	}
}

impl<'input> CustomRuleContext<'input> for FuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}
antlr_rust::type_id!{FuncContextExt<'a>}

impl<'input> FuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncContextExt{
				name: None, 
				t: None, implementation: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait FuncContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<FuncContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNCTION
/// Returns `None` if there is no child corresponding to token FUNCTION
fn FUNCTION(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(FUNCTION, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTPAREN
/// Returns `None` if there is no child corresponding to token LEFTPAREN
fn LEFTPAREN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(LEFTPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTPAREN
/// Returns `None` if there is no child corresponding to token RIGHTPAREN
fn RIGHTPAREN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(RIGHTPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn typ(&self) -> Option<Rc<TypContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameter_all(&self) ->  Vec<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameter(&self, i: usize) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JasmParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FuncContextAttrs<'input> for FuncContext<'input>{}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func(&mut self,)
	-> Result<Rc<FuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_func);
        let mut _localctx: Rc<FuncContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(78);
			recog.base.match_token(FUNCTION,&mut recog.err_handler)?;

			recog.base.set_state(79);
			let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
			 cast_mut::<_,FuncContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(80);
			recog.base.match_token(LEFTPAREN,&mut recog.err_handler)?;

			recog.base.set_state(89);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule parameter*/
				recog.base.set_state(81);
				recog.parameter()?;

				recog.base.set_state(86);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(82);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule parameter*/
					recog.base.set_state(83);
					recog.parameter()?;

					}
					}
					recog.base.set_state(88);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(91);
			recog.base.match_token(RIGHTPAREN,&mut recog.err_handler)?;

			recog.base.set_state(92);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule typ*/
			recog.base.set_state(93);
			let tmp = recog.typ()?;
			 cast_mut::<_,FuncContext >(&mut _localctx).t = Some(tmp.clone());
			  

			/*InvokeRule block*/
			recog.base.set_state(94);
			let tmp = recog.block()?;
			 cast_mut::<_,FuncContext >(&mut _localctx).implementation = Some(tmp.clone());
			  

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input>{
	BinaryExpressionContext(BinaryExpressionContext<'input>),
	CastExpressionContext(CastExpressionContext<'input>),
	ArrayAccessExpressionContext(ArrayAccessExpressionContext<'input>),
	ConstantExpressionContext(ConstantExpressionContext<'input>),
	ReferenceExpressionContext(ReferenceExpressionContext<'input>),
	InvocationExpressionContext(InvocationExpressionContext<'input>),
	StructAccessExpressionContext(StructAccessExpressionContext<'input>),
	ParenExpressionContext(ParenExpressionContext<'input>),
	UnaryExpressionContext(UnaryExpressionContext<'input>),
	DereferenceExpressionContext(DereferenceExpressionContext<'input>),
	VariableExpressionContext(VariableExpressionContext<'input>),
Error(ExpressionContext<'input>)
}
antlr_rust::type_id!{ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input>{}

impl<'input> JasmParserContext<'input> for ExpressionContextAll<'input>{}

impl<'input> Deref for ExpressionContextAll<'input>{
	type Target = dyn ExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExpressionContextAll::*;
		match self{
			BinaryExpressionContext(inner) => inner,
			CastExpressionContext(inner) => inner,
			ArrayAccessExpressionContext(inner) => inner,
			ConstantExpressionContext(inner) => inner,
			ReferenceExpressionContext(inner) => inner,
			InvocationExpressionContext(inner) => inner,
			StructAccessExpressionContext(inner) => inner,
			ParenExpressionContext(inner) => inner,
			UnaryExpressionContext(inner) => inner,
			DereferenceExpressionContext(inner) => inner,
			VariableExpressionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::type_id!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
		ExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExpressionContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{


}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

pub type BinaryExpressionContext<'input> = BaseParserRuleContext<'input,BinaryExpressionContextExt<'input>>;

pub trait BinaryExpressionContextAttrs<'input>: JasmParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token DIVISION
	/// Returns `None` if there is no child corresponding to token DIVISION
	fn DIVISION(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(DIVISION, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASTERISK
	/// Returns `None` if there is no child corresponding to token ASTERISK
	fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(ASTERISK, 0)
	}
	/// Retrieves first TerminalNode corresponding to token PERCENT
	/// Returns `None` if there is no child corresponding to token PERCENT
	fn PERCENT(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(PERCENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token PLUS
	/// Returns `None` if there is no child corresponding to token PLUS
	fn PLUS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(PLUS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EQUAL
	/// Returns `None` if there is no child corresponding to token EQUAL
	fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(EQUAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token NOTEQUAL
	/// Returns `None` if there is no child corresponding to token NOTEQUAL
	fn NOTEQUAL(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(NOTEQUAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token GREATER
	/// Returns `None` if there is no child corresponding to token GREATER
	fn GREATER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(GREATER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token GREATEREQUAL
	/// Returns `None` if there is no child corresponding to token GREATEREQUAL
	fn GREATEREQUAL(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(GREATEREQUAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LESS
	/// Returns `None` if there is no child corresponding to token LESS
	fn LESS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LESS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LESSEQUAL
	/// Returns `None` if there is no child corresponding to token LESSEQUAL
	fn LESSEQUAL(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LESSEQUAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token AND
	/// Returns `None` if there is no child corresponding to token AND
	fn AND(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(AND, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OR
	/// Returns `None` if there is no child corresponding to token OR
	fn OR(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(OR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token XOR
	/// Returns `None` if there is no child corresponding to token XOR
	fn XOR(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(XOR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SHIFTLEFT
	/// Returns `None` if there is no child corresponding to token SHIFTLEFT
	fn SHIFTLEFT(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(SHIFTLEFT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SHIFTRIGHT
	/// Returns `None` if there is no child corresponding to token SHIFTRIGHT
	fn SHIFTRIGHT(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(SHIFTRIGHT, 0)
	}
}

impl<'input> BinaryExpressionContextAttrs<'input> for BinaryExpressionContext<'input>{}

pub struct BinaryExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub left: Option<Rc<ExpressionContextAll<'input>>>,
	pub operator: Option<TokenType<'input>>,
	pub right: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{BinaryExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for BinaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for BinaryExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_binaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BinaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for BinaryExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for BinaryExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for BinaryExpressionContext<'input> {}

impl<'input> BinaryExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::BinaryExpressionContext(
				BaseParserRuleContext::copy_from(ctx,BinaryExpressionContextExt{
					operator:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CastExpressionContext<'input> = BaseParserRuleContext<'input,CastExpressionContextExt<'input>>;

pub trait CastExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token AS
	/// Returns `None` if there is no child corresponding to token AS
	fn AS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(AS, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn typ(&self) -> Option<Rc<TypContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CastExpressionContextAttrs<'input> for CastExpressionContext<'input>{}

pub struct CastExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	pub t: Option<Rc<TypContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{CastExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for CastExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for CastExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_castExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for CastExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for CastExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for CastExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for CastExpressionContext<'input> {}

impl<'input> CastExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::CastExpressionContext(
				BaseParserRuleContext::copy_from(ctx,CastExpressionContextExt{
        			expr:None, t:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrayAccessExpressionContext<'input> = BaseParserRuleContext<'input,ArrayAccessExpressionContextExt<'input>>;

pub trait ArrayAccessExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LEFTSQUARE
	/// Returns `None` if there is no child corresponding to token LEFTSQUARE
	fn LEFTSQUARE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LEFTSQUARE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTSQUARE
	/// Returns `None` if there is no child corresponding to token RIGHTSQUARE
	fn RIGHTSQUARE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(RIGHTSQUARE, 0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ArrayAccessExpressionContextAttrs<'input> for ArrayAccessExpressionContext<'input>{}

pub struct ArrayAccessExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub left: Option<Rc<ExpressionContextAll<'input>>>,
	pub right: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ArrayAccessExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for ArrayAccessExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ArrayAccessExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_arrayAccessExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayAccessExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ArrayAccessExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ArrayAccessExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ArrayAccessExpressionContext<'input> {}

impl<'input> ArrayAccessExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ArrayAccessExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ArrayAccessExpressionContextExt{
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConstantExpressionContext<'input> = BaseParserRuleContext<'input,ConstantExpressionContextExt<'input>>;

pub trait ConstantExpressionContextAttrs<'input>: JasmParserContext<'input>{
	fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ConstantExpressionContextAttrs<'input> for ConstantExpressionContext<'input>{}

pub struct ConstantExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub v: Option<Rc<ValueContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ConstantExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for ConstantExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ConstantExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_constantExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ConstantExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ConstantExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ConstantExpressionContext<'input> {}

impl<'input> ConstantExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ConstantExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ConstantExpressionContextExt{
        			v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ReferenceExpressionContext<'input> = BaseParserRuleContext<'input,ReferenceExpressionContextExt<'input>>;

pub trait ReferenceExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token AMPERSAND
	/// Returns `None` if there is no child corresponding to token AMPERSAND
	fn AMPERSAND(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(AMPERSAND, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ReferenceExpressionContextAttrs<'input> for ReferenceExpressionContext<'input>{}

pub struct ReferenceExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ReferenceExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for ReferenceExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ReferenceExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_referenceExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReferenceExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ReferenceExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ReferenceExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ReferenceExpressionContext<'input> {}

impl<'input> ReferenceExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ReferenceExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ReferenceExpressionContextExt{
        			expr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type InvocationExpressionContext<'input> = BaseParserRuleContext<'input,InvocationExpressionContextExt<'input>>;

pub trait InvocationExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LEFTPAREN
	/// Returns `None` if there is no child corresponding to token LEFTPAREN
	fn LEFTPAREN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LEFTPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTPAREN
	/// Returns `None` if there is no child corresponding to token RIGHTPAREN
	fn RIGHTPAREN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(RIGHTPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JasmParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> InvocationExpressionContextAttrs<'input> for InvocationExpressionContext<'input>{}

pub struct InvocationExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub function: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{InvocationExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for InvocationExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for InvocationExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_invocationExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for InvocationExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for InvocationExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for InvocationExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for InvocationExpressionContext<'input> {}

impl<'input> InvocationExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::InvocationExpressionContext(
				BaseParserRuleContext::copy_from(ctx,InvocationExpressionContextExt{
					function:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructAccessExpressionContext<'input> = BaseParserRuleContext<'input,StructAccessExpressionContextExt<'input>>;

pub trait StructAccessExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DOT
	/// Returns `None` if there is no child corresponding to token DOT
	fn DOT(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(DOT, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
}

impl<'input> StructAccessExpressionContextAttrs<'input> for StructAccessExpressionContext<'input>{}

pub struct StructAccessExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub object: Option<Rc<ExpressionContextAll<'input>>>,
	pub name: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{StructAccessExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for StructAccessExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for StructAccessExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_structAccessExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructAccessExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for StructAccessExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for StructAccessExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for StructAccessExpressionContext<'input> {}

impl<'input> StructAccessExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::StructAccessExpressionContext(
				BaseParserRuleContext::copy_from(ctx,StructAccessExpressionContextExt{
					name:None, 
        			object:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenExpressionContext<'input> = BaseParserRuleContext<'input,ParenExpressionContextExt<'input>>;

pub trait ParenExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LEFTPAREN
	/// Returns `None` if there is no child corresponding to token LEFTPAREN
	fn LEFTPAREN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(LEFTPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTPAREN
	/// Returns `None` if there is no child corresponding to token RIGHTPAREN
	fn RIGHTPAREN(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(RIGHTPAREN, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ParenExpressionContextAttrs<'input> for ParenExpressionContext<'input>{}

pub struct ParenExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ParenExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for ParenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ParenExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ParenExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ParenExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ParenExpressionContext<'input> {}

impl<'input> ParenExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ParenExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ParenExpressionContextExt{
        			expr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

pub trait UnaryExpressionContextAttrs<'input>: JasmParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EXCLAMATION
	/// Returns `None` if there is no child corresponding to token EXCLAMATION
	fn EXCLAMATION(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(EXCLAMATION, 0)
	}
}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

pub struct UnaryExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub operator: Option<TokenType<'input>>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{UnaryExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for UnaryExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for UnaryExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for UnaryExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for UnaryExpressionContext<'input> {}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::UnaryExpressionContext(
				BaseParserRuleContext::copy_from(ctx,UnaryExpressionContextExt{
					operator:None, 
        			expr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DereferenceExpressionContext<'input> = BaseParserRuleContext<'input,DereferenceExpressionContextExt<'input>>;

pub trait DereferenceExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ASTERISK
	/// Returns `None` if there is no child corresponding to token ASTERISK
	fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(ASTERISK, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DereferenceExpressionContextAttrs<'input> for DereferenceExpressionContext<'input>{}

pub struct DereferenceExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{DereferenceExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for DereferenceExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for DereferenceExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dereferenceExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for DereferenceExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for DereferenceExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for DereferenceExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for DereferenceExpressionContext<'input> {}

impl<'input> DereferenceExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::DereferenceExpressionContext(
				BaseParserRuleContext::copy_from(ctx,DereferenceExpressionContextExt{
        			expr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VariableExpressionContext<'input> = BaseParserRuleContext<'input,VariableExpressionContextExt<'input>>;

pub trait VariableExpressionContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
}

impl<'input> VariableExpressionContextAttrs<'input> for VariableExpressionContext<'input>{}

pub struct VariableExpressionContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub name: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{VariableExpressionContextExt<'a>}

impl<'input> JasmParserContext<'input> for VariableExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for VariableExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variableExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for VariableExpressionContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for VariableExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for VariableExpressionContext<'input> {}

impl<'input> VariableExpressionContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::VariableExpressionContext(
				BaseParserRuleContext::copy_from(ctx,VariableExpressionContextExt{
					name:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 8, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 8;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(122);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = ConstantExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					/*InvokeRule value*/
					recog.base.set_state(97);
					let tmp = recog.value()?;
					if let ExpressionContextAll::ConstantExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				2 =>{
					{
					let mut tmp = ParenExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(98);
					recog.base.match_token(LEFTPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(99);
					let tmp = recog.expression_rec(0)?;
					if let ExpressionContextAll::ParenExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.expr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(100);
					recog.base.match_token(RIGHTPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = ReferenceExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(102);
					recog.base.match_token(AMPERSAND,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(103);
					let tmp = recog.expression_rec(11)?;
					if let ExpressionContextAll::ReferenceExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.expr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				4 =>{
					{
					let mut tmp = DereferenceExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(104);
					recog.base.match_token(ASTERISK,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(105);
					let tmp = recog.expression_rec(10)?;
					if let ExpressionContextAll::DereferenceExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.expr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				5 =>{
					{
					let mut tmp = InvocationExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(106);
					let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
					if let ExpressionContextAll::InvocationExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.function = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(107);
					recog.base.match_token(LEFTPAREN,&mut recog.err_handler)?;

					recog.base.set_state(116);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << TRUE) | (1usize << FALSE) | (1usize << NULL) | (1usize << AMPERSAND) | (1usize << EXCLAMATION) | (1usize << MINUS) | (1usize << ASTERISK) | (1usize << LEFTPAREN) | (1usize << DOUBLEQUOTE) | (1usize << INTEGER) | (1usize << DECIMAL) | (1usize << IDENTIFIER))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(108);
						recog.expression_rec(0)?;

						recog.base.set_state(113);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(109);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(110);
							recog.expression_rec(0)?;

							}
							}
							recog.base.set_state(115);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(118);
					recog.base.match_token(RIGHTPAREN,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = UnaryExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(119);
					if let ExpressionContextAll::UnaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(_la==EXCLAMATION || _la==MINUS) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let ExpressionContextAll::UnaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
						ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(120);
					let tmp = recog.expression_rec(7)?;
					if let ExpressionContextAll::UnaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.expr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				7 =>{
					{
					let mut tmp = VariableExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(121);
					let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
					if let ExpressionContextAll::VariableExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(149);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(147);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BinaryExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(124);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(125);
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ASTERISK) | (1usize << DIVISION) | (1usize << PERCENT))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(126);
							let tmp = recog.expression_rec(6)?;
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BinaryExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(127);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(128);
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==PLUS || _la==MINUS) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(129);
							let tmp = recog.expression_rec(5)?;
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BinaryExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(130);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(131);
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << EQUAL) | (1usize << NOTEQUAL) | (1usize << GREATER) | (1usize << GREATEREQUAL) | (1usize << LESS) | (1usize << LESSEQUAL))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(132);
							let tmp = recog.expression_rec(4)?;
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = BinaryExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(133);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(134);
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.operator = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << SHIFTLEFT) | (1usize << SHIFTRIGHT) | (1usize << AND) | (1usize << OR) | (1usize << XOR))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.operator = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(135);
							let tmp = recog.expression_rec(3)?;
							if let ExpressionContextAll::BinaryExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = StructAccessExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::StructAccessExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.object = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(136);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							recog.base.set_state(137);
							recog.base.match_token(DOT,&mut recog.err_handler)?;

							recog.base.set_state(138);
							let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
							if let ExpressionContextAll::StructAccessExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						6 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ArrayAccessExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::ArrayAccessExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(139);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(140);
							recog.base.match_token(LEFTSQUARE,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(141);
							let tmp = recog.expression_rec(0)?;
							if let ExpressionContextAll::ArrayAccessExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							recog.base.set_state(142);
							recog.base.match_token(RIGHTSQUARE,&mut recog.err_handler)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = CastExpressionContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							if let ExpressionContextAll::CastExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut tmp){
								ctx.expr = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(144);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(145);
							recog.base.match_token(AS,&mut recog.err_handler)?;

							/*InvokeRule typ*/
							recog.base.set_state(146);
							let tmp = recog.typ()?;
							if let ExpressionContextAll::CastExpressionContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.t = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(151);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- value ----------------
#[derive(Debug)]
pub enum ValueContextAll<'input>{
	FalseValueContext(FalseValueContext<'input>),
	StringValueContext(StringValueContext<'input>),
	TrueValueContext(TrueValueContext<'input>),
	FloatValueContext(FloatValueContext<'input>),
	IntegerValueContext(IntegerValueContext<'input>),
	NullValueContext(NullValueContext<'input>),
Error(ValueContext<'input>)
}
antlr_rust::type_id!{ValueContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ValueContextAll<'input>{}

impl<'input> JasmParserContext<'input> for ValueContextAll<'input>{}

impl<'input> Deref for ValueContextAll<'input>{
	type Target = dyn ValueContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ValueContextAll::*;
		match self{
			FalseValueContext(inner) => inner,
			StringValueContext(inner) => inner,
			TrueValueContext(inner) => inner,
			FloatValueContext(inner) => inner,
			IntegerValueContext(inner) => inner,
			NullValueContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ValueContextAll<'input>{
    fn enter(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ValueContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::type_id!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
		ValueContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ValueContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<ValueContextExt<'input>>{


}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

pub type FalseValueContext<'input> = BaseParserRuleContext<'input,FalseValueContextExt<'input>>;

pub trait FalseValueContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FALSE
	/// Returns `None` if there is no child corresponding to token FALSE
	fn FALSE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(FALSE, 0)
	}
}

impl<'input> FalseValueContextAttrs<'input> for FalseValueContext<'input>{}

pub struct FalseValueContextExt<'input>{
	base:ValueContextExt<'input>,
	pub v: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{FalseValueContextExt<'a>}

impl<'input> JasmParserContext<'input> for FalseValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for FalseValueContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_falseValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for FalseValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for FalseValueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for FalseValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for FalseValueContext<'input> {}

impl<'input> FalseValueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::FalseValueContext(
				BaseParserRuleContext::copy_from(ctx,FalseValueContextExt{
					v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringValueContext<'input> = BaseParserRuleContext<'input,StringValueContextExt<'input>>;

pub trait StringValueContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves all `TerminalNode`s corresponding to token DOUBLEQUOTE in current rule
	fn DOUBLEQUOTE_all(&self) -> Vec<Rc<TerminalNode<'input,JasmParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token DOUBLEQUOTE, starting from 0.
	/// Returns `None` if number of children corresponding to token DOUBLEQUOTE is less or equal than `i`.
	fn DOUBLEQUOTE(&self, i: usize) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(DOUBLEQUOTE, i)
	}
	/// Retrieves first TerminalNode corresponding to token TEXT
	/// Returns `None` if there is no child corresponding to token TEXT
	fn TEXT(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(TEXT, 0)
	}
}

impl<'input> StringValueContextAttrs<'input> for StringValueContext<'input>{}

pub struct StringValueContextExt<'input>{
	base:ValueContextExt<'input>,
	pub v: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{StringValueContextExt<'a>}

impl<'input> JasmParserContext<'input> for StringValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for StringValueContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stringValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for StringValueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for StringValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for StringValueContext<'input> {}

impl<'input> StringValueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::StringValueContext(
				BaseParserRuleContext::copy_from(ctx,StringValueContextExt{
					v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TrueValueContext<'input> = BaseParserRuleContext<'input,TrueValueContextExt<'input>>;

pub trait TrueValueContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TRUE
	/// Returns `None` if there is no child corresponding to token TRUE
	fn TRUE(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(TRUE, 0)
	}
}

impl<'input> TrueValueContextAttrs<'input> for TrueValueContext<'input>{}

pub struct TrueValueContextExt<'input>{
	base:ValueContextExt<'input>,
	pub v: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{TrueValueContextExt<'a>}

impl<'input> JasmParserContext<'input> for TrueValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for TrueValueContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_trueValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for TrueValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for TrueValueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for TrueValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for TrueValueContext<'input> {}

impl<'input> TrueValueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::TrueValueContext(
				BaseParserRuleContext::copy_from(ctx,TrueValueContextExt{
					v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FloatValueContext<'input> = BaseParserRuleContext<'input,FloatValueContextExt<'input>>;

pub trait FloatValueContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DECIMAL
	/// Returns `None` if there is no child corresponding to token DECIMAL
	fn DECIMAL(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(DECIMAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> FloatValueContextAttrs<'input> for FloatValueContext<'input>{}

pub struct FloatValueContextExt<'input>{
	base:ValueContextExt<'input>,
	pub sign: Option<TokenType<'input>>,
	pub v: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{FloatValueContextExt<'a>}

impl<'input> JasmParserContext<'input> for FloatValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for FloatValueContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_floatValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for FloatValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for FloatValueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for FloatValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for FloatValueContext<'input> {}

impl<'input> FloatValueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::FloatValueContext(
				BaseParserRuleContext::copy_from(ctx,FloatValueContextExt{
					sign:None, v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntegerValueContext<'input> = BaseParserRuleContext<'input,IntegerValueContextExt<'input>>;

pub trait IntegerValueContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INTEGER
	/// Returns `None` if there is no child corresponding to token INTEGER
	fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(INTEGER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> IntegerValueContextAttrs<'input> for IntegerValueContext<'input>{}

pub struct IntegerValueContextExt<'input>{
	base:ValueContextExt<'input>,
	pub sign: Option<TokenType<'input>>,
	pub v: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{IntegerValueContextExt<'a>}

impl<'input> JasmParserContext<'input> for IntegerValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for IntegerValueContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_integerValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntegerValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for IntegerValueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for IntegerValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for IntegerValueContext<'input> {}

impl<'input> IntegerValueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::IntegerValueContext(
				BaseParserRuleContext::copy_from(ctx,IntegerValueContextExt{
					sign:None, v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NullValueContext<'input> = BaseParserRuleContext<'input,NullValueContextExt<'input>>;

pub trait NullValueContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NULL
	/// Returns `None` if there is no child corresponding to token NULL
	fn NULL(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(NULL, 0)
	}
}

impl<'input> NullValueContextAttrs<'input> for NullValueContext<'input>{}

pub struct NullValueContextExt<'input>{
	base:ValueContextExt<'input>,
	pub v: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{NullValueContextExt<'a>}

impl<'input> JasmParserContext<'input> for NullValueContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for NullValueContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nullValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for NullValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for NullValueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for NullValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for NullValueContext<'input> {}

impl<'input> NullValueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::NullValueContext(
				BaseParserRuleContext::copy_from(ctx,NullValueContextExt{
					v:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(166);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(16,&mut recog.base)? {
				1 =>{
					let tmp = IntegerValueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(153);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==MINUS {
						{
						recog.base.set_state(152);
						let tmp = recog.base.match_token(MINUS,&mut recog.err_handler)?;
						if let ValueContextAll::IntegerValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
						ctx.sign = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(155);
					let tmp = recog.base.match_token(INTEGER,&mut recog.err_handler)?;
					if let ValueContextAll::IntegerValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				2 =>{
					let tmp = FloatValueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(157);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==MINUS {
						{
						recog.base.set_state(156);
						let tmp = recog.base.match_token(MINUS,&mut recog.err_handler)?;
						if let ValueContextAll::FloatValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
						ctx.sign = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(159);
					let tmp = recog.base.match_token(DECIMAL,&mut recog.err_handler)?;
					if let ValueContextAll::FloatValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				3 =>{
					let tmp = TrueValueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(160);
					let tmp = recog.base.match_token(TRUE,&mut recog.err_handler)?;
					if let ValueContextAll::TrueValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				4 =>{
					let tmp = FalseValueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(161);
					let tmp = recog.base.match_token(FALSE,&mut recog.err_handler)?;
					if let ValueContextAll::FalseValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				5 =>{
					let tmp = NullValueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(162);
					let tmp = recog.base.match_token(NULL,&mut recog.err_handler)?;
					if let ValueContextAll::NullValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				6 =>{
					let tmp = StringValueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(163);
					recog.base.match_token(DOUBLEQUOTE,&mut recog.err_handler)?;

					recog.base.set_state(164);
					let tmp = recog.base.match_token(TEXT,&mut recog.err_handler)?;
					if let ValueContextAll::StringValueContext(ctx) = cast_mut::<_,ValueContextAll >(&mut _localctx){
					ctx.v = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(165);
					recog.base.match_token(DOUBLEQUOTE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameter ----------------
pub type ParameterContextAll<'input> = ParameterContext<'input>;


pub type ParameterContext<'input> = BaseParserRuleContext<'input,ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterContextExt<'input>{
	pub name: Option<TokenType<'input>>,
	pub t: Option<Rc<TypContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ParameterContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::type_id!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				name: None, 
				t: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<ParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn typ(&self) -> Option<Rc<TypContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(168);
			let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
			 cast_mut::<_,ParameterContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(169);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule typ*/
			recog.base.set_state(170);
			let tmp = recog.typ()?;
			 cast_mut::<_,ParameterContext >(&mut _localctx).t = Some(tmp.clone());
			  

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typ ----------------
#[derive(Debug)]
pub enum TypContextAll<'input>{
	PointerTypeContext(PointerTypeContext<'input>),
	ValueTypeContext(ValueTypeContext<'input>),
Error(TypContext<'input>)
}
antlr_rust::type_id!{TypContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TypContextAll<'input>{}

impl<'input> JasmParserContext<'input> for TypContextAll<'input>{}

impl<'input> Deref for TypContextAll<'input>{
	type Target = dyn TypContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TypContextAll::*;
		match self{
			PointerTypeContext(inner) => inner,
			ValueTypeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for TypContextAll<'input>{
    fn enter(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn JasmListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TypContext<'input> = BaseParserRuleContext<'input,TypContextExt<'input>>;

#[derive(Clone)]
pub struct TypContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for TypContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for TypContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TypContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typ }
}
antlr_rust::type_id!{TypContextExt<'a>}

impl<'input> TypContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypContextAll<'input>> {
		Rc::new(
		TypContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TypContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<TypContextExt<'input>>{


}

impl<'input> TypContextAttrs<'input> for TypContext<'input>{}

pub type PointerTypeContext<'input> = BaseParserRuleContext<'input,PointerTypeContextExt<'input>>;

pub trait PointerTypeContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ASTERISK
	/// Returns `None` if there is no child corresponding to token ASTERISK
	fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(ASTERISK, 0)
	}
	fn typ(&self) -> Option<Rc<TypContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PointerTypeContextAttrs<'input> for PointerTypeContext<'input>{}

pub struct PointerTypeContextExt<'input>{
	base:TypContextExt<'input>,
	pub t: Option<Rc<TypContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{PointerTypeContextExt<'a>}

impl<'input> JasmParserContext<'input> for PointerTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for PointerTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pointerType(self);
	}
}

impl<'input> CustomRuleContext<'input> for PointerTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typ }
}

impl<'input> Borrow<TypContextExt<'input>> for PointerTypeContext<'input>{
	fn borrow(&self) -> &TypContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypContextExt<'input>> for PointerTypeContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypContextExt<'input> { &mut self.base }
}

impl<'input> TypContextAttrs<'input> for PointerTypeContext<'input> {}

impl<'input> PointerTypeContextExt<'input>{
	fn new(ctx: &dyn TypContextAttrs<'input>) -> Rc<TypContextAll<'input>>  {
		Rc::new(
			TypContextAll::PointerTypeContext(
				BaseParserRuleContext::copy_from(ctx,PointerTypeContextExt{
        			t:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueTypeContext<'input> = BaseParserRuleContext<'input,ValueTypeContextExt<'input>>;

pub trait ValueTypeContextAttrs<'input>: JasmParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
}

impl<'input> ValueTypeContextAttrs<'input> for ValueTypeContext<'input>{}

pub struct ValueTypeContextExt<'input>{
	base:TypContextExt<'input>,
	pub name: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ValueTypeContextExt<'a>}

impl<'input> JasmParserContext<'input> for ValueTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for ValueTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_valueType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typ }
}

impl<'input> Borrow<TypContextExt<'input>> for ValueTypeContext<'input>{
	fn borrow(&self) -> &TypContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TypContextExt<'input>> for ValueTypeContext<'input>{
	fn borrow_mut(&mut self) -> &mut TypContextExt<'input> { &mut self.base }
}

impl<'input> TypContextAttrs<'input> for ValueTypeContext<'input> {}

impl<'input> ValueTypeContextExt<'input>{
	fn new(ctx: &dyn TypContextAttrs<'input>) -> Rc<TypContextAll<'input>>  {
		Rc::new(
			TypContextAll::ValueTypeContext(
				BaseParserRuleContext::copy_from(ctx,ValueTypeContextExt{
					name:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typ(&mut self,)
	-> Result<Rc<TypContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_typ);
        let mut _localctx: Rc<TypContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(175);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					let tmp = ValueTypeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(172);
					let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
					if let TypContextAll::ValueTypeContext(ctx) = cast_mut::<_,TypContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

			 ASTERISK 
				=> {
					let tmp = PointerTypeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(173);
					recog.base.match_token(ASTERISK,&mut recog.err_handler)?;

					/*InvokeRule typ*/
					recog.base.set_state(174);
					let tmp = recog.typ()?;
					if let TypContextAll::PointerTypeContext(ctx) = cast_mut::<_,TypContextAll >(&mut _localctx){
					ctx.t = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- memberAssign ----------------
pub type MemberAssignContextAll<'input> = MemberAssignContext<'input>;


pub type MemberAssignContext<'input> = BaseParserRuleContext<'input,MemberAssignContextExt<'input>>;

#[derive(Clone)]
pub struct MemberAssignContextExt<'input>{
	pub name: Option<TokenType<'input>>,
	pub expr: Option<Rc<ExpressionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JasmParserContext<'input> for MemberAssignContext<'input>{}

impl<'input,'a> Listenable<dyn JasmListener<'input> + 'a> for MemberAssignContext<'input>{
	fn enter(&self,listener: &mut (dyn JasmListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_memberAssign(self);
	}
}

impl<'input> CustomRuleContext<'input> for MemberAssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JasmParserContextType;
	fn get_rule_index(&self) -> usize { RULE_memberAssign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_memberAssign }
}
antlr_rust::type_id!{MemberAssignContextExt<'a>}

impl<'input> MemberAssignContextExt<'input>{
	fn new(parent: Option<Rc<dyn JasmParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MemberAssignContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MemberAssignContextExt{
				name: None, 
				expr: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MemberAssignContextAttrs<'input>: JasmParserContext<'input> + BorrowMut<MemberAssignContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JasmParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MemberAssignContextAttrs<'input> for MemberAssignContext<'input>{}

impl<'input, I, H> JasmParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn memberAssign(&mut self,)
	-> Result<Rc<MemberAssignContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MemberAssignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_memberAssign);
        let mut _localctx: Rc<MemberAssignContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(177);
			let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
			 cast_mut::<_,MemberAssignContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(178);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(179);
			let tmp = recog.expression_rec(0)?;
			 cast_mut::<_,MemberAssignContext >(&mut _localctx).expr = Some(tmp.clone());
			  

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<DFA>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ))
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x31\u{b8}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\
	\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x07\x02\x24\x0a\
	\x02\x0c\x02\x0e\x02\x27\x0b\x02\x05\x02\x29\x0a\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x03\x02\x05\x02\x39\x0a\x02\x03\x02\x05\x02\x3c\x0a\x02\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\x44\x0a\x03\x05\x03\
	\x46\x0a\x03\x03\x04\x03\x04\x07\x04\x4a\x0a\x04\x0c\x04\x0e\x04\x4d\x0b\
	\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x07\
	\x05\x57\x0a\x05\x0c\x05\x0e\x05\x5a\x0b\x05\x05\x05\x5c\x0a\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x07\x06\x72\x0a\x06\x0c\x06\x0e\x06\x75\x0b\x06\x05\x06\x77\x0a\
	\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\x7d\x0a\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x07\x06\u{96}\x0a\x06\x0c\x06\x0e\x06\u{99}\x0b\
	\x06\x03\x07\x05\x07\u{9c}\x0a\x07\x03\x07\x03\x07\x05\x07\u{a0}\x0a\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\u{a9}\x0a\
	\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x05\x09\u{b2}\
	\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x02\x03\x0a\x0b\x02\x04\
	\x06\x08\x0a\x0c\x0e\x10\x12\x02\x07\x04\x02\x13\x13\x15\x15\x03\x02\x16\
	\x18\x03\x02\x14\x15\x03\x02\x1a\x1f\x04\x02\x0f\x10\x20\x22\x02\u{d4}\x02\
	\x3b\x03\x02\x02\x02\x04\x3d\x03\x02\x02\x02\x06\x47\x03\x02\x02\x02\x08\
	\x50\x03\x02\x02\x02\x0a\x7c\x03\x02\x02\x02\x0c\u{a8}\x03\x02\x02\x02\x0e\
	\u{aa}\x03\x02\x02\x02\x10\u{b1}\x03\x02\x02\x02\x12\u{b3}\x03\x02\x02\x02\
	\x14\x15\x05\x0a\x06\x02\x15\x16\x07\x29\x02\x02\x16\x3c\x03\x02\x02\x02\
	\x17\x3c\x05\x04\x03\x02\x18\x19\x07\x09\x02\x02\x19\x1a\x05\x0a\x06\x02\
	\x1a\x1b\x05\x06\x04\x02\x1b\x3c\x03\x02\x02\x02\x1c\x3c\x05\x08\x05\x02\
	\x1d\x1e\x07\x0c\x02\x02\x1e\x1f\x07\x30\x02\x02\x1f\x28\x07\x25\x02\x02\
	\x20\x25\x05\x0e\x08\x02\x21\x22\x07\x2a\x02\x02\x22\x24\x05\x0e\x08\x02\
	\x23\x21\x03\x02\x02\x02\x24\x27\x03\x02\x02\x02\x25\x23\x03\x02\x02\x02\
	\x25\x26\x03\x02\x02\x02\x26\x29\x03\x02\x02\x02\x27\x25\x03\x02\x02\x02\
	\x28\x20\x03\x02\x02\x02\x28\x29\x03\x02\x02\x02\x29\x2a\x03\x02\x02\x02\
	\x2a\x3c\x07\x26\x02\x02\x2b\x2c\x07\x04\x02\x02\x2c\x2d\x07\x30\x02\x02\
	\x2d\x2e\x07\x2b\x02\x02\x2e\x2f\x05\x10\x09\x02\x2f\x30\x07\x29\x02\x02\
	\x30\x3c\x03\x02\x02\x02\x31\x32\x05\x0a\x06\x02\x32\x33\x07\x19\x02\x02\
	\x33\x34\x05\x0a\x06\x02\x34\x35\x07\x29\x02\x02\x35\x3c\x03\x02\x02\x02\
	\x36\x38\x07\x0e\x02\x02\x37\x39\x05\x0a\x06\x02\x38\x37\x03\x02\x02\x02\
	\x38\x39\x03\x02\x02\x02\x39\x3a\x03\x02\x02\x02\x3a\x3c\x07\x29\x02\x02\
	\x3b\x14\x03\x02\x02\x02\x3b\x17\x03\x02\x02\x02\x3b\x18\x03\x02\x02\x02\
	\x3b\x1c\x03\x02\x02\x02\x3b\x1d\x03\x02\x02\x02\x3b\x2b\x03\x02\x02\x02\
	\x3b\x31\x03\x02\x02\x02\x3b\x36\x03\x02\x02\x02\x3c\x03\x03\x02\x02\x02\
	\x3d\x3e\x07\x05\x02\x02\x3e\x3f\x05\x0a\x06\x02\x3f\x45\x05\x06\x04\x02\
	\x40\x43\x07\x06\x02\x02\x41\x44\x05\x04\x03\x02\x42\x44\x05\x06\x04\x02\
	\x43\x41\x03\x02\x02\x02\x43\x42\x03\x02\x02\x02\x44\x46\x03\x02\x02\x02\
	\x45\x40\x03\x02\x02\x02\x45\x46\x03\x02\x02\x02\x46\x05\x03\x02\x02\x02\
	\x47\x4b\x07\x25\x02\x02\x48\x4a\x05\x02\x02\x02\x49\x48\x03\x02\x02\x02\
	\x4a\x4d\x03\x02\x02\x02\x4b\x49\x03\x02\x02\x02\x4b\x4c\x03\x02\x02\x02\
	\x4c\x4e\x03\x02\x02\x02\x4d\x4b\x03\x02\x02\x02\x4e\x4f\x07\x26\x02\x02\
	\x4f\x07\x03\x02\x02\x02\x50\x51\x07\x0b\x02\x02\x51\x52\x07\x30\x02\x02\
	\x52\x5b\x07\x23\x02\x02\x53\x58\x05\x0e\x08\x02\x54\x55\x07\x2a\x02\x02\
	\x55\x57\x05\x0e\x08\x02\x56\x54\x03\x02\x02\x02\x57\x5a\x03\x02\x02\x02\
	\x58\x56\x03\x02\x02\x02\x58\x59\x03\x02\x02\x02\x59\x5c\x03\x02\x02\x02\
	\x5a\x58\x03\x02\x02\x02\x5b\x53\x03\x02\x02\x02\x5b\x5c\x03\x02\x02\x02\
	\x5c\x5d\x03\x02\x02\x02\x5d\x5e\x07\x24\x02\x02\x5e\x5f\x07\x2b\x02\x02\
	\x5f\x60\x05\x10\x09\x02\x60\x61\x05\x06\x04\x02\x61\x09\x03\x02\x02\x02\
	\x62\x63\x08\x06\x01\x02\x63\x7d\x05\x0c\x07\x02\x64\x65\x07\x23\x02\x02\
	\x65\x66\x05\x0a\x06\x02\x66\x67\x07\x24\x02\x02\x67\x7d\x03\x02\x02\x02\
	\x68\x69\x07\x11\x02\x02\x69\x7d\x05\x0a\x06\x0d\x6a\x6b\x07\x16\x02\x02\
	\x6b\x7d\x05\x0a\x06\x0c\x6c\x6d\x07\x30\x02\x02\x6d\x76\x07\x23\x02\x02\
	\x6e\x73\x05\x0a\x06\x02\x6f\x70\x07\x2a\x02\x02\x70\x72\x05\x0a\x06\x02\
	\x71\x6f\x03\x02\x02\x02\x72\x75\x03\x02\x02\x02\x73\x71\x03\x02\x02\x02\
	\x73\x74\x03\x02\x02\x02\x74\x77\x03\x02\x02\x02\x75\x73\x03\x02\x02\x02\
	\x76\x6e\x03\x02\x02\x02\x76\x77\x03\x02\x02\x02\x77\x78\x03\x02\x02\x02\
	\x78\x7d\x07\x24\x02\x02\x79\x7a\x09\x02\x02\x02\x7a\x7d\x05\x0a\x06\x09\
	\x7b\x7d\x07\x30\x02\x02\x7c\x62\x03\x02\x02\x02\x7c\x64\x03\x02\x02\x02\
	\x7c\x68\x03\x02\x02\x02\x7c\x6a\x03\x02\x02\x02\x7c\x6c\x03\x02\x02\x02\
	\x7c\x79\x03\x02\x02\x02\x7c\x7b\x03\x02\x02\x02\x7d\u{97}\x03\x02\x02\x02\
	\x7e\x7f\x0c\x07\x02\x02\x7f\u{80}\x09\x03\x02\x02\u{80}\u{96}\x05\x0a\x06\
	\x08\u{81}\u{82}\x0c\x06\x02\x02\u{82}\u{83}\x09\x04\x02\x02\u{83}\u{96}\
	\x05\x0a\x06\x07\u{84}\u{85}\x0c\x05\x02\x02\u{85}\u{86}\x09\x05\x02\x02\
	\u{86}\u{96}\x05\x0a\x06\x06\u{87}\u{88}\x0c\x04\x02\x02\u{88}\u{89}\x09\
	\x06\x02\x02\u{89}\u{96}\x05\x0a\x06\x05\u{8a}\u{8b}\x0c\x0e\x02\x02\u{8b}\
	\u{8c}\x07\x12\x02\x02\u{8c}\u{96}\x07\x30\x02\x02\u{8d}\u{8e}\x0c\x0a\x02\
	\x02\u{8e}\u{8f}\x07\x27\x02\x02\u{8f}\u{90}\x05\x0a\x06\x02\u{90}\u{91}\
	\x07\x28\x02\x02\u{91}\u{96}\x03\x02\x02\x02\u{92}\u{93}\x0c\x08\x02\x02\
	\u{93}\u{94}\x07\x0d\x02\x02\u{94}\u{96}\x05\x10\x09\x02\u{95}\x7e\x03\x02\
	\x02\x02\u{95}\u{81}\x03\x02\x02\x02\u{95}\u{84}\x03\x02\x02\x02\u{95}\u{87}\
	\x03\x02\x02\x02\u{95}\u{8a}\x03\x02\x02\x02\u{95}\u{8d}\x03\x02\x02\x02\
	\u{95}\u{92}\x03\x02\x02\x02\u{96}\u{99}\x03\x02\x02\x02\u{97}\u{95}\x03\
	\x02\x02\x02\u{97}\u{98}\x03\x02\x02\x02\u{98}\x0b\x03\x02\x02\x02\u{99}\
	\u{97}\x03\x02\x02\x02\u{9a}\u{9c}\x07\x15\x02\x02\u{9b}\u{9a}\x03\x02\x02\
	\x02\u{9b}\u{9c}\x03\x02\x02\x02\u{9c}\u{9d}\x03\x02\x02\x02\u{9d}\u{a9}\
	\x07\x2e\x02\x02\u{9e}\u{a0}\x07\x15\x02\x02\u{9f}\u{9e}\x03\x02\x02\x02\
	\u{9f}\u{a0}\x03\x02\x02\x02\u{a0}\u{a1}\x03\x02\x02\x02\u{a1}\u{a9}\x07\
	\x2f\x02\x02\u{a2}\u{a9}\x07\x07\x02\x02\u{a3}\u{a9}\x07\x08\x02\x02\u{a4}\
	\u{a9}\x07\x0a\x02\x02\u{a5}\u{a6}\x07\x2d\x02\x02\u{a6}\u{a7}\x07\x31\x02\
	\x02\u{a7}\u{a9}\x07\x2d\x02\x02\u{a8}\u{9b}\x03\x02\x02\x02\u{a8}\u{9f}\
	\x03\x02\x02\x02\u{a8}\u{a2}\x03\x02\x02\x02\u{a8}\u{a3}\x03\x02\x02\x02\
	\u{a8}\u{a4}\x03\x02\x02\x02\u{a8}\u{a5}\x03\x02\x02\x02\u{a9}\x0d\x03\x02\
	\x02\x02\u{aa}\u{ab}\x07\x30\x02\x02\u{ab}\u{ac}\x07\x2b\x02\x02\u{ac}\u{ad}\
	\x05\x10\x09\x02\u{ad}\x0f\x03\x02\x02\x02\u{ae}\u{b2}\x07\x30\x02\x02\u{af}\
	\u{b0}\x07\x16\x02\x02\u{b0}\u{b2}\x05\x10\x09\x02\u{b1}\u{ae}\x03\x02\x02\
	\x02\u{b1}\u{af}\x03\x02\x02\x02\u{b2}\x11\x03\x02\x02\x02\u{b3}\u{b4}\x07\
	\x30\x02\x02\u{b4}\u{b5}\x07\x2b\x02\x02\u{b5}\u{b6}\x05\x0a\x06\x02\u{b6}\
	\x13\x03\x02\x02\x02\x14\x25\x28\x38\x3b\x43\x45\x4b\x58\x5b\x73\x76\x7c\
	\u{95}\u{97}\u{9b}\u{9f}\u{a8}\u{b1}";

