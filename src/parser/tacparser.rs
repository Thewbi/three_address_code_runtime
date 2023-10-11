// Generated from ../../src/parser/tac.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
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
use super::taclistener::*;
use super::tacvisitor::*;

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

		pub const ACALL:isize=1; 
		pub const AMPERSAND:isize=2; 
		pub const ASTERISK:isize=3; 
		pub const AT:isize=4; 
		pub const BACKSLASH:isize=5; 
		pub const BAR:isize=6; 
		pub const BEGINFUNC:isize=7; 
		pub const BREAK:isize=8; 
		pub const CALL:isize=9; 
		pub const CLOSEING_BRACKET:isize=10; 
		pub const COLON:isize=11; 
		pub const COMMA:isize=12; 
		pub const DOT:isize=13; 
		pub const ELSE:isize=14; 
		pub const ENDIF:isize=15; 
		pub const ENDFUNC:isize=16; 
		pub const EQUALS:isize=17; 
		pub const FALSE:isize=18; 
		pub const GOTO:isize=19; 
		pub const GT:isize=20; 
		pub const GTE:isize=21; 
		pub const HASH_TAG:isize=22; 
		pub const IF:isize=23; 
		pub const INCLUDE:isize=24; 
		pub const LCALL:isize=25; 
		pub const LEFT_SHIFT:isize=26; 
		pub const LT:isize=27; 
		pub const LTE:isize=28; 
		pub const MACRO:isize=29; 
		pub const MINUS:isize=30; 
		pub const OPENING_BRACKET:isize=31; 
		pub const PLUS:isize=32; 
		pub const POP:isize=33; 
		pub const PRINT:isize=34; 
		pub const PUSH:isize=35; 
		pub const RETURN:isize=36; 
		pub const RIGHT_SHIFT:isize=37; 
		pub const SIZEOF:isize=38; 
		pub const SQRT:isize=39; 
		pub const SLASH:isize=40; 
		pub const TRUE:isize=41; 
		pub const VTABLE:isize=42; 
		pub const NEWLINE:isize=43; 
		pub const WS:isize=44; 
		pub const LINE_COMMENT:isize=45; 
		pub const BLOCK_COMMENT:isize=46; 
		pub const DOUBLE_SLASH_LINE_COMMENT:isize=47; 
		pub const STRING:isize=48; 
		pub const CHAR:isize=49; 
		pub const ESCAPED_CHAR:isize=50; 
		pub const NUMBER:isize=51; 
		pub const HEX_NUMBER:isize=52; 
		pub const BINARY_NUMBER:isize=53; 
		pub const IDENTIFIER:isize=54;
	pub const RULE_compilation_unit:usize = 0; 
	pub const RULE_source_line:usize = 1; 
	pub const RULE_label:usize = 2; 
	pub const RULE_assignment:usize = 3; 
	pub const RULE_left_hand_side:usize = 4; 
	pub const RULE_lcall_expression:usize = 5; 
	pub const RULE_predicate:usize = 6; 
	pub const RULE_expression:usize = 7; 
	pub const RULE_unary_operator:usize = 8; 
	pub const RULE_binary_operator:usize = 9; 
	pub const RULE_or_operator:usize = 10; 
	pub const RULE_and_operator:usize = 11; 
	pub const RULE_equals_operator:usize = 12; 
	pub const RULE_operand:usize = 13; 
	pub const RULE_function_call:usize = 14; 
	pub const RULE_parameter_list:usize = 15; 
	pub const RULE_parameter:usize = 16; 
	pub const RULE_control_flow:usize = 17; 
	pub const RULE_if_statement:usize = 18; 
	pub const RULE_function_identifier:usize = 19; 
	pub const RULE_function_definition:usize = 20; 
	pub const RULE_function_body:usize = 21; 
	pub const RULE_class_method_identifier:usize = 22; 
	pub const RULE_vtable_assignment:usize = 23; 
	pub const RULE_class_method_list:usize = 24;
	pub const ruleNames: [&'static str; 25] =  [
		"compilation_unit", "source_line", "label", "assignment", "left_hand_side", 
		"lcall_expression", "predicate", "expression", "unary_operator", "binary_operator", 
		"or_operator", "and_operator", "equals_operator", "operand", "function_call", 
		"parameter_list", "parameter", "control_flow", "if_statement", "function_identifier", 
		"function_definition", "function_body", "class_method_identifier", "vtable_assignment", 
		"class_method_list"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;41] = [
		None, None, Some("'&'"), Some("'*'"), Some("'@'"), Some("'\\'"), Some("'|'"), 
		None, None, None, Some("')'"), Some("':'"), Some("','"), Some("'.'"), 
		Some("'else'"), Some("'endif'"), None, Some("'='"), None, None, Some("'>'"), 
		Some("'>='"), Some("'#'"), Some("'if'"), Some("'include'"), None, Some("'<<'"), 
		Some("'<'"), Some("'<='"), Some("'macro'"), Some("'-'"), Some("'('"), 
		Some("'+'"), None, None, None, None, Some("'>>'"), None, None, Some("'/'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;55]  = [
		None, Some("ACALL"), Some("AMPERSAND"), Some("ASTERISK"), Some("AT"), 
		Some("BACKSLASH"), Some("BAR"), Some("BEGINFUNC"), Some("BREAK"), Some("CALL"), 
		Some("CLOSEING_BRACKET"), Some("COLON"), Some("COMMA"), Some("DOT"), Some("ELSE"), 
		Some("ENDIF"), Some("ENDFUNC"), Some("EQUALS"), Some("FALSE"), Some("GOTO"), 
		Some("GT"), Some("GTE"), Some("HASH_TAG"), Some("IF"), Some("INCLUDE"), 
		Some("LCALL"), Some("LEFT_SHIFT"), Some("LT"), Some("LTE"), Some("MACRO"), 
		Some("MINUS"), Some("OPENING_BRACKET"), Some("PLUS"), Some("POP"), Some("PRINT"), 
		Some("PUSH"), Some("RETURN"), Some("RIGHT_SHIFT"), Some("SIZEOF"), Some("SQRT"), 
		Some("SLASH"), Some("TRUE"), Some("VTABLE"), Some("NEWLINE"), Some("WS"), 
		Some("LINE_COMMENT"), Some("BLOCK_COMMENT"), Some("DOUBLE_SLASH_LINE_COMMENT"), 
		Some("STRING"), Some("CHAR"), Some("ESCAPED_CHAR"), Some("NUMBER"), Some("HEX_NUMBER"), 
		Some("BINARY_NUMBER"), Some("IDENTIFIER")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,tacExt<'input>, I, tacContextType , dyn tacListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
//pub type LocalTokenFactory<'input> = CommonTokenFactory;
pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;

pub type tacTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, tacContextType , dyn tacListener<'input> + 'a>;

/// Parser for tac grammar
pub struct tac<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				tacExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> tac<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> tac<'input, I, DefaultErrorStrategy<'input,tacContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for tac
pub trait tacContext<'input>:
	for<'x> Listenable<dyn tacListener<'input> + 'x > + 
	for<'x> Visitable<dyn tacVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=tacContextType>
{}

antlr_rust::coerce_from!{ 'input : tacContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn tacContext<'input> + 'input
where
    T: tacVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn tacVisitor<'input> + 'x))
    }
}

impl<'input> tacContext<'input> for TerminalNode<'input,tacContextType> {}
impl<'input> tacContext<'input> for ErrorNode<'input,tacContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn tacContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn tacListener<'input> + 'input }

pub struct tacContextType;
antlr_rust::tid!{tacContextType}

impl<'input> ParserNodeType<'input> for tacContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn tacContext<'input> + 'input;
}

impl<'input, I, H> Deref for tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct tacExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> tacExt<'input>{
}
antlr_rust::tid! { tacExt<'a> }

impl<'input> TokenAware<'input> for tacExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for tacExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for tacExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "tac.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn tacContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					7 => tac::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					24 => tac::<'input,I,_>::class_method_list_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> tac<'input, I, DefaultErrorStrategy<'input,tacContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn class_method_list_sempred(_localctx: Option<&Class_method_listContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- compilation_unit ----------------
pub type Compilation_unitContextAll<'input> = Compilation_unitContext<'input>;


pub type Compilation_unitContext<'input> = BaseParserRuleContext<'input,Compilation_unitContextExt<'input>>;

#[derive(Clone)]
pub struct Compilation_unitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Compilation_unitContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Compilation_unitContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilation_unit(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_compilation_unit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Compilation_unitContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_compilation_unit(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compilation_unitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_compilation_unit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilation_unit }
}
antlr_rust::tid!{Compilation_unitContextExt<'a>}

impl<'input> Compilation_unitContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compilation_unitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compilation_unitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compilation_unitContextAttrs<'input>: tacContext<'input> + BorrowMut<Compilation_unitContextExt<'input>>{

fn source_line_all(&self) ->  Vec<Rc<Source_lineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn source_line(&self, i: usize) -> Option<Rc<Source_lineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,tacContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> Compilation_unitContextAttrs<'input> for Compilation_unitContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilation_unit(&mut self,)
	-> Result<Rc<Compilation_unitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compilation_unitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_compilation_unit);
        let mut _localctx: Rc<Compilation_unitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(53);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(50);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(55);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule source_line*/
			recog.base.set_state(56);
			recog.source_line()?;

			recog.base.set_state(66);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(60);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(57);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(62);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule source_line*/
					recog.base.set_state(63);
					recog.source_line()?;

					}
					} 
				}
				recog.base.set_state(68);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			recog.base.set_state(72);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(69);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(74);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(75);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- source_line ----------------
pub type Source_lineContextAll<'input> = Source_lineContext<'input>;


pub type Source_lineContext<'input> = BaseParserRuleContext<'input,Source_lineContextExt<'input>>;

#[derive(Clone)]
pub struct Source_lineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Source_lineContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Source_lineContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_source_line(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_source_line(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Source_lineContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_source_line(self);
	}
}

impl<'input> CustomRuleContext<'input> for Source_lineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_source_line }
	//fn type_rule_index() -> usize where Self: Sized { RULE_source_line }
}
antlr_rust::tid!{Source_lineContextExt<'a>}

impl<'input> Source_lineContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Source_lineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Source_lineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Source_lineContextAttrs<'input>: tacContext<'input> + BorrowMut<Source_lineContextExt<'input>>{

fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn control_flow(&self) -> Option<Rc<Control_flowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_method_identifier(&self) -> Option<Rc<Class_method_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_definition(&self) -> Option<Rc<Function_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn vtable_assignment(&self) -> Option<Rc<Vtable_assignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Source_lineContextAttrs<'input> for Source_lineContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn source_line(&mut self,)
	-> Result<Rc<Source_lineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Source_lineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_source_line);
        let mut _localctx: Rc<Source_lineContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(103);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(80);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule label*/
							recog.base.set_state(77);
							recog.label()?;

							recog.base.set_state(78);
							recog.base.match_token(COLON,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					/*InvokeRule assignment*/
					recog.base.set_state(82);
					recog.assignment()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(86);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IDENTIFIER {
						{
						/*InvokeRule label*/
						recog.base.set_state(83);
						recog.label()?;

						recog.base.set_state(84);
						recog.base.match_token(COLON,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule control_flow*/
					recog.base.set_state(88);
					recog.control_flow()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(92);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IDENTIFIER {
						{
						/*InvokeRule label*/
						recog.base.set_state(89);
						recog.label()?;

						recog.base.set_state(90);
						recog.base.match_token(COLON,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule function_call*/
					recog.base.set_state(94);
					recog.function_call()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					{
					/*InvokeRule label*/
					recog.base.set_state(95);
					recog.label()?;

					recog.base.set_state(96);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					{
					/*InvokeRule class_method_identifier*/
					recog.base.set_state(98);
					recog.class_method_identifier()?;

					recog.base.set_state(99);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					}
					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule function_definition*/
					recog.base.set_state(101);
					recog.function_definition()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule vtable_assignment*/
					recog.base.set_state(102);
					recog.vtable_assignment()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- label ----------------
pub type LabelContextAll<'input> = LabelContext<'input>;


pub type LabelContext<'input> = BaseParserRuleContext<'input,LabelContextExt<'input>>;

#[derive(Clone)]
pub struct LabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for LabelContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for LabelContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_label(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_label(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for LabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_label(self);
	}
}

impl<'input> CustomRuleContext<'input> for LabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_label }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label }
}
antlr_rust::tid!{LabelContextExt<'a>}

impl<'input> LabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LabelContextAttrs<'input>: tacContext<'input> + BorrowMut<LabelContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> LabelContextAttrs<'input> for LabelContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label(&mut self,)
	-> Result<Rc<LabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_label);
        let mut _localctx: Rc<LabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(105);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- assignment ----------------
pub type AssignmentContextAll<'input> = AssignmentContext<'input>;


pub type AssignmentContext<'input> = BaseParserRuleContext<'input,AssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for AssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for AssignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assignment(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_assignment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for AssignmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_assignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment }
}
antlr_rust::tid!{AssignmentContextExt<'a>}

impl<'input> AssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentContextAttrs<'input>: tacContext<'input> + BorrowMut<AssignmentContextExt<'input>>{

fn left_hand_side(&self) -> Option<Rc<Left_hand_sideContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lcall_expression(&self) -> Option<Rc<Lcall_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssignmentContextAttrs<'input> for AssignmentContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignment(&mut self,)
	-> Result<Rc<AssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_assignment);
        let mut _localctx: Rc<AssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(115);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule left_hand_side*/
					recog.base.set_state(107);
					recog.left_hand_side()?;

					recog.base.set_state(108);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(109);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule left_hand_side*/
					recog.base.set_state(111);
					recog.left_hand_side()?;

					recog.base.set_state(112);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					/*InvokeRule lcall_expression*/
					recog.base.set_state(113);
					recog.lcall_expression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- left_hand_side ----------------
pub type Left_hand_sideContextAll<'input> = Left_hand_sideContext<'input>;


pub type Left_hand_sideContext<'input> = BaseParserRuleContext<'input,Left_hand_sideContextExt<'input>>;

#[derive(Clone)]
pub struct Left_hand_sideContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Left_hand_sideContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Left_hand_sideContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_left_hand_side(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_left_hand_side(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Left_hand_sideContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_left_hand_side(self);
	}
}

impl<'input> CustomRuleContext<'input> for Left_hand_sideContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_left_hand_side }
	//fn type_rule_index() -> usize where Self: Sized { RULE_left_hand_side }
}
antlr_rust::tid!{Left_hand_sideContextExt<'a>}

impl<'input> Left_hand_sideContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Left_hand_sideContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Left_hand_sideContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Left_hand_sideContextAttrs<'input>: tacContext<'input> + BorrowMut<Left_hand_sideContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Left_hand_sideContextAttrs<'input> for Left_hand_sideContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn left_hand_side(&mut self,)
	-> Result<Rc<Left_hand_sideContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Left_hand_sideContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_left_hand_side);
        let mut _localctx: Rc<Left_hand_sideContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(117);
			recog.expression_rec(0)?;

			}
			Ok(())
		})();
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
//------------------- lcall_expression ----------------
pub type Lcall_expressionContextAll<'input> = Lcall_expressionContext<'input>;


pub type Lcall_expressionContext<'input> = BaseParserRuleContext<'input,Lcall_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Lcall_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Lcall_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Lcall_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lcall_expression(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_lcall_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Lcall_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_lcall_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Lcall_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_lcall_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lcall_expression }
}
antlr_rust::tid!{Lcall_expressionContextExt<'a>}

impl<'input> Lcall_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Lcall_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Lcall_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Lcall_expressionContextAttrs<'input>: tacContext<'input> + BorrowMut<Lcall_expressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LCALL
/// Returns `None` if there is no child corresponding to token LCALL
fn LCALL(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(LCALL, 0)
}
fn function_identifier(&self) -> Option<Rc<Function_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_method_identifier(&self) -> Option<Rc<Class_method_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Lcall_expressionContextAttrs<'input> for Lcall_expressionContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lcall_expression(&mut self,)
	-> Result<Rc<Lcall_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Lcall_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_lcall_expression);
        let mut _localctx: Rc<Lcall_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(119);
			recog.base.match_token(LCALL,&mut recog.err_handler)?;

			recog.base.set_state(122);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule function_identifier*/
					recog.base.set_state(120);
					recog.function_identifier()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule class_method_identifier*/
					recog.base.set_state(121);
					recog.class_method_identifier()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
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
//------------------- predicate ----------------
pub type PredicateContextAll<'input> = PredicateContext<'input>;


pub type PredicateContext<'input> = BaseParserRuleContext<'input,PredicateContextExt<'input>>;

#[derive(Clone)]
pub struct PredicateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for PredicateContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for PredicateContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predicate(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_predicate(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for PredicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate }
}
antlr_rust::tid!{PredicateContextExt<'a>}

impl<'input> PredicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredicateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredicateContextAttrs<'input>: tacContext<'input> + BorrowMut<PredicateContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equals_operator(&self) -> Option<Rc<Equals_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TRUE
/// Returns `None` if there is no child corresponding to token TRUE
fn TRUE(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE
/// Returns `None` if there is no child corresponding to token FALSE
fn FALSE(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(FALSE, 0)
}

}

impl<'input> PredicateContextAttrs<'input> for PredicateContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate(&mut self,)
	-> Result<Rc<PredicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_predicate);
        let mut _localctx: Rc<PredicateContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(135);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(124);
					recog.expression_rec(0)?;

					/*InvokeRule equals_operator*/
					recog.base.set_state(125);
					recog.equals_operator()?;

					recog.base.set_state(126);
					_la = recog.base.input.la(1);
					if { !(_la==FALSE || _la==TRUE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(128);
					_la = recog.base.input.la(1);
					if { !(_la==FALSE || _la==TRUE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule equals_operator*/
					recog.base.set_state(129);
					recog.equals_operator()?;

					/*InvokeRule expression*/
					recog.base.set_state(130);
					recog.expression_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expression*/
					recog.base.set_state(132);
					recog.expression_rec(0)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(133);
					recog.base.match_token(TRUE,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(134);
					recog.base.match_token(FALSE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: tacContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPENING_BRACKET
/// Returns `None` if there is no child corresponding to token OPENING_BRACKET
fn OPENING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(OPENING_BRACKET, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token CLOSEING_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSEING_BRACKET
fn CLOSEING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(CLOSEING_BRACKET, 0)
}
fn unary_operator(&self) -> Option<Rc<Unary_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn operand(&self) -> Option<Rc<OperandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn binary_operator(&self) -> Option<Rc<Binary_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
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
		recog.base.enter_recursion_rule(_localctx.clone(), 14, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 14;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(146);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 OPENING_BRACKET 
				=> {
					{
					recog.base.set_state(138);
					recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(139);
					recog.expression_rec(0)?;

					recog.base.set_state(140);
					recog.base.match_token(CLOSEING_BRACKET,&mut recog.err_handler)?;

					}
				}

			 AMPERSAND | ASTERISK | MINUS | PLUS 
				=> {
					{
					/*InvokeRule unary_operator*/
					recog.base.set_state(142);
					recog.unary_operator()?;

					/*InvokeRule expression*/
					recog.base.set_state(143);
					recog.expression_rec(2)?;

					}
				}

			 BREAK | POP | PRINT | PUSH | SIZEOF | SQRT | STRING | NUMBER | IDENTIFIER 
				=> {
					{
					/*InvokeRule operand*/
					recog.base.set_state(145);
					recog.operand()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
					_localctx = tmp;
					recog.base.set_state(148);
					if !({recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					/*InvokeRule binary_operator*/
					recog.base.set_state(149);
					recog.binary_operator()?;

					/*InvokeRule expression*/
					recog.base.set_state(150);
					recog.expression_rec(4)?;

					}
					} 
				}
				recog.base.set_state(156);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			}
			Ok(())
		})();
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
//------------------- unary_operator ----------------
pub type Unary_operatorContextAll<'input> = Unary_operatorContext<'input>;


pub type Unary_operatorContext<'input> = BaseParserRuleContext<'input,Unary_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Unary_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Unary_operatorContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unary_operator(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_unary_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Unary_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_unary_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for Unary_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_operator }
}
antlr_rust::tid!{Unary_operatorContextExt<'a>}

impl<'input> Unary_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Unary_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_operatorContextAttrs<'input>: tacContext<'input> + BorrowMut<Unary_operatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AMPERSAND
/// Returns `None` if there is no child corresponding to token AMPERSAND
fn AMPERSAND(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(AMPERSAND, 0)
}
/// Retrieves first TerminalNode corresponding to token ASTERISK
/// Returns `None` if there is no child corresponding to token ASTERISK
fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(ASTERISK, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}

}

impl<'input> Unary_operatorContextAttrs<'input> for Unary_operatorContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary_operator(&mut self,)
	-> Result<Rc<Unary_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_unary_operator);
        let mut _localctx: Rc<Unary_operatorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(157);
			_la = recog.base.input.la(1);
			if { !(((((_la - 2)) & !0x3f) == 0 && ((1usize << (_la - 2)) & ((1usize << (AMPERSAND - 2)) | (1usize << (ASTERISK - 2)) | (1usize << (MINUS - 2)) | (1usize << (PLUS - 2)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
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
//------------------- binary_operator ----------------
pub type Binary_operatorContextAll<'input> = Binary_operatorContext<'input>;


pub type Binary_operatorContext<'input> = BaseParserRuleContext<'input,Binary_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Binary_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Binary_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Binary_operatorContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_binary_operator(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_binary_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Binary_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_binary_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for Binary_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_binary_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binary_operator }
}
antlr_rust::tid!{Binary_operatorContextExt<'a>}

impl<'input> Binary_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Binary_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Binary_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Binary_operatorContextAttrs<'input>: tacContext<'input> + BorrowMut<Binary_operatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token ASTERISK
/// Returns `None` if there is no child corresponding to token ASTERISK
fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(ASTERISK, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
fn or_operator(&self) -> Option<Rc<Or_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn and_operator(&self) -> Option<Rc<And_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equals_operator(&self) -> Option<Rc<Equals_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LTE
/// Returns `None` if there is no child corresponding to token LTE
fn LTE(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(LTE, 0)
}
/// Retrieves first TerminalNode corresponding to token GTE
/// Returns `None` if there is no child corresponding to token GTE
fn GTE(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(GTE, 0)
}

}

impl<'input> Binary_operatorContextAttrs<'input> for Binary_operatorContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binary_operator(&mut self,)
	-> Result<Rc<Binary_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Binary_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_binary_operator);
        let mut _localctx: Rc<Binary_operatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(170);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PLUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(159);
					recog.base.match_token(PLUS,&mut recog.err_handler)?;

					}
				}

			 MINUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(160);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					}
				}

			 ASTERISK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(161);
					recog.base.match_token(ASTERISK,&mut recog.err_handler)?;

					}
				}

			 SLASH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(162);
					recog.base.match_token(SLASH,&mut recog.err_handler)?;

					}
				}

			 BAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule or_operator*/
					recog.base.set_state(163);
					recog.or_operator()?;

					}
				}

			 AMPERSAND 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule and_operator*/
					recog.base.set_state(164);
					recog.and_operator()?;

					}
				}

			 EQUALS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule equals_operator*/
					recog.base.set_state(165);
					recog.equals_operator()?;

					}
				}

			 LT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(166);
					recog.base.match_token(LT,&mut recog.err_handler)?;

					}
				}

			 GT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(167);
					recog.base.match_token(GT,&mut recog.err_handler)?;

					}
				}

			 LTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(168);
					recog.base.match_token(LTE,&mut recog.err_handler)?;

					}
				}

			 GTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					recog.base.set_state(169);
					recog.base.match_token(GTE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- or_operator ----------------
pub type Or_operatorContextAll<'input> = Or_operatorContext<'input>;


pub type Or_operatorContext<'input> = BaseParserRuleContext<'input,Or_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Or_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Or_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Or_operatorContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_or_operator(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_or_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Or_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_or_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for Or_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_or_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_or_operator }
}
antlr_rust::tid!{Or_operatorContextExt<'a>}

impl<'input> Or_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Or_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Or_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Or_operatorContextAttrs<'input>: tacContext<'input> + BorrowMut<Or_operatorContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token BAR in current rule
fn BAR_all(&self) -> Vec<Rc<TerminalNode<'input,tacContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BAR, starting from 0.
/// Returns `None` if number of children corresponding to token BAR is less or equal than `i`.
fn BAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(BAR, i)
}

}

impl<'input> Or_operatorContextAttrs<'input> for Or_operatorContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn or_operator(&mut self,)
	-> Result<Rc<Or_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Or_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_or_operator);
        let mut _localctx: Rc<Or_operatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(172);
			recog.base.match_token(BAR,&mut recog.err_handler)?;

			recog.base.set_state(173);
			recog.base.match_token(BAR,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- and_operator ----------------
pub type And_operatorContextAll<'input> = And_operatorContext<'input>;


pub type And_operatorContext<'input> = BaseParserRuleContext<'input,And_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct And_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for And_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for And_operatorContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_and_operator(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_and_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for And_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_and_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for And_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_and_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_and_operator }
}
antlr_rust::tid!{And_operatorContextExt<'a>}

impl<'input> And_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<And_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,And_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait And_operatorContextAttrs<'input>: tacContext<'input> + BorrowMut<And_operatorContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token AMPERSAND in current rule
fn AMPERSAND_all(&self) -> Vec<Rc<TerminalNode<'input,tacContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AMPERSAND, starting from 0.
/// Returns `None` if number of children corresponding to token AMPERSAND is less or equal than `i`.
fn AMPERSAND(&self, i: usize) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(AMPERSAND, i)
}

}

impl<'input> And_operatorContextAttrs<'input> for And_operatorContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn and_operator(&mut self,)
	-> Result<Rc<And_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = And_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_and_operator);
        let mut _localctx: Rc<And_operatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(175);
			recog.base.match_token(AMPERSAND,&mut recog.err_handler)?;

			recog.base.set_state(176);
			recog.base.match_token(AMPERSAND,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- equals_operator ----------------
pub type Equals_operatorContextAll<'input> = Equals_operatorContext<'input>;


pub type Equals_operatorContext<'input> = BaseParserRuleContext<'input,Equals_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Equals_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Equals_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Equals_operatorContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equals_operator(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_equals_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Equals_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_equals_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for Equals_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_equals_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equals_operator }
}
antlr_rust::tid!{Equals_operatorContextExt<'a>}

impl<'input> Equals_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Equals_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Equals_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Equals_operatorContextAttrs<'input>: tacContext<'input> + BorrowMut<Equals_operatorContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token EQUALS in current rule
fn EQUALS_all(&self) -> Vec<Rc<TerminalNode<'input,tacContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EQUALS, starting from 0.
/// Returns `None` if number of children corresponding to token EQUALS is less or equal than `i`.
fn EQUALS(&self, i: usize) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(EQUALS, i)
}

}

impl<'input> Equals_operatorContextAttrs<'input> for Equals_operatorContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equals_operator(&mut self,)
	-> Result<Rc<Equals_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Equals_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_equals_operator);
        let mut _localctx: Rc<Equals_operatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(178);
			recog.base.match_token(EQUALS,&mut recog.err_handler)?;

			recog.base.set_state(179);
			recog.base.match_token(EQUALS,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- operand ----------------
pub type OperandContextAll<'input> = OperandContext<'input>;


pub type OperandContext<'input> = BaseParserRuleContext<'input,OperandContextExt<'input>>;

#[derive(Clone)]
pub struct OperandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for OperandContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for OperandContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_operand(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_operand(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for OperandContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_operand(self);
	}
}

impl<'input> CustomRuleContext<'input> for OperandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_operand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operand }
}
antlr_rust::tid!{OperandContextExt<'a>}

impl<'input> OperandContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperandContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OperandContextAttrs<'input>: tacContext<'input> + BorrowMut<OperandContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_method_identifier(&self) -> Option<Rc<Class_method_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OperandContextAttrs<'input> for OperandContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operand(&mut self,)
	-> Result<Rc<OperandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_operand);
        let mut _localctx: Rc<OperandContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(186);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(14,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(181);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(182);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(183);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule function_call*/
					recog.base.set_state(184);
					recog.function_call()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule class_method_identifier*/
					recog.base.set_state(185);
					recog.class_method_identifier()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
//------------------- function_call ----------------
pub type Function_callContextAll<'input> = Function_callContext<'input>;


pub type Function_callContext<'input> = BaseParserRuleContext<'input,Function_callContextExt<'input>>;

#[derive(Clone)]
pub struct Function_callContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Function_callContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Function_callContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_call(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_function_call(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Function_callContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_function_call(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_callContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_function_call }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_call }
}
antlr_rust::tid!{Function_callContextExt<'a>}

impl<'input> Function_callContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_callContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_callContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_callContextAttrs<'input>: tacContext<'input> + BorrowMut<Function_callContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SIZEOF
/// Returns `None` if there is no child corresponding to token SIZEOF
fn SIZEOF(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(SIZEOF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SQRT
/// Returns `None` if there is no child corresponding to token SQRT
fn SQRT(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(SQRT, 0)
}
/// Retrieves first TerminalNode corresponding to token PUSH
/// Returns `None` if there is no child corresponding to token PUSH
fn PUSH(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(PUSH, 0)
}
/// Retrieves first TerminalNode corresponding to token POP
/// Returns `None` if there is no child corresponding to token POP
fn POP(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(POP, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token PRINT
/// Returns `None` if there is no child corresponding to token PRINT
fn PRINT(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(PRINT, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn parameter_list(&self) -> Option<Rc<Parameter_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BREAK
/// Returns `None` if there is no child corresponding to token BREAK
fn BREAK(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(BREAK, 0)
}

}

impl<'input> Function_callContextAttrs<'input> for Function_callContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_call(&mut self,)
	-> Result<Rc<Function_callContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_function_call);
        let mut _localctx: Rc<Function_callContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(203);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SIZEOF 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(188);
					recog.base.match_token(SIZEOF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(189);
					recog.expression_rec(0)?;

					}
				}

			 SQRT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(190);
					recog.base.match_token(SQRT,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(191);
					recog.expression_rec(0)?;

					}
				}

			 PUSH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(192);
					recog.base.match_token(PUSH,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(193);
					recog.expression_rec(0)?;

					}
				}

			 POP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(194);
					recog.base.match_token(POP,&mut recog.err_handler)?;

					recog.base.set_state(195);
					_la = recog.base.input.la(1);
					if { !(_la==NUMBER || _la==IDENTIFIER) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

			 PRINT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(196);
					recog.base.match_token(PRINT,&mut recog.err_handler)?;

					recog.base.set_state(197);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					recog.base.set_state(200);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(198);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule parameter_list*/
							recog.base.set_state(199);
							recog.parameter_list()?;

							}
						}

						_ => {}
					}
					}
				}

			 BREAK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(202);
					recog.base.match_token(BREAK,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
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
//------------------- parameter_list ----------------
pub type Parameter_listContextAll<'input> = Parameter_listContext<'input>;


pub type Parameter_listContext<'input> = BaseParserRuleContext<'input,Parameter_listContextExt<'input>>;

#[derive(Clone)]
pub struct Parameter_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Parameter_listContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Parameter_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameter_list(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_parameter_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Parameter_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_parameter_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parameter_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter_list }
}
antlr_rust::tid!{Parameter_listContextExt<'a>}

impl<'input> Parameter_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Parameter_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parameter_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Parameter_listContextAttrs<'input>: tacContext<'input> + BorrowMut<Parameter_listContextExt<'input>>{

fn parameter(&self) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn parameter_list(&self) -> Option<Rc<Parameter_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Parameter_listContextAttrs<'input> for Parameter_listContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter_list(&mut self,)
	-> Result<Rc<Parameter_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parameter_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_parameter_list);
        let mut _localctx: Rc<Parameter_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(210);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule parameter*/
					recog.base.set_state(205);
					recog.parameter()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule parameter*/
					recog.base.set_state(206);
					recog.parameter()?;

					recog.base.set_state(207);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule parameter_list*/
					recog.base.set_state(208);
					recog.parameter_list()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
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
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for ParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameter(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_parameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for ParameterContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::tid!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: tacContext<'input> + BorrowMut<ParameterContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(212);
			recog.expression_rec(0)?;

			}
			Ok(())
		})();
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
//------------------- control_flow ----------------
pub type Control_flowContextAll<'input> = Control_flowContext<'input>;


pub type Control_flowContext<'input> = BaseParserRuleContext<'input,Control_flowContextExt<'input>>;

#[derive(Clone)]
pub struct Control_flowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Control_flowContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Control_flowContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_control_flow(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_control_flow(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Control_flowContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_control_flow(self);
	}
}

impl<'input> CustomRuleContext<'input> for Control_flowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_control_flow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_control_flow }
}
antlr_rust::tid!{Control_flowContextExt<'a>}

impl<'input> Control_flowContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Control_flowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Control_flowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Control_flowContextAttrs<'input>: tacContext<'input> + BorrowMut<Control_flowContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GOTO
/// Returns `None` if there is no child corresponding to token GOTO
fn GOTO(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(GOTO, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_identifier(&self) -> Option<Rc<Function_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(CALL, 0)
}
/// Retrieves first TerminalNode corresponding to token LCALL
/// Returns `None` if there is no child corresponding to token LCALL
fn LCALL(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(LCALL, 0)
}
/// Retrieves first TerminalNode corresponding to token ACALL
/// Returns `None` if there is no child corresponding to token ACALL
fn ACALL(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(ACALL, 0)
}
fn class_method_identifier(&self) -> Option<Rc<Class_method_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Control_flowContextAttrs<'input> for Control_flowContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn control_flow(&mut self,)
	-> Result<Rc<Control_flowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Control_flowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_control_flow);
        let mut _localctx: Rc<Control_flowContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(227);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(215);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IF {
						{
						/*InvokeRule if_statement*/
						recog.base.set_state(214);
						recog.if_statement()?;

						}
					}

					recog.base.set_state(217);
					recog.base.match_token(GOTO,&mut recog.err_handler)?;

					recog.base.set_state(218);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(219);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ACALL) | (1usize << CALL) | (1usize << LCALL))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule function_identifier*/
					recog.base.set_state(220);
					recog.function_identifier()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(221);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ACALL) | (1usize << CALL) | (1usize << LCALL))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule class_method_identifier*/
					recog.base.set_state(222);
					recog.class_method_identifier()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(223);
					recog.base.match_token(RETURN,&mut recog.err_handler)?;

					recog.base.set_state(225);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule expression*/
							recog.base.set_state(224);
							recog.expression_rec(0)?;

							}
						}

						_ => {}
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
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
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for If_statementContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for If_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_if_statement(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_if_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for If_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_if_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for If_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_if_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_statement }
}
antlr_rust::tid!{If_statementContextExt<'a>}

impl<'input> If_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait If_statementContextAttrs<'input>: tacContext<'input> + BorrowMut<If_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
/// Retrieves first TerminalNode corresponding to token OPENING_BRACKET
/// Returns `None` if there is no child corresponding to token OPENING_BRACKET
fn OPENING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(OPENING_BRACKET, 0)
}
fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSEING_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSEING_BRACKET
fn CLOSEING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(CLOSEING_BRACKET, 0)
}

}

impl<'input> If_statementContextAttrs<'input> for If_statementContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_statement(&mut self,)
	-> Result<Rc<If_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_if_statement);
        let mut _localctx: Rc<If_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(229);
			recog.base.match_token(IF,&mut recog.err_handler)?;

			recog.base.set_state(230);
			recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

			/*InvokeRule predicate*/
			recog.base.set_state(231);
			recog.predicate()?;

			recog.base.set_state(232);
			recog.base.match_token(CLOSEING_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- function_identifier ----------------
pub type Function_identifierContextAll<'input> = Function_identifierContext<'input>;


pub type Function_identifierContext<'input> = BaseParserRuleContext<'input,Function_identifierContextExt<'input>>;

#[derive(Clone)]
pub struct Function_identifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Function_identifierContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Function_identifierContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_identifier(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_function_identifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Function_identifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_function_identifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_identifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_function_identifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_identifier }
}
antlr_rust::tid!{Function_identifierContextExt<'a>}

impl<'input> Function_identifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_identifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_identifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_identifierContextAttrs<'input>: tacContext<'input> + BorrowMut<Function_identifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Function_identifierContextAttrs<'input> for Function_identifierContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_identifier(&mut self,)
	-> Result<Rc<Function_identifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_identifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_function_identifier);
        let mut _localctx: Rc<Function_identifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(234);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- function_definition ----------------
pub type Function_definitionContextAll<'input> = Function_definitionContext<'input>;


pub type Function_definitionContext<'input> = BaseParserRuleContext<'input,Function_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Function_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Function_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Function_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_definition(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_function_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Function_definitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_function_definition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_function_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_definition }
}
antlr_rust::tid!{Function_definitionContextExt<'a>}

impl<'input> Function_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_definitionContextAttrs<'input>: tacContext<'input> + BorrowMut<Function_definitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BEGINFUNC
/// Returns `None` if there is no child corresponding to token BEGINFUNC
fn BEGINFUNC(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(BEGINFUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn function_body(&self) -> Option<Rc<Function_bodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ENDFUNC
/// Returns `None` if there is no child corresponding to token ENDFUNC
fn ENDFUNC(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(ENDFUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token NEWLINE
/// Returns `None` if there is no child corresponding to token NEWLINE
fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(NEWLINE, 0)
}

}

impl<'input> Function_definitionContextAttrs<'input> for Function_definitionContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_definition(&mut self,)
	-> Result<Rc<Function_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_function_definition);
        let mut _localctx: Rc<Function_definitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(236);
			recog.base.match_token(BEGINFUNC,&mut recog.err_handler)?;

			recog.base.set_state(237);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			/*InvokeRule function_body*/
			recog.base.set_state(238);
			recog.function_body()?;

			recog.base.set_state(240);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==NEWLINE {
				{
				recog.base.set_state(239);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(242);
			recog.base.match_token(ENDFUNC,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- function_body ----------------
pub type Function_bodyContextAll<'input> = Function_bodyContext<'input>;


pub type Function_bodyContext<'input> = BaseParserRuleContext<'input,Function_bodyContextExt<'input>>;

#[derive(Clone)]
pub struct Function_bodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Function_bodyContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Function_bodyContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_body(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_function_body(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Function_bodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_function_body(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_bodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_function_body }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_body }
}
antlr_rust::tid!{Function_bodyContextExt<'a>}

impl<'input> Function_bodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_bodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_bodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_bodyContextAttrs<'input>: tacContext<'input> + BorrowMut<Function_bodyContextExt<'input>>{

fn source_line_all(&self) ->  Vec<Rc<Source_lineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn source_line(&self, i: usize) -> Option<Rc<Source_lineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,tacContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> Function_bodyContextAttrs<'input> for Function_bodyContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_body(&mut self,)
	-> Result<Rc<Function_bodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_bodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_function_body);
        let mut _localctx: Rc<Function_bodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(247);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(244);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(249);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule source_line*/
			recog.base.set_state(250);
			recog.source_line()?;

			recog.base.set_state(260);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(24,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(254);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(251);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(256);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule source_line*/
					recog.base.set_state(257);
					recog.source_line()?;

					}
					} 
				}
				recog.base.set_state(262);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(24,&mut recog.base)?;
			}
			recog.base.set_state(266);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(263);
					recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(268);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
			}
			}
			Ok(())
		})();
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
//------------------- class_method_identifier ----------------
pub type Class_method_identifierContextAll<'input> = Class_method_identifierContext<'input>;


pub type Class_method_identifierContext<'input> = BaseParserRuleContext<'input,Class_method_identifierContextExt<'input>>;

#[derive(Clone)]
pub struct Class_method_identifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Class_method_identifierContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Class_method_identifierContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_class_method_identifier(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_class_method_identifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Class_method_identifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_class_method_identifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Class_method_identifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_class_method_identifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_class_method_identifier }
}
antlr_rust::tid!{Class_method_identifierContextExt<'a>}

impl<'input> Class_method_identifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Class_method_identifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Class_method_identifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Class_method_identifierContextAttrs<'input>: tacContext<'input> + BorrowMut<Class_method_identifierContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,tacContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}

}

impl<'input> Class_method_identifierContextAttrs<'input> for Class_method_identifierContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn class_method_identifier(&mut self,)
	-> Result<Rc<Class_method_identifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Class_method_identifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_class_method_identifier);
        let mut _localctx: Rc<Class_method_identifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(269);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(270);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(271);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
//------------------- vtable_assignment ----------------
pub type Vtable_assignmentContextAll<'input> = Vtable_assignmentContext<'input>;


pub type Vtable_assignmentContext<'input> = BaseParserRuleContext<'input,Vtable_assignmentContextExt<'input>>;

#[derive(Clone)]
pub struct Vtable_assignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Vtable_assignmentContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Vtable_assignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_vtable_assignment(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_vtable_assignment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Vtable_assignmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_vtable_assignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for Vtable_assignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_vtable_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_vtable_assignment }
}
antlr_rust::tid!{Vtable_assignmentContextExt<'a>}

impl<'input> Vtable_assignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Vtable_assignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Vtable_assignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Vtable_assignmentContextAttrs<'input>: tacContext<'input> + BorrowMut<Vtable_assignmentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VTABLE
/// Returns `None` if there is no child corresponding to token VTABLE
fn VTABLE(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(VTABLE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn class_method_list(&self) -> Option<Rc<Class_method_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Vtable_assignmentContextAttrs<'input> for Vtable_assignmentContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn vtable_assignment(&mut self,)
	-> Result<Rc<Vtable_assignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Vtable_assignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_vtable_assignment);
        let mut _localctx: Rc<Vtable_assignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(273);
			recog.base.match_token(VTABLE,&mut recog.err_handler)?;

			recog.base.set_state(274);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(275);
			recog.base.match_token(EQUALS,&mut recog.err_handler)?;

			/*InvokeRule class_method_list*/
			recog.base.set_state(276);
			recog.class_method_list_rec(0)?;

			}
			Ok(())
		})();
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
//------------------- class_method_list ----------------
pub type Class_method_listContextAll<'input> = Class_method_listContext<'input>;


pub type Class_method_listContext<'input> = BaseParserRuleContext<'input,Class_method_listContextExt<'input>>;

#[derive(Clone)]
pub struct Class_method_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tacContext<'input> for Class_method_listContext<'input>{}

impl<'input,'a> Listenable<dyn tacListener<'input> + 'a> for Class_method_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_class_method_list(self);
		}
		fn exit(&self,listener: &mut (dyn tacListener<'input> + 'a)) {
			listener.exit_class_method_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tacVisitor<'input> + 'a> for Class_method_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tacVisitor<'input> + 'a)) {
		visitor.visit_class_method_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Class_method_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tacContextType;
	fn get_rule_index(&self) -> usize { RULE_class_method_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_class_method_list }
}
antlr_rust::tid!{Class_method_listContextExt<'a>}

impl<'input> Class_method_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tacContext<'input> + 'input > >, invoking_state: isize) -> Rc<Class_method_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Class_method_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Class_method_listContextAttrs<'input>: tacContext<'input> + BorrowMut<Class_method_listContextExt<'input>>{

fn class_method_identifier(&self) -> Option<Rc<Class_method_identifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_method_list(&self) -> Option<Rc<Class_method_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,tacContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> Class_method_listContextAttrs<'input> for Class_method_listContext<'input>{}

impl<'input, I, H> tac<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  class_method_list(&mut self,)
	-> Result<Rc<Class_method_listContextAll<'input>>,ANTLRError> {
		self.class_method_list_rec(0)
	}

	fn class_method_list_rec(&mut self, _p: isize)
	-> Result<Rc<Class_method_listContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Class_method_listContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 48, RULE_class_method_list, _p);
	    let mut _localctx: Rc<Class_method_listContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 48;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule class_method_identifier*/
			recog.base.set_state(279);
			recog.class_method_identifier()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(286);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Class_method_listContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_class_method_list);
					_localctx = tmp;
					recog.base.set_state(281);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(282);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule class_method_identifier*/
					recog.base.set_state(283);
					recog.class_method_identifier()?;

					}
					} 
				}
				recog.base.set_state(288);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			}
			}
			Ok(())
		})();
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

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x38\u{124}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x03\x02\x07\x02\x36\
	\x0a\x02\x0c\x02\x0e\x02\x39\x0b\x02\x03\x02\x03\x02\x07\x02\x3d\x0a\x02\
	\x0c\x02\x0e\x02\x40\x0b\x02\x03\x02\x07\x02\x43\x0a\x02\x0c\x02\x0e\x02\
	\x46\x0b\x02\x03\x02\x07\x02\x49\x0a\x02\x0c\x02\x0e\x02\x4c\x0b\x02\x03\
	\x02\x03\x02\x03\x03\x03\x03\x03\x03\x05\x03\x53\x0a\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x05\x03\x59\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\
	\x03\x5f\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x05\x03\x6a\x0a\x03\x03\x04\x03\x04\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\x76\x0a\x05\x03\x06\
	\x03\x06\x03\x07\x03\x07\x03\x07\x05\x07\x7d\x0a\x07\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\
	\x08\u{8a}\x0a\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\
	\x03\x09\x03\x09\x05\x09\u{95}\x0a\x09\x03\x09\x03\x09\x03\x09\x03\x09\x07\
	\x09\u{9b}\x0a\x09\x0c\x09\x0e\x09\u{9e}\x0b\x09\x03\x0a\x03\x0a\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x05\x0b\u{ad}\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x05\
	\x0f\u{bd}\x0a\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{cb}\x0a\x10\x03\x10\x05\
	\x10\u{ce}\x0a\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x05\x11\u{d5}\
	\x0a\x11\x03\x12\x03\x12\x03\x13\x05\x13\u{da}\x0a\x13\x03\x13\x03\x13\x03\
	\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\u{e4}\x0a\x13\x05\x13\
	\u{e6}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\
	\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{f3}\x0a\x16\x03\x16\x03\x16\x03\x17\
	\x07\x17\u{f8}\x0a\x17\x0c\x17\x0e\x17\u{fb}\x0b\x17\x03\x17\x03\x17\x07\
	\x17\u{ff}\x0a\x17\x0c\x17\x0e\x17\u{102}\x0b\x17\x03\x17\x07\x17\u{105}\
	\x0a\x17\x0c\x17\x0e\x17\u{108}\x0b\x17\x03\x17\x07\x17\u{10b}\x0a\x17\x0c\
	\x17\x0e\x17\u{10e}\x0b\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x07\x1a\u{11f}\x0a\x1a\x0c\x1a\x0e\x1a\u{122}\x0b\x1a\x03\x1a\x02\x04\
	\x10\x32\x1b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\
	\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x02\x06\x04\x02\x14\x14\x2b\x2b\
	\x05\x02\x04\x05\x20\x20\x22\x22\x04\x02\x35\x35\x38\x38\x05\x02\x03\x03\
	\x0b\x0b\x1b\x1b\x02\u{140}\x02\x37\x03\x02\x02\x02\x04\x69\x03\x02\x02\
	\x02\x06\x6b\x03\x02\x02\x02\x08\x75\x03\x02\x02\x02\x0a\x77\x03\x02\x02\
	\x02\x0c\x79\x03\x02\x02\x02\x0e\u{89}\x03\x02\x02\x02\x10\u{94}\x03\x02\
	\x02\x02\x12\u{9f}\x03\x02\x02\x02\x14\u{ac}\x03\x02\x02\x02\x16\u{ae}\x03\
	\x02\x02\x02\x18\u{b1}\x03\x02\x02\x02\x1a\u{b4}\x03\x02\x02\x02\x1c\u{bc}\
	\x03\x02\x02\x02\x1e\u{cd}\x03\x02\x02\x02\x20\u{d4}\x03\x02\x02\x02\x22\
	\u{d6}\x03\x02\x02\x02\x24\u{e5}\x03\x02\x02\x02\x26\u{e7}\x03\x02\x02\x02\
	\x28\u{ec}\x03\x02\x02\x02\x2a\u{ee}\x03\x02\x02\x02\x2c\u{f9}\x03\x02\x02\
	\x02\x2e\u{10f}\x03\x02\x02\x02\x30\u{113}\x03\x02\x02\x02\x32\u{118}\x03\
	\x02\x02\x02\x34\x36\x07\x2d\x02\x02\x35\x34\x03\x02\x02\x02\x36\x39\x03\
	\x02\x02\x02\x37\x35\x03\x02\x02\x02\x37\x38\x03\x02\x02\x02\x38\x3a\x03\
	\x02\x02\x02\x39\x37\x03\x02\x02\x02\x3a\x44\x05\x04\x03\x02\x3b\x3d\x07\
	\x2d\x02\x02\x3c\x3b\x03\x02\x02\x02\x3d\x40\x03\x02\x02\x02\x3e\x3c\x03\
	\x02\x02\x02\x3e\x3f\x03\x02\x02\x02\x3f\x41\x03\x02\x02\x02\x40\x3e\x03\
	\x02\x02\x02\x41\x43\x05\x04\x03\x02\x42\x3e\x03\x02\x02\x02\x43\x46\x03\
	\x02\x02\x02\x44\x42\x03\x02\x02\x02\x44\x45\x03\x02\x02\x02\x45\x4a\x03\
	\x02\x02\x02\x46\x44\x03\x02\x02\x02\x47\x49\x07\x2d\x02\x02\x48\x47\x03\
	\x02\x02\x02\x49\x4c\x03\x02\x02\x02\x4a\x48\x03\x02\x02\x02\x4a\x4b\x03\
	\x02\x02\x02\x4b\x4d\x03\x02\x02\x02\x4c\x4a\x03\x02\x02\x02\x4d\x4e\x07\
	\x02\x02\x03\x4e\x03\x03\x02\x02\x02\x4f\x50\x05\x06\x04\x02\x50\x51\x07\
	\x0d\x02\x02\x51\x53\x03\x02\x02\x02\x52\x4f\x03\x02\x02\x02\x52\x53\x03\
	\x02\x02\x02\x53\x54\x03\x02\x02\x02\x54\x6a\x05\x08\x05\x02\x55\x56\x05\
	\x06\x04\x02\x56\x57\x07\x0d\x02\x02\x57\x59\x03\x02\x02\x02\x58\x55\x03\
	\x02\x02\x02\x58\x59\x03\x02\x02\x02\x59\x5a\x03\x02\x02\x02\x5a\x6a\x05\
	\x24\x13\x02\x5b\x5c\x05\x06\x04\x02\x5c\x5d\x07\x0d\x02\x02\x5d\x5f\x03\
	\x02\x02\x02\x5e\x5b\x03\x02\x02\x02\x5e\x5f\x03\x02\x02\x02\x5f\x60\x03\
	\x02\x02\x02\x60\x6a\x05\x1e\x10\x02\x61\x62\x05\x06\x04\x02\x62\x63\x07\
	\x0d\x02\x02\x63\x6a\x03\x02\x02\x02\x64\x65\x05\x2e\x18\x02\x65\x66\x07\
	\x0d\x02\x02\x66\x6a\x03\x02\x02\x02\x67\x6a\x05\x2a\x16\x02\x68\x6a\x05\
	\x30\x19\x02\x69\x52\x03\x02\x02\x02\x69\x58\x03\x02\x02\x02\x69\x5e\x03\
	\x02\x02\x02\x69\x61\x03\x02\x02\x02\x69\x64\x03\x02\x02\x02\x69\x67\x03\
	\x02\x02\x02\x69\x68\x03\x02\x02\x02\x6a\x05\x03\x02\x02\x02\x6b\x6c\x07\
	\x38\x02\x02\x6c\x07\x03\x02\x02\x02\x6d\x6e\x05\x0a\x06\x02\x6e\x6f\x07\
	\x13\x02\x02\x6f\x70\x05\x10\x09\x02\x70\x76\x03\x02\x02\x02\x71\x72\x05\
	\x0a\x06\x02\x72\x73\x07\x13\x02\x02\x73\x74\x05\x0c\x07\x02\x74\x76\x03\
	\x02\x02\x02\x75\x6d\x03\x02\x02\x02\x75\x71\x03\x02\x02\x02\x76\x09\x03\
	\x02\x02\x02\x77\x78\x05\x10\x09\x02\x78\x0b\x03\x02\x02\x02\x79\x7c\x07\
	\x1b\x02\x02\x7a\x7d\x05\x28\x15\x02\x7b\x7d\x05\x2e\x18\x02\x7c\x7a\x03\
	\x02\x02\x02\x7c\x7b\x03\x02\x02\x02\x7d\x0d\x03\x02\x02\x02\x7e\x7f\x05\
	\x10\x09\x02\x7f\u{80}\x05\x1a\x0e\x02\u{80}\u{81}\x09\x02\x02\x02\u{81}\
	\u{8a}\x03\x02\x02\x02\u{82}\u{83}\x09\x02\x02\x02\u{83}\u{84}\x05\x1a\x0e\
	\x02\u{84}\u{85}\x05\x10\x09\x02\u{85}\u{8a}\x03\x02\x02\x02\u{86}\u{8a}\
	\x05\x10\x09\x02\u{87}\u{8a}\x07\x2b\x02\x02\u{88}\u{8a}\x07\x14\x02\x02\
	\u{89}\x7e\x03\x02\x02\x02\u{89}\u{82}\x03\x02\x02\x02\u{89}\u{86}\x03\x02\
	\x02\x02\u{89}\u{87}\x03\x02\x02\x02\u{89}\u{88}\x03\x02\x02\x02\u{8a}\x0f\
	\x03\x02\x02\x02\u{8b}\u{8c}\x08\x09\x01\x02\u{8c}\u{8d}\x07\x21\x02\x02\
	\u{8d}\u{8e}\x05\x10\x09\x02\u{8e}\u{8f}\x07\x0c\x02\x02\u{8f}\u{95}\x03\
	\x02\x02\x02\u{90}\u{91}\x05\x12\x0a\x02\u{91}\u{92}\x05\x10\x09\x04\u{92}\
	\u{95}\x03\x02\x02\x02\u{93}\u{95}\x05\x1c\x0f\x02\u{94}\u{8b}\x03\x02\x02\
	\x02\u{94}\u{90}\x03\x02\x02\x02\u{94}\u{93}\x03\x02\x02\x02\u{95}\u{9c}\
	\x03\x02\x02\x02\u{96}\u{97}\x0c\x05\x02\x02\u{97}\u{98}\x05\x14\x0b\x02\
	\u{98}\u{99}\x05\x10\x09\x06\u{99}\u{9b}\x03\x02\x02\x02\u{9a}\u{96}\x03\
	\x02\x02\x02\u{9b}\u{9e}\x03\x02\x02\x02\u{9c}\u{9a}\x03\x02\x02\x02\u{9c}\
	\u{9d}\x03\x02\x02\x02\u{9d}\x11\x03\x02\x02\x02\u{9e}\u{9c}\x03\x02\x02\
	\x02\u{9f}\u{a0}\x09\x03\x02\x02\u{a0}\x13\x03\x02\x02\x02\u{a1}\u{ad}\x07\
	\x22\x02\x02\u{a2}\u{ad}\x07\x20\x02\x02\u{a3}\u{ad}\x07\x05\x02\x02\u{a4}\
	\u{ad}\x07\x2a\x02\x02\u{a5}\u{ad}\x05\x16\x0c\x02\u{a6}\u{ad}\x05\x18\x0d\
	\x02\u{a7}\u{ad}\x05\x1a\x0e\x02\u{a8}\u{ad}\x07\x1d\x02\x02\u{a9}\u{ad}\
	\x07\x16\x02\x02\u{aa}\u{ad}\x07\x1e\x02\x02\u{ab}\u{ad}\x07\x17\x02\x02\
	\u{ac}\u{a1}\x03\x02\x02\x02\u{ac}\u{a2}\x03\x02\x02\x02\u{ac}\u{a3}\x03\
	\x02\x02\x02\u{ac}\u{a4}\x03\x02\x02\x02\u{ac}\u{a5}\x03\x02\x02\x02\u{ac}\
	\u{a6}\x03\x02\x02\x02\u{ac}\u{a7}\x03\x02\x02\x02\u{ac}\u{a8}\x03\x02\x02\
	\x02\u{ac}\u{a9}\x03\x02\x02\x02\u{ac}\u{aa}\x03\x02\x02\x02\u{ac}\u{ab}\
	\x03\x02\x02\x02\u{ad}\x15\x03\x02\x02\x02\u{ae}\u{af}\x07\x08\x02\x02\u{af}\
	\u{b0}\x07\x08\x02\x02\u{b0}\x17\x03\x02\x02\x02\u{b1}\u{b2}\x07\x04\x02\
	\x02\u{b2}\u{b3}\x07\x04\x02\x02\u{b3}\x19\x03\x02\x02\x02\u{b4}\u{b5}\x07\
	\x13\x02\x02\u{b5}\u{b6}\x07\x13\x02\x02\u{b6}\x1b\x03\x02\x02\x02\u{b7}\
	\u{bd}\x07\x35\x02\x02\u{b8}\u{bd}\x07\x38\x02\x02\u{b9}\u{bd}\x07\x32\x02\
	\x02\u{ba}\u{bd}\x05\x1e\x10\x02\u{bb}\u{bd}\x05\x2e\x18\x02\u{bc}\u{b7}\
	\x03\x02\x02\x02\u{bc}\u{b8}\x03\x02\x02\x02\u{bc}\u{b9}\x03\x02\x02\x02\
	\u{bc}\u{ba}\x03\x02\x02\x02\u{bc}\u{bb}\x03\x02\x02\x02\u{bd}\x1d\x03\x02\
	\x02\x02\u{be}\u{bf}\x07\x28\x02\x02\u{bf}\u{ce}\x05\x10\x09\x02\u{c0}\u{c1}\
	\x07\x29\x02\x02\u{c1}\u{ce}\x05\x10\x09\x02\u{c2}\u{c3}\x07\x25\x02\x02\
	\u{c3}\u{ce}\x05\x10\x09\x02\u{c4}\u{c5}\x07\x23\x02\x02\u{c5}\u{ce}\x09\
	\x04\x02\x02\u{c6}\u{c7}\x07\x24\x02\x02\u{c7}\u{ca}\x07\x32\x02\x02\u{c8}\
	\u{c9}\x07\x0e\x02\x02\u{c9}\u{cb}\x05\x20\x11\x02\u{ca}\u{c8}\x03\x02\x02\
	\x02\u{ca}\u{cb}\x03\x02\x02\x02\u{cb}\u{ce}\x03\x02\x02\x02\u{cc}\u{ce}\
	\x07\x0a\x02\x02\u{cd}\u{be}\x03\x02\x02\x02\u{cd}\u{c0}\x03\x02\x02\x02\
	\u{cd}\u{c2}\x03\x02\x02\x02\u{cd}\u{c4}\x03\x02\x02\x02\u{cd}\u{c6}\x03\
	\x02\x02\x02\u{cd}\u{cc}\x03\x02\x02\x02\u{ce}\x1f\x03\x02\x02\x02\u{cf}\
	\u{d5}\x05\x22\x12\x02\u{d0}\u{d1}\x05\x22\x12\x02\u{d1}\u{d2}\x07\x0e\x02\
	\x02\u{d2}\u{d3}\x05\x20\x11\x02\u{d3}\u{d5}\x03\x02\x02\x02\u{d4}\u{cf}\
	\x03\x02\x02\x02\u{d4}\u{d0}\x03\x02\x02\x02\u{d5}\x21\x03\x02\x02\x02\u{d6}\
	\u{d7}\x05\x10\x09\x02\u{d7}\x23\x03\x02\x02\x02\u{d8}\u{da}\x05\x26\x14\
	\x02\u{d9}\u{d8}\x03\x02\x02\x02\u{d9}\u{da}\x03\x02\x02\x02\u{da}\u{db}\
	\x03\x02\x02\x02\u{db}\u{dc}\x07\x15\x02\x02\u{dc}\u{e6}\x07\x38\x02\x02\
	\u{dd}\u{de}\x09\x05\x02\x02\u{de}\u{e6}\x05\x28\x15\x02\u{df}\u{e0}\x09\
	\x05\x02\x02\u{e0}\u{e6}\x05\x2e\x18\x02\u{e1}\u{e3}\x07\x26\x02\x02\u{e2}\
	\u{e4}\x05\x10\x09\x02\u{e3}\u{e2}\x03\x02\x02\x02\u{e3}\u{e4}\x03\x02\x02\
	\x02\u{e4}\u{e6}\x03\x02\x02\x02\u{e5}\u{d9}\x03\x02\x02\x02\u{e5}\u{dd}\
	\x03\x02\x02\x02\u{e5}\u{df}\x03\x02\x02\x02\u{e5}\u{e1}\x03\x02\x02\x02\
	\u{e6}\x25\x03\x02\x02\x02\u{e7}\u{e8}\x07\x19\x02\x02\u{e8}\u{e9}\x07\x21\
	\x02\x02\u{e9}\u{ea}\x05\x0e\x08\x02\u{ea}\u{eb}\x07\x0c\x02\x02\u{eb}\x27\
	\x03\x02\x02\x02\u{ec}\u{ed}\x07\x38\x02\x02\u{ed}\x29\x03\x02\x02\x02\u{ee}\
	\u{ef}\x07\x09\x02\x02\u{ef}\u{f0}\x07\x38\x02\x02\u{f0}\u{f2}\x05\x2c\x17\
	\x02\u{f1}\u{f3}\x07\x2d\x02\x02\u{f2}\u{f1}\x03\x02\x02\x02\u{f2}\u{f3}\
	\x03\x02\x02\x02\u{f3}\u{f4}\x03\x02\x02\x02\u{f4}\u{f5}\x07\x12\x02\x02\
	\u{f5}\x2b\x03\x02\x02\x02\u{f6}\u{f8}\x07\x2d\x02\x02\u{f7}\u{f6}\x03\x02\
	\x02\x02\u{f8}\u{fb}\x03\x02\x02\x02\u{f9}\u{f7}\x03\x02\x02\x02\u{f9}\u{fa}\
	\x03\x02\x02\x02\u{fa}\u{fc}\x03\x02\x02\x02\u{fb}\u{f9}\x03\x02\x02\x02\
	\u{fc}\u{106}\x05\x04\x03\x02\u{fd}\u{ff}\x07\x2d\x02\x02\u{fe}\u{fd}\x03\
	\x02\x02\x02\u{ff}\u{102}\x03\x02\x02\x02\u{100}\u{fe}\x03\x02\x02\x02\u{100}\
	\u{101}\x03\x02\x02\x02\u{101}\u{103}\x03\x02\x02\x02\u{102}\u{100}\x03\
	\x02\x02\x02\u{103}\u{105}\x05\x04\x03\x02\u{104}\u{100}\x03\x02\x02\x02\
	\u{105}\u{108}\x03\x02\x02\x02\u{106}\u{104}\x03\x02\x02\x02\u{106}\u{107}\
	\x03\x02\x02\x02\u{107}\u{10c}\x03\x02\x02\x02\u{108}\u{106}\x03\x02\x02\
	\x02\u{109}\u{10b}\x07\x2d\x02\x02\u{10a}\u{109}\x03\x02\x02\x02\u{10b}\
	\u{10e}\x03\x02\x02\x02\u{10c}\u{10a}\x03\x02\x02\x02\u{10c}\u{10d}\x03\
	\x02\x02\x02\u{10d}\x2d\x03\x02\x02\x02\u{10e}\u{10c}\x03\x02\x02\x02\u{10f}\
	\u{110}\x07\x38\x02\x02\u{110}\u{111}\x07\x0f\x02\x02\u{111}\u{112}\x07\
	\x38\x02\x02\u{112}\x2f\x03\x02\x02\x02\u{113}\u{114}\x07\x2c\x02\x02\u{114}\
	\u{115}\x07\x38\x02\x02\u{115}\u{116}\x07\x13\x02\x02\u{116}\u{117}\x05\
	\x32\x1a\x02\u{117}\x31\x03\x02\x02\x02\u{118}\u{119}\x08\x1a\x01\x02\u{119}\
	\u{11a}\x05\x2e\x18\x02\u{11a}\u{120}\x03\x02\x02\x02\u{11b}\u{11c}\x0c\
	\x03\x02\x02\u{11c}\u{11d}\x07\x0e\x02\x02\u{11d}\u{11f}\x05\x2e\x18\x02\
	\u{11e}\u{11b}\x03\x02\x02\x02\u{11f}\u{122}\x03\x02\x02\x02\u{120}\u{11e}\
	\x03\x02\x02\x02\u{120}\u{121}\x03\x02\x02\x02\u{121}\x33\x03\x02\x02\x02\
	\u{122}\u{120}\x03\x02\x02\x02\x1d\x37\x3e\x44\x4a\x52\x58\x5e\x69\x75\x7c\
	\u{89}\u{94}\u{9c}\u{ac}\u{bc}\u{ca}\u{cd}\u{d4}\u{d9}\u{e3}\u{e5}\u{f2}\
	\u{f9}\u{100}\u{106}\u{10c}\u{120}";

