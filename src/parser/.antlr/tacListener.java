// Generated from c:/aaa_se/rust/three_address_code_runtime/src/parser/tac.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link tac}.
 */
public interface tacListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link tac#compilation_unit}.
	 * @param ctx the parse tree
	 */
	void enterCompilation_unit(tac.Compilation_unitContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#compilation_unit}.
	 * @param ctx the parse tree
	 */
	void exitCompilation_unit(tac.Compilation_unitContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#source_line}.
	 * @param ctx the parse tree
	 */
	void enterSource_line(tac.Source_lineContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#source_line}.
	 * @param ctx the parse tree
	 */
	void exitSource_line(tac.Source_lineContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#label}.
	 * @param ctx the parse tree
	 */
	void enterLabel(tac.LabelContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#label}.
	 * @param ctx the parse tree
	 */
	void exitLabel(tac.LabelContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#assignment}.
	 * @param ctx the parse tree
	 */
	void enterAssignment(tac.AssignmentContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#assignment}.
	 * @param ctx the parse tree
	 */
	void exitAssignment(tac.AssignmentContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#left_hand_side}.
	 * @param ctx the parse tree
	 */
	void enterLeft_hand_side(tac.Left_hand_sideContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#left_hand_side}.
	 * @param ctx the parse tree
	 */
	void exitLeft_hand_side(tac.Left_hand_sideContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#lcall_expression}.
	 * @param ctx the parse tree
	 */
	void enterLcall_expression(tac.Lcall_expressionContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#lcall_expression}.
	 * @param ctx the parse tree
	 */
	void exitLcall_expression(tac.Lcall_expressionContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#predicate}.
	 * @param ctx the parse tree
	 */
	void enterPredicate(tac.PredicateContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#predicate}.
	 * @param ctx the parse tree
	 */
	void exitPredicate(tac.PredicateContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#expression}.
	 * @param ctx the parse tree
	 */
	void enterExpression(tac.ExpressionContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#expression}.
	 * @param ctx the parse tree
	 */
	void exitExpression(tac.ExpressionContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#unary_operator}.
	 * @param ctx the parse tree
	 */
	void enterUnary_operator(tac.Unary_operatorContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#unary_operator}.
	 * @param ctx the parse tree
	 */
	void exitUnary_operator(tac.Unary_operatorContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#binary_operator}.
	 * @param ctx the parse tree
	 */
	void enterBinary_operator(tac.Binary_operatorContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#binary_operator}.
	 * @param ctx the parse tree
	 */
	void exitBinary_operator(tac.Binary_operatorContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#or_operator}.
	 * @param ctx the parse tree
	 */
	void enterOr_operator(tac.Or_operatorContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#or_operator}.
	 * @param ctx the parse tree
	 */
	void exitOr_operator(tac.Or_operatorContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#and_operator}.
	 * @param ctx the parse tree
	 */
	void enterAnd_operator(tac.And_operatorContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#and_operator}.
	 * @param ctx the parse tree
	 */
	void exitAnd_operator(tac.And_operatorContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#equals_operator}.
	 * @param ctx the parse tree
	 */
	void enterEquals_operator(tac.Equals_operatorContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#equals_operator}.
	 * @param ctx the parse tree
	 */
	void exitEquals_operator(tac.Equals_operatorContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#operand}.
	 * @param ctx the parse tree
	 */
	void enterOperand(tac.OperandContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#operand}.
	 * @param ctx the parse tree
	 */
	void exitOperand(tac.OperandContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#function_call}.
	 * @param ctx the parse tree
	 */
	void enterFunction_call(tac.Function_callContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#function_call}.
	 * @param ctx the parse tree
	 */
	void exitFunction_call(tac.Function_callContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#parameter_list}.
	 * @param ctx the parse tree
	 */
	void enterParameter_list(tac.Parameter_listContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#parameter_list}.
	 * @param ctx the parse tree
	 */
	void exitParameter_list(tac.Parameter_listContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#parameter}.
	 * @param ctx the parse tree
	 */
	void enterParameter(tac.ParameterContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#parameter}.
	 * @param ctx the parse tree
	 */
	void exitParameter(tac.ParameterContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#control_flow}.
	 * @param ctx the parse tree
	 */
	void enterControl_flow(tac.Control_flowContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#control_flow}.
	 * @param ctx the parse tree
	 */
	void exitControl_flow(tac.Control_flowContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#if_statement}.
	 * @param ctx the parse tree
	 */
	void enterIf_statement(tac.If_statementContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#if_statement}.
	 * @param ctx the parse tree
	 */
	void exitIf_statement(tac.If_statementContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#function_identifier}.
	 * @param ctx the parse tree
	 */
	void enterFunction_identifier(tac.Function_identifierContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#function_identifier}.
	 * @param ctx the parse tree
	 */
	void exitFunction_identifier(tac.Function_identifierContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#function_definition}.
	 * @param ctx the parse tree
	 */
	void enterFunction_definition(tac.Function_definitionContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#function_definition}.
	 * @param ctx the parse tree
	 */
	void exitFunction_definition(tac.Function_definitionContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#function_body}.
	 * @param ctx the parse tree
	 */
	void enterFunction_body(tac.Function_bodyContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#function_body}.
	 * @param ctx the parse tree
	 */
	void exitFunction_body(tac.Function_bodyContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#class_method_identifier}.
	 * @param ctx the parse tree
	 */
	void enterClass_method_identifier(tac.Class_method_identifierContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#class_method_identifier}.
	 * @param ctx the parse tree
	 */
	void exitClass_method_identifier(tac.Class_method_identifierContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#vtable_assignment}.
	 * @param ctx the parse tree
	 */
	void enterVtable_assignment(tac.Vtable_assignmentContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#vtable_assignment}.
	 * @param ctx the parse tree
	 */
	void exitVtable_assignment(tac.Vtable_assignmentContext ctx);
	/**
	 * Enter a parse tree produced by {@link tac#class_method_list}.
	 * @param ctx the parse tree
	 */
	void enterClass_method_list(tac.Class_method_listContext ctx);
	/**
	 * Exit a parse tree produced by {@link tac#class_method_list}.
	 * @param ctx the parse tree
	 */
	void exitClass_method_list(tac.Class_method_listContext ctx);
}