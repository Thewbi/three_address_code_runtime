// Generated from c:/aaa_se/rust/three_address_code_runtime/src/parser/tac.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class tac extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		ACALL=1, AMPERSAND=2, ASTERISK=3, AT=4, BACKSLASH=5, BAR=6, BEGINFUNC=7, 
		BREAK=8, CALL=9, CLOSEING_BRACKET=10, COLON=11, COMMA=12, DOT=13, ELSE=14, 
		ENDIF=15, ENDFUNC=16, EQUALS=17, FALSE=18, GOTO=19, GT=20, GTE=21, HASH_TAG=22, 
		IF=23, INCLUDE=24, LCALL=25, LEFT_SHIFT=26, LT=27, LTE=28, MACRO=29, MINUS=30, 
		OPENING_BRACKET=31, PLUS=32, POP=33, PRINT=34, PUSH=35, RETURN=36, RIGHT_SHIFT=37, 
		SIZEOF=38, SQRT=39, SLASH=40, TRUE=41, VTABLE=42, NEWLINE=43, WS=44, LINE_COMMENT=45, 
		BLOCK_COMMENT=46, DOUBLE_SLASH_LINE_COMMENT=47, STRING=48, CHAR=49, ESCAPED_CHAR=50, 
		NUMBER=51, HEX_NUMBER=52, BINARY_NUMBER=53, IDENTIFIER=54;
	public static final int
		RULE_compilation_unit = 0, RULE_source_line = 1, RULE_label = 2, RULE_assignment = 3, 
		RULE_left_hand_side = 4, RULE_lcall_expression = 5, RULE_predicate = 6, 
		RULE_expression = 7, RULE_unary_operator = 8, RULE_binary_operator = 9, 
		RULE_or_operator = 10, RULE_and_operator = 11, RULE_equals_operator = 12, 
		RULE_operand = 13, RULE_function_call = 14, RULE_parameter_list = 15, 
		RULE_parameter = 16, RULE_control_flow = 17, RULE_if_statement = 18, RULE_function_identifier = 19, 
		RULE_function_definition = 20, RULE_function_body = 21, RULE_class_method_identifier = 22, 
		RULE_vtable_assignment = 23, RULE_class_method_list = 24;
	private static String[] makeRuleNames() {
		return new String[] {
			"compilation_unit", "source_line", "label", "assignment", "left_hand_side", 
			"lcall_expression", "predicate", "expression", "unary_operator", "binary_operator", 
			"or_operator", "and_operator", "equals_operator", "operand", "function_call", 
			"parameter_list", "parameter", "control_flow", "if_statement", "function_identifier", 
			"function_definition", "function_body", "class_method_identifier", "vtable_assignment", 
			"class_method_list"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, "'&'", "'*'", "'@'", "'\\'", "'|'", null, null, null, "')'", 
			"':'", "','", "'.'", "'else'", "'endif'", null, "'='", null, null, "'>'", 
			"'>='", "'#'", "'if'", "'include'", null, "'<<'", "'<'", "'<='", "'macro'", 
			"'-'", "'('", "'+'", null, null, null, null, "'>>'", null, null, "'/'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "ACALL", "AMPERSAND", "ASTERISK", "AT", "BACKSLASH", "BAR", "BEGINFUNC", 
			"BREAK", "CALL", "CLOSEING_BRACKET", "COLON", "COMMA", "DOT", "ELSE", 
			"ENDIF", "ENDFUNC", "EQUALS", "FALSE", "GOTO", "GT", "GTE", "HASH_TAG", 
			"IF", "INCLUDE", "LCALL", "LEFT_SHIFT", "LT", "LTE", "MACRO", "MINUS", 
			"OPENING_BRACKET", "PLUS", "POP", "PRINT", "PUSH", "RETURN", "RIGHT_SHIFT", 
			"SIZEOF", "SQRT", "SLASH", "TRUE", "VTABLE", "NEWLINE", "WS", "LINE_COMMENT", 
			"BLOCK_COMMENT", "DOUBLE_SLASH_LINE_COMMENT", "STRING", "CHAR", "ESCAPED_CHAR", 
			"NUMBER", "HEX_NUMBER", "BINARY_NUMBER", "IDENTIFIER"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "tac.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public tac(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Compilation_unitContext extends ParserRuleContext {
		public List<Source_lineContext> source_line() {
			return getRuleContexts(Source_lineContext.class);
		}
		public Source_lineContext source_line(int i) {
			return getRuleContext(Source_lineContext.class,i);
		}
		public Compilation_unitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_compilation_unit; }
	}

	public final Compilation_unitContext compilation_unit() throws RecognitionException {
		Compilation_unitContext _localctx = new Compilation_unitContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_compilation_unit);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(53);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 18019752228684682L) != 0)) {
				{
				{
				setState(50);
				source_line();
				}
				}
				setState(55);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Source_lineContext extends ParserRuleContext {
		public AssignmentContext assignment() {
			return getRuleContext(AssignmentContext.class,0);
		}
		public LabelContext label() {
			return getRuleContext(LabelContext.class,0);
		}
		public TerminalNode COLON() { return getToken(tac.COLON, 0); }
		public Control_flowContext control_flow() {
			return getRuleContext(Control_flowContext.class,0);
		}
		public Function_callContext function_call() {
			return getRuleContext(Function_callContext.class,0);
		}
		public Class_method_identifierContext class_method_identifier() {
			return getRuleContext(Class_method_identifierContext.class,0);
		}
		public Function_definitionContext function_definition() {
			return getRuleContext(Function_definitionContext.class,0);
		}
		public Vtable_assignmentContext vtable_assignment() {
			return getRuleContext(Vtable_assignmentContext.class,0);
		}
		public Source_lineContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_source_line; }
	}

	public final Source_lineContext source_line() throws RecognitionException {
		Source_lineContext _localctx = new Source_lineContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_source_line);
		int _la;
		try {
			setState(82);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,4,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(59);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,1,_ctx) ) {
				case 1:
					{
					setState(56);
					label();
					setState(57);
					match(COLON);
					}
					break;
				}
				setState(61);
				assignment();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(65);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==IDENTIFIER) {
					{
					setState(62);
					label();
					setState(63);
					match(COLON);
					}
				}

				setState(67);
				control_flow();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(71);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==IDENTIFIER) {
					{
					setState(68);
					label();
					setState(69);
					match(COLON);
					}
				}

				setState(73);
				function_call();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				{
				setState(74);
				label();
				setState(75);
				match(COLON);
				}
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				{
				setState(77);
				class_method_identifier();
				setState(78);
				match(COLON);
				}
				}
				break;
			case 6:
				enterOuterAlt(_localctx, 6);
				{
				setState(80);
				function_definition();
				}
				break;
			case 7:
				enterOuterAlt(_localctx, 7);
				{
				setState(81);
				vtable_assignment();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LabelContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public LabelContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_label; }
	}

	public final LabelContext label() throws RecognitionException {
		LabelContext _localctx = new LabelContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_label);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(84);
			match(IDENTIFIER);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AssignmentContext extends ParserRuleContext {
		public Left_hand_sideContext left_hand_side() {
			return getRuleContext(Left_hand_sideContext.class,0);
		}
		public TerminalNode EQUALS() { return getToken(tac.EQUALS, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public Lcall_expressionContext lcall_expression() {
			return getRuleContext(Lcall_expressionContext.class,0);
		}
		public AssignmentContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_assignment; }
	}

	public final AssignmentContext assignment() throws RecognitionException {
		AssignmentContext _localctx = new AssignmentContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_assignment);
		try {
			setState(94);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,5,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(86);
				left_hand_side();
				setState(87);
				match(EQUALS);
				setState(88);
				expression();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(90);
				left_hand_side();
				setState(91);
				match(EQUALS);
				setState(92);
				lcall_expression();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Left_hand_sideContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public TerminalNode ASTERISK() { return getToken(tac.ASTERISK, 0); }
		public TerminalNode OPENING_BRACKET() { return getToken(tac.OPENING_BRACKET, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode CLOSEING_BRACKET() { return getToken(tac.CLOSEING_BRACKET, 0); }
		public Left_hand_sideContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_left_hand_side; }
	}

	public final Left_hand_sideContext left_hand_side() throws RecognitionException {
		Left_hand_sideContext _localctx = new Left_hand_sideContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_left_hand_side);
		int _la;
		try {
			setState(107);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,8,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(97);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==ASTERISK) {
					{
					setState(96);
					match(ASTERISK);
					}
				}

				setState(99);
				match(IDENTIFIER);
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(101);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==ASTERISK) {
					{
					setState(100);
					match(ASTERISK);
					}
				}

				setState(103);
				match(OPENING_BRACKET);
				setState(104);
				expression();
				setState(105);
				match(CLOSEING_BRACKET);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Lcall_expressionContext extends ParserRuleContext {
		public TerminalNode LCALL() { return getToken(tac.LCALL, 0); }
		public Function_identifierContext function_identifier() {
			return getRuleContext(Function_identifierContext.class,0);
		}
		public Class_method_identifierContext class_method_identifier() {
			return getRuleContext(Class_method_identifierContext.class,0);
		}
		public Lcall_expressionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_lcall_expression; }
	}

	public final Lcall_expressionContext lcall_expression() throws RecognitionException {
		Lcall_expressionContext _localctx = new Lcall_expressionContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_lcall_expression);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(109);
			match(LCALL);
			setState(112);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				{
				setState(110);
				function_identifier();
				}
				break;
			case 2:
				{
				setState(111);
				class_method_identifier();
				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PredicateContext extends ParserRuleContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public Equals_operatorContext equals_operator() {
			return getRuleContext(Equals_operatorContext.class,0);
		}
		public TerminalNode TRUE() { return getToken(tac.TRUE, 0); }
		public TerminalNode FALSE() { return getToken(tac.FALSE, 0); }
		public PredicateContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_predicate; }
	}

	public final PredicateContext predicate() throws RecognitionException {
		PredicateContext _localctx = new PredicateContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_predicate);
		int _la;
		try {
			setState(125);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,10,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(114);
				expression();
				setState(115);
				equals_operator();
				setState(116);
				_la = _input.LA(1);
				if ( !(_la==FALSE || _la==TRUE) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(118);
				_la = _input.LA(1);
				if ( !(_la==FALSE || _la==TRUE) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(119);
				equals_operator();
				setState(120);
				expression();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(122);
				expression();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(123);
				match(TRUE);
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				setState(124);
				match(FALSE);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExpressionContext extends ParserRuleContext {
		public List<OperandContext> operand() {
			return getRuleContexts(OperandContext.class);
		}
		public OperandContext operand(int i) {
			return getRuleContext(OperandContext.class,i);
		}
		public Binary_operatorContext binary_operator() {
			return getRuleContext(Binary_operatorContext.class,0);
		}
		public Unary_operatorContext unary_operator() {
			return getRuleContext(Unary_operatorContext.class,0);
		}
		public ExpressionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expression; }
	}

	public final ExpressionContext expression() throws RecognitionException {
		ExpressionContext _localctx = new ExpressionContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_expression);
		try {
			setState(135);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,11,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(127);
				operand();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(128);
				operand();
				setState(129);
				binary_operator();
				setState(130);
				operand();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(132);
				unary_operator();
				setState(133);
				operand();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Unary_operatorContext extends ParserRuleContext {
		public TerminalNode AMPERSAND() { return getToken(tac.AMPERSAND, 0); }
		public TerminalNode ASTERISK() { return getToken(tac.ASTERISK, 0); }
		public TerminalNode MINUS() { return getToken(tac.MINUS, 0); }
		public TerminalNode PLUS() { return getToken(tac.PLUS, 0); }
		public Unary_operatorContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_unary_operator; }
	}

	public final Unary_operatorContext unary_operator() throws RecognitionException {
		Unary_operatorContext _localctx = new Unary_operatorContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_unary_operator);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(137);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 5368709132L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Binary_operatorContext extends ParserRuleContext {
		public TerminalNode PLUS() { return getToken(tac.PLUS, 0); }
		public TerminalNode MINUS() { return getToken(tac.MINUS, 0); }
		public TerminalNode ASTERISK() { return getToken(tac.ASTERISK, 0); }
		public TerminalNode SLASH() { return getToken(tac.SLASH, 0); }
		public Or_operatorContext or_operator() {
			return getRuleContext(Or_operatorContext.class,0);
		}
		public And_operatorContext and_operator() {
			return getRuleContext(And_operatorContext.class,0);
		}
		public Equals_operatorContext equals_operator() {
			return getRuleContext(Equals_operatorContext.class,0);
		}
		public TerminalNode LT() { return getToken(tac.LT, 0); }
		public TerminalNode GT() { return getToken(tac.GT, 0); }
		public TerminalNode LTE() { return getToken(tac.LTE, 0); }
		public TerminalNode GTE() { return getToken(tac.GTE, 0); }
		public Binary_operatorContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_binary_operator; }
	}

	public final Binary_operatorContext binary_operator() throws RecognitionException {
		Binary_operatorContext _localctx = new Binary_operatorContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_binary_operator);
		try {
			setState(150);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case PLUS:
				enterOuterAlt(_localctx, 1);
				{
				setState(139);
				match(PLUS);
				}
				break;
			case MINUS:
				enterOuterAlt(_localctx, 2);
				{
				setState(140);
				match(MINUS);
				}
				break;
			case ASTERISK:
				enterOuterAlt(_localctx, 3);
				{
				setState(141);
				match(ASTERISK);
				}
				break;
			case SLASH:
				enterOuterAlt(_localctx, 4);
				{
				setState(142);
				match(SLASH);
				}
				break;
			case BAR:
				enterOuterAlt(_localctx, 5);
				{
				setState(143);
				or_operator();
				}
				break;
			case AMPERSAND:
				enterOuterAlt(_localctx, 6);
				{
				setState(144);
				and_operator();
				}
				break;
			case EQUALS:
				enterOuterAlt(_localctx, 7);
				{
				setState(145);
				equals_operator();
				}
				break;
			case LT:
				enterOuterAlt(_localctx, 8);
				{
				setState(146);
				match(LT);
				}
				break;
			case GT:
				enterOuterAlt(_localctx, 9);
				{
				setState(147);
				match(GT);
				}
				break;
			case LTE:
				enterOuterAlt(_localctx, 10);
				{
				setState(148);
				match(LTE);
				}
				break;
			case GTE:
				enterOuterAlt(_localctx, 11);
				{
				setState(149);
				match(GTE);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Or_operatorContext extends ParserRuleContext {
		public List<TerminalNode> BAR() { return getTokens(tac.BAR); }
		public TerminalNode BAR(int i) {
			return getToken(tac.BAR, i);
		}
		public Or_operatorContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_or_operator; }
	}

	public final Or_operatorContext or_operator() throws RecognitionException {
		Or_operatorContext _localctx = new Or_operatorContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_or_operator);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(152);
			match(BAR);
			setState(153);
			match(BAR);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class And_operatorContext extends ParserRuleContext {
		public List<TerminalNode> AMPERSAND() { return getTokens(tac.AMPERSAND); }
		public TerminalNode AMPERSAND(int i) {
			return getToken(tac.AMPERSAND, i);
		}
		public And_operatorContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_and_operator; }
	}

	public final And_operatorContext and_operator() throws RecognitionException {
		And_operatorContext _localctx = new And_operatorContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_and_operator);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(155);
			match(AMPERSAND);
			setState(156);
			match(AMPERSAND);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Equals_operatorContext extends ParserRuleContext {
		public List<TerminalNode> EQUALS() { return getTokens(tac.EQUALS); }
		public TerminalNode EQUALS(int i) {
			return getToken(tac.EQUALS, i);
		}
		public Equals_operatorContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_equals_operator; }
	}

	public final Equals_operatorContext equals_operator() throws RecognitionException {
		Equals_operatorContext _localctx = new Equals_operatorContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_equals_operator);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(158);
			match(EQUALS);
			setState(159);
			match(EQUALS);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class OperandContext extends ParserRuleContext {
		public TerminalNode NUMBER() { return getToken(tac.NUMBER, 0); }
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public TerminalNode STRING() { return getToken(tac.STRING, 0); }
		public Function_callContext function_call() {
			return getRuleContext(Function_callContext.class,0);
		}
		public Class_method_identifierContext class_method_identifier() {
			return getRuleContext(Class_method_identifierContext.class,0);
		}
		public TerminalNode OPENING_BRACKET() { return getToken(tac.OPENING_BRACKET, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode CLOSEING_BRACKET() { return getToken(tac.CLOSEING_BRACKET, 0); }
		public OperandContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_operand; }
	}

	public final OperandContext operand() throws RecognitionException {
		OperandContext _localctx = new OperandContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_operand);
		try {
			setState(170);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,13,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(161);
				match(NUMBER);
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(162);
				match(IDENTIFIER);
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(163);
				match(STRING);
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(164);
				function_call();
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				setState(165);
				class_method_identifier();
				}
				break;
			case 6:
				enterOuterAlt(_localctx, 6);
				{
				setState(166);
				match(OPENING_BRACKET);
				setState(167);
				expression();
				setState(168);
				match(CLOSEING_BRACKET);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Function_callContext extends ParserRuleContext {
		public TerminalNode SIZEOF() { return getToken(tac.SIZEOF, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode SQRT() { return getToken(tac.SQRT, 0); }
		public TerminalNode PUSH() { return getToken(tac.PUSH, 0); }
		public TerminalNode POP() { return getToken(tac.POP, 0); }
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public TerminalNode NUMBER() { return getToken(tac.NUMBER, 0); }
		public TerminalNode PRINT() { return getToken(tac.PRINT, 0); }
		public TerminalNode STRING() { return getToken(tac.STRING, 0); }
		public TerminalNode COMMA() { return getToken(tac.COMMA, 0); }
		public Parameter_listContext parameter_list() {
			return getRuleContext(Parameter_listContext.class,0);
		}
		public TerminalNode BREAK() { return getToken(tac.BREAK, 0); }
		public Function_callContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_function_call; }
	}

	public final Function_callContext function_call() throws RecognitionException {
		Function_callContext _localctx = new Function_callContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_function_call);
		int _la;
		try {
			setState(187);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case SIZEOF:
				enterOuterAlt(_localctx, 1);
				{
				setState(172);
				match(SIZEOF);
				setState(173);
				expression();
				}
				break;
			case SQRT:
				enterOuterAlt(_localctx, 2);
				{
				setState(174);
				match(SQRT);
				setState(175);
				expression();
				}
				break;
			case PUSH:
				enterOuterAlt(_localctx, 3);
				{
				setState(176);
				match(PUSH);
				setState(177);
				expression();
				}
				break;
			case POP:
				enterOuterAlt(_localctx, 4);
				{
				setState(178);
				match(POP);
				setState(179);
				_la = _input.LA(1);
				if ( !(_la==NUMBER || _la==IDENTIFIER) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				}
				break;
			case PRINT:
				enterOuterAlt(_localctx, 5);
				{
				setState(180);
				match(PRINT);
				setState(181);
				match(STRING);
				setState(184);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,14,_ctx) ) {
				case 1:
					{
					setState(182);
					match(COMMA);
					setState(183);
					parameter_list(0);
					}
					break;
				}
				}
				break;
			case BREAK:
				enterOuterAlt(_localctx, 6);
				{
				setState(186);
				match(BREAK);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Parameter_listContext extends ParserRuleContext {
		public ParameterContext parameter() {
			return getRuleContext(ParameterContext.class,0);
		}
		public Parameter_listContext parameter_list() {
			return getRuleContext(Parameter_listContext.class,0);
		}
		public TerminalNode COMMA() { return getToken(tac.COMMA, 0); }
		public Parameter_listContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_parameter_list; }
	}

	public final Parameter_listContext parameter_list() throws RecognitionException {
		return parameter_list(0);
	}

	private Parameter_listContext parameter_list(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		Parameter_listContext _localctx = new Parameter_listContext(_ctx, _parentState);
		Parameter_listContext _prevctx = _localctx;
		int _startState = 30;
		enterRecursionRule(_localctx, 30, RULE_parameter_list, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			{
			setState(190);
			parameter();
			}
			_ctx.stop = _input.LT(-1);
			setState(197);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,16,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					{
					_localctx = new Parameter_listContext(_parentctx, _parentState);
					pushNewRecursionContext(_localctx, _startState, RULE_parameter_list);
					setState(192);
					if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
					setState(193);
					match(COMMA);
					setState(194);
					parameter();
					}
					} 
				}
				setState(199);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,16,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ParameterContext extends ParserRuleContext {
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public ParameterContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_parameter; }
	}

	public final ParameterContext parameter() throws RecognitionException {
		ParameterContext _localctx = new ParameterContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_parameter);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(200);
			expression();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Control_flowContext extends ParserRuleContext {
		public TerminalNode GOTO() { return getToken(tac.GOTO, 0); }
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public If_statementContext if_statement() {
			return getRuleContext(If_statementContext.class,0);
		}
		public Function_identifierContext function_identifier() {
			return getRuleContext(Function_identifierContext.class,0);
		}
		public TerminalNode CALL() { return getToken(tac.CALL, 0); }
		public TerminalNode LCALL() { return getToken(tac.LCALL, 0); }
		public TerminalNode ACALL() { return getToken(tac.ACALL, 0); }
		public Class_method_identifierContext class_method_identifier() {
			return getRuleContext(Class_method_identifierContext.class,0);
		}
		public TerminalNode RETURN() { return getToken(tac.RETURN, 0); }
		public Control_flowContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_control_flow; }
	}

	public final Control_flowContext control_flow() throws RecognitionException {
		Control_flowContext _localctx = new Control_flowContext(_ctx, getState());
		enterRule(_localctx, 34, RULE_control_flow);
		int _la;
		try {
			setState(212);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,18,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(203);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==IF) {
					{
					setState(202);
					if_statement();
					}
				}

				setState(205);
				match(GOTO);
				setState(206);
				match(IDENTIFIER);
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(207);
				_la = _input.LA(1);
				if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 33554946L) != 0)) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(208);
				function_identifier();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(209);
				_la = _input.LA(1);
				if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 33554946L) != 0)) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(210);
				class_method_identifier();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(211);
				match(RETURN);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class If_statementContext extends ParserRuleContext {
		public TerminalNode IF() { return getToken(tac.IF, 0); }
		public TerminalNode OPENING_BRACKET() { return getToken(tac.OPENING_BRACKET, 0); }
		public PredicateContext predicate() {
			return getRuleContext(PredicateContext.class,0);
		}
		public TerminalNode CLOSEING_BRACKET() { return getToken(tac.CLOSEING_BRACKET, 0); }
		public If_statementContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_if_statement; }
	}

	public final If_statementContext if_statement() throws RecognitionException {
		If_statementContext _localctx = new If_statementContext(_ctx, getState());
		enterRule(_localctx, 36, RULE_if_statement);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(214);
			match(IF);
			setState(215);
			match(OPENING_BRACKET);
			setState(216);
			predicate();
			setState(217);
			match(CLOSEING_BRACKET);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Function_identifierContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public Function_identifierContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_function_identifier; }
	}

	public final Function_identifierContext function_identifier() throws RecognitionException {
		Function_identifierContext _localctx = new Function_identifierContext(_ctx, getState());
		enterRule(_localctx, 38, RULE_function_identifier);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(219);
			match(IDENTIFIER);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Function_definitionContext extends ParserRuleContext {
		public TerminalNode BEGINFUNC() { return getToken(tac.BEGINFUNC, 0); }
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public Function_bodyContext function_body() {
			return getRuleContext(Function_bodyContext.class,0);
		}
		public TerminalNode ENDFUNC() { return getToken(tac.ENDFUNC, 0); }
		public Function_definitionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_function_definition; }
	}

	public final Function_definitionContext function_definition() throws RecognitionException {
		Function_definitionContext _localctx = new Function_definitionContext(_ctx, getState());
		enterRule(_localctx, 40, RULE_function_definition);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(221);
			match(BEGINFUNC);
			setState(222);
			match(IDENTIFIER);
			setState(223);
			function_body();
			setState(224);
			match(ENDFUNC);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Function_bodyContext extends ParserRuleContext {
		public List<Source_lineContext> source_line() {
			return getRuleContexts(Source_lineContext.class);
		}
		public Source_lineContext source_line(int i) {
			return getRuleContext(Source_lineContext.class,i);
		}
		public Function_bodyContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_function_body; }
	}

	public final Function_bodyContext function_body() throws RecognitionException {
		Function_bodyContext _localctx = new Function_bodyContext(_ctx, getState());
		enterRule(_localctx, 42, RULE_function_body);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(227); 
			_errHandler.sync(this);
			_la = _input.LA(1);
			do {
				{
				{
				setState(226);
				source_line();
				}
				}
				setState(229); 
				_errHandler.sync(this);
				_la = _input.LA(1);
			} while ( (((_la) & ~0x3f) == 0 && ((1L << _la) & 18019752228684682L) != 0) );
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Class_method_identifierContext extends ParserRuleContext {
		public List<TerminalNode> IDENTIFIER() { return getTokens(tac.IDENTIFIER); }
		public TerminalNode IDENTIFIER(int i) {
			return getToken(tac.IDENTIFIER, i);
		}
		public TerminalNode DOT() { return getToken(tac.DOT, 0); }
		public Class_method_identifierContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_class_method_identifier; }
	}

	public final Class_method_identifierContext class_method_identifier() throws RecognitionException {
		Class_method_identifierContext _localctx = new Class_method_identifierContext(_ctx, getState());
		enterRule(_localctx, 44, RULE_class_method_identifier);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(231);
			match(IDENTIFIER);
			setState(232);
			match(DOT);
			setState(233);
			match(IDENTIFIER);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Vtable_assignmentContext extends ParserRuleContext {
		public TerminalNode VTABLE() { return getToken(tac.VTABLE, 0); }
		public TerminalNode IDENTIFIER() { return getToken(tac.IDENTIFIER, 0); }
		public TerminalNode EQUALS() { return getToken(tac.EQUALS, 0); }
		public Class_method_listContext class_method_list() {
			return getRuleContext(Class_method_listContext.class,0);
		}
		public Vtable_assignmentContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_vtable_assignment; }
	}

	public final Vtable_assignmentContext vtable_assignment() throws RecognitionException {
		Vtable_assignmentContext _localctx = new Vtable_assignmentContext(_ctx, getState());
		enterRule(_localctx, 46, RULE_vtable_assignment);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(235);
			match(VTABLE);
			setState(236);
			match(IDENTIFIER);
			setState(237);
			match(EQUALS);
			setState(238);
			class_method_list(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Class_method_listContext extends ParserRuleContext {
		public Class_method_identifierContext class_method_identifier() {
			return getRuleContext(Class_method_identifierContext.class,0);
		}
		public Class_method_listContext class_method_list() {
			return getRuleContext(Class_method_listContext.class,0);
		}
		public TerminalNode COMMA() { return getToken(tac.COMMA, 0); }
		public Class_method_listContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_class_method_list; }
	}

	public final Class_method_listContext class_method_list() throws RecognitionException {
		return class_method_list(0);
	}

	private Class_method_listContext class_method_list(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		Class_method_listContext _localctx = new Class_method_listContext(_ctx, _parentState);
		Class_method_listContext _prevctx = _localctx;
		int _startState = 48;
		enterRecursionRule(_localctx, 48, RULE_class_method_list, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			{
			setState(241);
			class_method_identifier();
			}
			_ctx.stop = _input.LT(-1);
			setState(248);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,20,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					{
					_localctx = new Class_method_listContext(_parentctx, _parentState);
					pushNewRecursionContext(_localctx, _startState, RULE_class_method_list);
					setState(243);
					if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
					setState(244);
					match(COMMA);
					setState(245);
					class_method_identifier();
					}
					} 
				}
				setState(250);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,20,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 15:
			return parameter_list_sempred((Parameter_listContext)_localctx, predIndex);
		case 24:
			return class_method_list_sempred((Class_method_listContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean parameter_list_sempred(Parameter_listContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 1);
		}
		return true;
	}
	private boolean class_method_list_sempred(Class_method_listContext _localctx, int predIndex) {
		switch (predIndex) {
		case 1:
			return precpred(_ctx, 1);
		}
		return true;
	}

	public static final String _serializedATN =
		"\u0004\u00016\u00fc\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001\u0002"+
		"\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004\u0002"+
		"\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007\u0002"+
		"\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b\u0002"+
		"\f\u0007\f\u0002\r\u0007\r\u0002\u000e\u0007\u000e\u0002\u000f\u0007\u000f"+
		"\u0002\u0010\u0007\u0010\u0002\u0011\u0007\u0011\u0002\u0012\u0007\u0012"+
		"\u0002\u0013\u0007\u0013\u0002\u0014\u0007\u0014\u0002\u0015\u0007\u0015"+
		"\u0002\u0016\u0007\u0016\u0002\u0017\u0007\u0017\u0002\u0018\u0007\u0018"+
		"\u0001\u0000\u0005\u00004\b\u0000\n\u0000\f\u00007\t\u0000\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0003\u0001<\b\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0003\u0001B\b\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0003\u0001H\b\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0003\u0001S\b\u0001\u0001\u0002\u0001\u0002\u0001\u0003"+
		"\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0003\u0001\u0003"+
		"\u0001\u0003\u0003\u0003_\b\u0003\u0001\u0004\u0003\u0004b\b\u0004\u0001"+
		"\u0004\u0001\u0004\u0003\u0004f\b\u0004\u0001\u0004\u0001\u0004\u0001"+
		"\u0004\u0001\u0004\u0003\u0004l\b\u0004\u0001\u0005\u0001\u0005\u0001"+
		"\u0005\u0003\u0005q\b\u0005\u0001\u0006\u0001\u0006\u0001\u0006\u0001"+
		"\u0006\u0001\u0006\u0001\u0006\u0001\u0006\u0001\u0006\u0001\u0006\u0001"+
		"\u0006\u0001\u0006\u0003\u0006~\b\u0006\u0001\u0007\u0001\u0007\u0001"+
		"\u0007\u0001\u0007\u0001\u0007\u0001\u0007\u0001\u0007\u0001\u0007\u0003"+
		"\u0007\u0088\b\u0007\u0001\b\u0001\b\u0001\t\u0001\t\u0001\t\u0001\t\u0001"+
		"\t\u0001\t\u0001\t\u0001\t\u0001\t\u0001\t\u0001\t\u0003\t\u0097\b\t\u0001"+
		"\n\u0001\n\u0001\n\u0001\u000b\u0001\u000b\u0001\u000b\u0001\f\u0001\f"+
		"\u0001\f\u0001\r\u0001\r\u0001\r\u0001\r\u0001\r\u0001\r\u0001\r\u0001"+
		"\r\u0001\r\u0003\r\u00ab\b\r\u0001\u000e\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0003\u000e\u00b9\b\u000e\u0001\u000e\u0003"+
		"\u000e\u00bc\b\u000e\u0001\u000f\u0001\u000f\u0001\u000f\u0001\u000f\u0001"+
		"\u000f\u0001\u000f\u0005\u000f\u00c4\b\u000f\n\u000f\f\u000f\u00c7\t\u000f"+
		"\u0001\u0010\u0001\u0010\u0001\u0011\u0003\u0011\u00cc\b\u0011\u0001\u0011"+
		"\u0001\u0011\u0001\u0011\u0001\u0011\u0001\u0011\u0001\u0011\u0001\u0011"+
		"\u0003\u0011\u00d5\b\u0011\u0001\u0012\u0001\u0012\u0001\u0012\u0001\u0012"+
		"\u0001\u0012\u0001\u0013\u0001\u0013\u0001\u0014\u0001\u0014\u0001\u0014"+
		"\u0001\u0014\u0001\u0014\u0001\u0015\u0004\u0015\u00e4\b\u0015\u000b\u0015"+
		"\f\u0015\u00e5\u0001\u0016\u0001\u0016\u0001\u0016\u0001\u0016\u0001\u0017"+
		"\u0001\u0017\u0001\u0017\u0001\u0017\u0001\u0017\u0001\u0018\u0001\u0018"+
		"\u0001\u0018\u0001\u0018\u0001\u0018\u0001\u0018\u0005\u0018\u00f7\b\u0018"+
		"\n\u0018\f\u0018\u00fa\t\u0018\u0001\u0018\u0000\u0002\u001e0\u0019\u0000"+
		"\u0002\u0004\u0006\b\n\f\u000e\u0010\u0012\u0014\u0016\u0018\u001a\u001c"+
		"\u001e \"$&(*,.0\u0000\u0004\u0002\u0000\u0012\u0012))\u0003\u0000\u0002"+
		"\u0003\u001e\u001e  \u0002\u00003366\u0003\u0000\u0001\u0001\t\t\u0019"+
		"\u0019\u0113\u00005\u0001\u0000\u0000\u0000\u0002R\u0001\u0000\u0000\u0000"+
		"\u0004T\u0001\u0000\u0000\u0000\u0006^\u0001\u0000\u0000\u0000\bk\u0001"+
		"\u0000\u0000\u0000\nm\u0001\u0000\u0000\u0000\f}\u0001\u0000\u0000\u0000"+
		"\u000e\u0087\u0001\u0000\u0000\u0000\u0010\u0089\u0001\u0000\u0000\u0000"+
		"\u0012\u0096\u0001\u0000\u0000\u0000\u0014\u0098\u0001\u0000\u0000\u0000"+
		"\u0016\u009b\u0001\u0000\u0000\u0000\u0018\u009e\u0001\u0000\u0000\u0000"+
		"\u001a\u00aa\u0001\u0000\u0000\u0000\u001c\u00bb\u0001\u0000\u0000\u0000"+
		"\u001e\u00bd\u0001\u0000\u0000\u0000 \u00c8\u0001\u0000\u0000\u0000\""+
		"\u00d4\u0001\u0000\u0000\u0000$\u00d6\u0001\u0000\u0000\u0000&\u00db\u0001"+
		"\u0000\u0000\u0000(\u00dd\u0001\u0000\u0000\u0000*\u00e3\u0001\u0000\u0000"+
		"\u0000,\u00e7\u0001\u0000\u0000\u0000.\u00eb\u0001\u0000\u0000\u00000"+
		"\u00f0\u0001\u0000\u0000\u000024\u0003\u0002\u0001\u000032\u0001\u0000"+
		"\u0000\u000047\u0001\u0000\u0000\u000053\u0001\u0000\u0000\u000056\u0001"+
		"\u0000\u0000\u00006\u0001\u0001\u0000\u0000\u000075\u0001\u0000\u0000"+
		"\u000089\u0003\u0004\u0002\u00009:\u0005\u000b\u0000\u0000:<\u0001\u0000"+
		"\u0000\u0000;8\u0001\u0000\u0000\u0000;<\u0001\u0000\u0000\u0000<=\u0001"+
		"\u0000\u0000\u0000=S\u0003\u0006\u0003\u0000>?\u0003\u0004\u0002\u0000"+
		"?@\u0005\u000b\u0000\u0000@B\u0001\u0000\u0000\u0000A>\u0001\u0000\u0000"+
		"\u0000AB\u0001\u0000\u0000\u0000BC\u0001\u0000\u0000\u0000CS\u0003\"\u0011"+
		"\u0000DE\u0003\u0004\u0002\u0000EF\u0005\u000b\u0000\u0000FH\u0001\u0000"+
		"\u0000\u0000GD\u0001\u0000\u0000\u0000GH\u0001\u0000\u0000\u0000HI\u0001"+
		"\u0000\u0000\u0000IS\u0003\u001c\u000e\u0000JK\u0003\u0004\u0002\u0000"+
		"KL\u0005\u000b\u0000\u0000LS\u0001\u0000\u0000\u0000MN\u0003,\u0016\u0000"+
		"NO\u0005\u000b\u0000\u0000OS\u0001\u0000\u0000\u0000PS\u0003(\u0014\u0000"+
		"QS\u0003.\u0017\u0000R;\u0001\u0000\u0000\u0000RA\u0001\u0000\u0000\u0000"+
		"RG\u0001\u0000\u0000\u0000RJ\u0001\u0000\u0000\u0000RM\u0001\u0000\u0000"+
		"\u0000RP\u0001\u0000\u0000\u0000RQ\u0001\u0000\u0000\u0000S\u0003\u0001"+
		"\u0000\u0000\u0000TU\u00056\u0000\u0000U\u0005\u0001\u0000\u0000\u0000"+
		"VW\u0003\b\u0004\u0000WX\u0005\u0011\u0000\u0000XY\u0003\u000e\u0007\u0000"+
		"Y_\u0001\u0000\u0000\u0000Z[\u0003\b\u0004\u0000[\\\u0005\u0011\u0000"+
		"\u0000\\]\u0003\n\u0005\u0000]_\u0001\u0000\u0000\u0000^V\u0001\u0000"+
		"\u0000\u0000^Z\u0001\u0000\u0000\u0000_\u0007\u0001\u0000\u0000\u0000"+
		"`b\u0005\u0003\u0000\u0000a`\u0001\u0000\u0000\u0000ab\u0001\u0000\u0000"+
		"\u0000bc\u0001\u0000\u0000\u0000cl\u00056\u0000\u0000df\u0005\u0003\u0000"+
		"\u0000ed\u0001\u0000\u0000\u0000ef\u0001\u0000\u0000\u0000fg\u0001\u0000"+
		"\u0000\u0000gh\u0005\u001f\u0000\u0000hi\u0003\u000e\u0007\u0000ij\u0005"+
		"\n\u0000\u0000jl\u0001\u0000\u0000\u0000ka\u0001\u0000\u0000\u0000ke\u0001"+
		"\u0000\u0000\u0000l\t\u0001\u0000\u0000\u0000mp\u0005\u0019\u0000\u0000"+
		"nq\u0003&\u0013\u0000oq\u0003,\u0016\u0000pn\u0001\u0000\u0000\u0000p"+
		"o\u0001\u0000\u0000\u0000q\u000b\u0001\u0000\u0000\u0000rs\u0003\u000e"+
		"\u0007\u0000st\u0003\u0018\f\u0000tu\u0007\u0000\u0000\u0000u~\u0001\u0000"+
		"\u0000\u0000vw\u0007\u0000\u0000\u0000wx\u0003\u0018\f\u0000xy\u0003\u000e"+
		"\u0007\u0000y~\u0001\u0000\u0000\u0000z~\u0003\u000e\u0007\u0000{~\u0005"+
		")\u0000\u0000|~\u0005\u0012\u0000\u0000}r\u0001\u0000\u0000\u0000}v\u0001"+
		"\u0000\u0000\u0000}z\u0001\u0000\u0000\u0000}{\u0001\u0000\u0000\u0000"+
		"}|\u0001\u0000\u0000\u0000~\r\u0001\u0000\u0000\u0000\u007f\u0088\u0003"+
		"\u001a\r\u0000\u0080\u0081\u0003\u001a\r\u0000\u0081\u0082\u0003\u0012"+
		"\t\u0000\u0082\u0083\u0003\u001a\r\u0000\u0083\u0088\u0001\u0000\u0000"+
		"\u0000\u0084\u0085\u0003\u0010\b\u0000\u0085\u0086\u0003\u001a\r\u0000"+
		"\u0086\u0088\u0001\u0000\u0000\u0000\u0087\u007f\u0001\u0000\u0000\u0000"+
		"\u0087\u0080\u0001\u0000\u0000\u0000\u0087\u0084\u0001\u0000\u0000\u0000"+
		"\u0088\u000f\u0001\u0000\u0000\u0000\u0089\u008a\u0007\u0001\u0000\u0000"+
		"\u008a\u0011\u0001\u0000\u0000\u0000\u008b\u0097\u0005 \u0000\u0000\u008c"+
		"\u0097\u0005\u001e\u0000\u0000\u008d\u0097\u0005\u0003\u0000\u0000\u008e"+
		"\u0097\u0005(\u0000\u0000\u008f\u0097\u0003\u0014\n\u0000\u0090\u0097"+
		"\u0003\u0016\u000b\u0000\u0091\u0097\u0003\u0018\f\u0000\u0092\u0097\u0005"+
		"\u001b\u0000\u0000\u0093\u0097\u0005\u0014\u0000\u0000\u0094\u0097\u0005"+
		"\u001c\u0000\u0000\u0095\u0097\u0005\u0015\u0000\u0000\u0096\u008b\u0001"+
		"\u0000\u0000\u0000\u0096\u008c\u0001\u0000\u0000\u0000\u0096\u008d\u0001"+
		"\u0000\u0000\u0000\u0096\u008e\u0001\u0000\u0000\u0000\u0096\u008f\u0001"+
		"\u0000\u0000\u0000\u0096\u0090\u0001\u0000\u0000\u0000\u0096\u0091\u0001"+
		"\u0000\u0000\u0000\u0096\u0092\u0001\u0000\u0000\u0000\u0096\u0093\u0001"+
		"\u0000\u0000\u0000\u0096\u0094\u0001\u0000\u0000\u0000\u0096\u0095\u0001"+
		"\u0000\u0000\u0000\u0097\u0013\u0001\u0000\u0000\u0000\u0098\u0099\u0005"+
		"\u0006\u0000\u0000\u0099\u009a\u0005\u0006\u0000\u0000\u009a\u0015\u0001"+
		"\u0000\u0000\u0000\u009b\u009c\u0005\u0002\u0000\u0000\u009c\u009d\u0005"+
		"\u0002\u0000\u0000\u009d\u0017\u0001\u0000\u0000\u0000\u009e\u009f\u0005"+
		"\u0011\u0000\u0000\u009f\u00a0\u0005\u0011\u0000\u0000\u00a0\u0019\u0001"+
		"\u0000\u0000\u0000\u00a1\u00ab\u00053\u0000\u0000\u00a2\u00ab\u00056\u0000"+
		"\u0000\u00a3\u00ab\u00050\u0000\u0000\u00a4\u00ab\u0003\u001c\u000e\u0000"+
		"\u00a5\u00ab\u0003,\u0016\u0000\u00a6\u00a7\u0005\u001f\u0000\u0000\u00a7"+
		"\u00a8\u0003\u000e\u0007\u0000\u00a8\u00a9\u0005\n\u0000\u0000\u00a9\u00ab"+
		"\u0001\u0000\u0000\u0000\u00aa\u00a1\u0001\u0000\u0000\u0000\u00aa\u00a2"+
		"\u0001\u0000\u0000\u0000\u00aa\u00a3\u0001\u0000\u0000\u0000\u00aa\u00a4"+
		"\u0001\u0000\u0000\u0000\u00aa\u00a5\u0001\u0000\u0000\u0000\u00aa\u00a6"+
		"\u0001\u0000\u0000\u0000\u00ab\u001b\u0001\u0000\u0000\u0000\u00ac\u00ad"+
		"\u0005&\u0000\u0000\u00ad\u00bc\u0003\u000e\u0007\u0000\u00ae\u00af\u0005"+
		"\'\u0000\u0000\u00af\u00bc\u0003\u000e\u0007\u0000\u00b0\u00b1\u0005#"+
		"\u0000\u0000\u00b1\u00bc\u0003\u000e\u0007\u0000\u00b2\u00b3\u0005!\u0000"+
		"\u0000\u00b3\u00bc\u0007\u0002\u0000\u0000\u00b4\u00b5\u0005\"\u0000\u0000"+
		"\u00b5\u00b8\u00050\u0000\u0000\u00b6\u00b7\u0005\f\u0000\u0000\u00b7"+
		"\u00b9\u0003\u001e\u000f\u0000\u00b8\u00b6\u0001\u0000\u0000\u0000\u00b8"+
		"\u00b9\u0001\u0000\u0000\u0000\u00b9\u00bc\u0001\u0000\u0000\u0000\u00ba"+
		"\u00bc\u0005\b\u0000\u0000\u00bb\u00ac\u0001\u0000\u0000\u0000\u00bb\u00ae"+
		"\u0001\u0000\u0000\u0000\u00bb\u00b0\u0001\u0000\u0000\u0000\u00bb\u00b2"+
		"\u0001\u0000\u0000\u0000\u00bb\u00b4\u0001\u0000\u0000\u0000\u00bb\u00ba"+
		"\u0001\u0000\u0000\u0000\u00bc\u001d\u0001\u0000\u0000\u0000\u00bd\u00be"+
		"\u0006\u000f\uffff\uffff\u0000\u00be\u00bf\u0003 \u0010\u0000\u00bf\u00c5"+
		"\u0001\u0000\u0000\u0000\u00c0\u00c1\n\u0001\u0000\u0000\u00c1\u00c2\u0005"+
		"\f\u0000\u0000\u00c2\u00c4\u0003 \u0010\u0000\u00c3\u00c0\u0001\u0000"+
		"\u0000\u0000\u00c4\u00c7\u0001\u0000\u0000\u0000\u00c5\u00c3\u0001\u0000"+
		"\u0000\u0000\u00c5\u00c6\u0001\u0000\u0000\u0000\u00c6\u001f\u0001\u0000"+
		"\u0000\u0000\u00c7\u00c5\u0001\u0000\u0000\u0000\u00c8\u00c9\u0003\u000e"+
		"\u0007\u0000\u00c9!\u0001\u0000\u0000\u0000\u00ca\u00cc\u0003$\u0012\u0000"+
		"\u00cb\u00ca\u0001\u0000\u0000\u0000\u00cb\u00cc\u0001\u0000\u0000\u0000"+
		"\u00cc\u00cd\u0001\u0000\u0000\u0000\u00cd\u00ce\u0005\u0013\u0000\u0000"+
		"\u00ce\u00d5\u00056\u0000\u0000\u00cf\u00d0\u0007\u0003\u0000\u0000\u00d0"+
		"\u00d5\u0003&\u0013\u0000\u00d1\u00d2\u0007\u0003\u0000\u0000\u00d2\u00d5"+
		"\u0003,\u0016\u0000\u00d3\u00d5\u0005$\u0000\u0000\u00d4\u00cb\u0001\u0000"+
		"\u0000\u0000\u00d4\u00cf\u0001\u0000\u0000\u0000\u00d4\u00d1\u0001\u0000"+
		"\u0000\u0000\u00d4\u00d3\u0001\u0000\u0000\u0000\u00d5#\u0001\u0000\u0000"+
		"\u0000\u00d6\u00d7\u0005\u0017\u0000\u0000\u00d7\u00d8\u0005\u001f\u0000"+
		"\u0000\u00d8\u00d9\u0003\f\u0006\u0000\u00d9\u00da\u0005\n\u0000\u0000"+
		"\u00da%\u0001\u0000\u0000\u0000\u00db\u00dc\u00056\u0000\u0000\u00dc\'"+
		"\u0001\u0000\u0000\u0000\u00dd\u00de\u0005\u0007\u0000\u0000\u00de\u00df"+
		"\u00056\u0000\u0000\u00df\u00e0\u0003*\u0015\u0000\u00e0\u00e1\u0005\u0010"+
		"\u0000\u0000\u00e1)\u0001\u0000\u0000\u0000\u00e2\u00e4\u0003\u0002\u0001"+
		"\u0000\u00e3\u00e2\u0001\u0000\u0000\u0000\u00e4\u00e5\u0001\u0000\u0000"+
		"\u0000\u00e5\u00e3\u0001\u0000\u0000\u0000\u00e5\u00e6\u0001\u0000\u0000"+
		"\u0000\u00e6+\u0001\u0000\u0000\u0000\u00e7\u00e8\u00056\u0000\u0000\u00e8"+
		"\u00e9\u0005\r\u0000\u0000\u00e9\u00ea\u00056\u0000\u0000\u00ea-\u0001"+
		"\u0000\u0000\u0000\u00eb\u00ec\u0005*\u0000\u0000\u00ec\u00ed\u00056\u0000"+
		"\u0000\u00ed\u00ee\u0005\u0011\u0000\u0000\u00ee\u00ef\u00030\u0018\u0000"+
		"\u00ef/\u0001\u0000\u0000\u0000\u00f0\u00f1\u0006\u0018\uffff\uffff\u0000"+
		"\u00f1\u00f2\u0003,\u0016\u0000\u00f2\u00f8\u0001\u0000\u0000\u0000\u00f3"+
		"\u00f4\n\u0001\u0000\u0000\u00f4\u00f5\u0005\f\u0000\u0000\u00f5\u00f7"+
		"\u0003,\u0016\u0000\u00f6\u00f3\u0001\u0000\u0000\u0000\u00f7\u00fa\u0001"+
		"\u0000\u0000\u0000\u00f8\u00f6\u0001\u0000\u0000\u0000\u00f8\u00f9\u0001"+
		"\u0000\u0000\u0000\u00f91\u0001\u0000\u0000\u0000\u00fa\u00f8\u0001\u0000"+
		"\u0000\u0000\u00155;AGR^aekp}\u0087\u0096\u00aa\u00b8\u00bb\u00c5\u00cb"+
		"\u00d4\u00e5\u00f8";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}