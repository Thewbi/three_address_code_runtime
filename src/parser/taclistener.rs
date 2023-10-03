#![allow(nonstandard_style)]
// Generated from ../../src/parser/tac.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
//use super::tac::*;
use super::tacparser::*;
use crate::parser::tacparser::tacContextType;

pub trait tacListener<'input> : ParseTreeListener<'input,tacContextType>{
/**
 * Enter a parse tree produced by {@link tac#compilation_unit}.
 * @param ctx the parse tree
 */
fn enter_compilation_unit(&mut self, _ctx: &Compilation_unitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#compilation_unit}.
 * @param ctx the parse tree
 */
fn exit_compilation_unit(&mut self, _ctx: &Compilation_unitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#source_line}.
 * @param ctx the parse tree
 */
fn enter_source_line(&mut self, _ctx: &Source_lineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#source_line}.
 * @param ctx the parse tree
 */
fn exit_source_line(&mut self, _ctx: &Source_lineContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#label}.
 * @param ctx the parse tree
 */
fn enter_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#label}.
 * @param ctx the parse tree
 */
fn exit_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#left_hand_side}.
 * @param ctx the parse tree
 */
fn enter_left_hand_side(&mut self, _ctx: &Left_hand_sideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#left_hand_side}.
 * @param ctx the parse tree
 */
fn exit_left_hand_side(&mut self, _ctx: &Left_hand_sideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#lcall_expression}.
 * @param ctx the parse tree
 */
fn enter_lcall_expression(&mut self, _ctx: &Lcall_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#lcall_expression}.
 * @param ctx the parse tree
 */
fn exit_lcall_expression(&mut self, _ctx: &Lcall_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#unary_operator}.
 * @param ctx the parse tree
 */
fn enter_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#unary_operator}.
 * @param ctx the parse tree
 */
fn exit_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#binary_operator}.
 * @param ctx the parse tree
 */
fn enter_binary_operator(&mut self, _ctx: &Binary_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#binary_operator}.
 * @param ctx the parse tree
 */
fn exit_binary_operator(&mut self, _ctx: &Binary_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#or_operator}.
 * @param ctx the parse tree
 */
fn enter_or_operator(&mut self, _ctx: &Or_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#or_operator}.
 * @param ctx the parse tree
 */
fn exit_or_operator(&mut self, _ctx: &Or_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#and_operator}.
 * @param ctx the parse tree
 */
fn enter_and_operator(&mut self, _ctx: &And_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#and_operator}.
 * @param ctx the parse tree
 */
fn exit_and_operator(&mut self, _ctx: &And_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#equals_operator}.
 * @param ctx the parse tree
 */
fn enter_equals_operator(&mut self, _ctx: &Equals_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#equals_operator}.
 * @param ctx the parse tree
 */
fn exit_equals_operator(&mut self, _ctx: &Equals_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#operand}.
 * @param ctx the parse tree
 */
fn enter_operand(&mut self, _ctx: &OperandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#operand}.
 * @param ctx the parse tree
 */
fn exit_operand(&mut self, _ctx: &OperandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#function_call}.
 * @param ctx the parse tree
 */
fn enter_function_call(&mut self, _ctx: &Function_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#function_call}.
 * @param ctx the parse tree
 */
fn exit_function_call(&mut self, _ctx: &Function_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#parameter_list}.
 * @param ctx the parse tree
 */
fn enter_parameter_list(&mut self, _ctx: &Parameter_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#parameter_list}.
 * @param ctx the parse tree
 */
fn exit_parameter_list(&mut self, _ctx: &Parameter_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#control_flow}.
 * @param ctx the parse tree
 */
fn enter_control_flow(&mut self, _ctx: &Control_flowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#control_flow}.
 * @param ctx the parse tree
 */
fn exit_control_flow(&mut self, _ctx: &Control_flowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#if_statement}.
 * @param ctx the parse tree
 */
fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#if_statement}.
 * @param ctx the parse tree
 */
fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#function_identifier}.
 * @param ctx the parse tree
 */
fn enter_function_identifier(&mut self, _ctx: &Function_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#function_identifier}.
 * @param ctx the parse tree
 */
fn exit_function_identifier(&mut self, _ctx: &Function_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#function_definition}.
 * @param ctx the parse tree
 */
fn enter_function_definition(&mut self, _ctx: &Function_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#function_definition}.
 * @param ctx the parse tree
 */
fn exit_function_definition(&mut self, _ctx: &Function_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#function_body}.
 * @param ctx the parse tree
 */
fn enter_function_body(&mut self, _ctx: &Function_bodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#function_body}.
 * @param ctx the parse tree
 */
fn exit_function_body(&mut self, _ctx: &Function_bodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#class_method_identifier}.
 * @param ctx the parse tree
 */
fn enter_class_method_identifier(&mut self, _ctx: &Class_method_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#class_method_identifier}.
 * @param ctx the parse tree
 */
fn exit_class_method_identifier(&mut self, _ctx: &Class_method_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#vtable_assignment}.
 * @param ctx the parse tree
 */
fn enter_vtable_assignment(&mut self, _ctx: &Vtable_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#vtable_assignment}.
 * @param ctx the parse tree
 */
fn exit_vtable_assignment(&mut self, _ctx: &Vtable_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link tac#class_method_list}.
 * @param ctx the parse tree
 */
fn enter_class_method_list(&mut self, _ctx: &Class_method_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link tac#class_method_list}.
 * @param ctx the parse tree
 */
fn exit_class_method_list(&mut self, _ctx: &Class_method_listContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : tacListener<'input> }


