#![allow(nonstandard_style)]
// Generated from ../../src/parser/tac.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
//use super::tac::*;
use super::tacparser::*;
use crate::parser::tacparser::tacContextType;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link tac}.
 */
pub trait tacVisitor<'input>: ParseTreeVisitor<'input,tacContextType>{
	/**
	 * Visit a parse tree produced by {@link tac#compilation_unit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilation_unit(&mut self, ctx: &Compilation_unitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#source_line}.
	 * @param ctx the parse tree
	 */
	fn visit_source_line(&mut self, ctx: &Source_lineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#label}.
	 * @param ctx the parse tree
	 */
	fn visit_label(&mut self, ctx: &LabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#left_hand_side}.
	 * @param ctx the parse tree
	 */
	fn visit_left_hand_side(&mut self, ctx: &Left_hand_sideContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#lcall_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_lcall_expression(&mut self, ctx: &Lcall_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#unary_operator}.
	 * @param ctx the parse tree
	 */
	fn visit_unary_operator(&mut self, ctx: &Unary_operatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#binary_operator}.
	 * @param ctx the parse tree
	 */
	fn visit_binary_operator(&mut self, ctx: &Binary_operatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#or_operator}.
	 * @param ctx the parse tree
	 */
	fn visit_or_operator(&mut self, ctx: &Or_operatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#and_operator}.
	 * @param ctx the parse tree
	 */
	fn visit_and_operator(&mut self, ctx: &And_operatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#equals_operator}.
	 * @param ctx the parse tree
	 */
	fn visit_equals_operator(&mut self, ctx: &Equals_operatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#operand}.
	 * @param ctx the parse tree
	 */
	fn visit_operand(&mut self, ctx: &OperandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#function_call}.
	 * @param ctx the parse tree
	 */
	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#parameter_list}.
	 * @param ctx the parse tree
	 */
	fn visit_parameter_list(&mut self, ctx: &Parameter_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#parameter}.
	 * @param ctx the parse tree
	 */
	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#control_flow}.
	 * @param ctx the parse tree
	 */
	fn visit_control_flow(&mut self, ctx: &Control_flowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#if_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#function_identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_function_identifier(&mut self, ctx: &Function_identifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#function_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_function_definition(&mut self, ctx: &Function_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#function_body}.
	 * @param ctx the parse tree
	 */
	fn visit_function_body(&mut self, ctx: &Function_bodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#class_method_identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_class_method_identifier(&mut self, ctx: &Class_method_identifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#vtable_assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_vtable_assignment(&mut self, ctx: &Vtable_assignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tac#class_method_list}.
	 * @param ctx the parse tree
	 */
	fn visit_class_method_list(&mut self, ctx: &Class_method_listContext<'input>) { self.visit_children(ctx) }

}

pub trait tacVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= tacContextType>{
	/**
	 * Visit a parse tree produced by {@link tac#compilation_unit}.
	 * @param ctx the parse tree
	 */
		fn visit_compilation_unit(&mut self, ctx: &Compilation_unitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#source_line}.
	 * @param ctx the parse tree
	 */
		fn visit_source_line(&mut self, ctx: &Source_lineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#label}.
	 * @param ctx the parse tree
	 */
		fn visit_label(&mut self, ctx: &LabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#left_hand_side}.
	 * @param ctx the parse tree
	 */
		fn visit_left_hand_side(&mut self, ctx: &Left_hand_sideContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#lcall_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_lcall_expression(&mut self, ctx: &Lcall_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#predicate}.
	 * @param ctx the parse tree
	 */
		fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#unary_operator}.
	 * @param ctx the parse tree
	 */
		fn visit_unary_operator(&mut self, ctx: &Unary_operatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#binary_operator}.
	 * @param ctx the parse tree
	 */
		fn visit_binary_operator(&mut self, ctx: &Binary_operatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#or_operator}.
	 * @param ctx the parse tree
	 */
		fn visit_or_operator(&mut self, ctx: &Or_operatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#and_operator}.
	 * @param ctx the parse tree
	 */
		fn visit_and_operator(&mut self, ctx: &And_operatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#equals_operator}.
	 * @param ctx the parse tree
	 */
		fn visit_equals_operator(&mut self, ctx: &Equals_operatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#operand}.
	 * @param ctx the parse tree
	 */
		fn visit_operand(&mut self, ctx: &OperandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#function_call}.
	 * @param ctx the parse tree
	 */
		fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#parameter_list}.
	 * @param ctx the parse tree
	 */
		fn visit_parameter_list(&mut self, ctx: &Parameter_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#parameter}.
	 * @param ctx the parse tree
	 */
		fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#control_flow}.
	 * @param ctx the parse tree
	 */
		fn visit_control_flow(&mut self, ctx: &Control_flowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#if_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#function_identifier}.
	 * @param ctx the parse tree
	 */
		fn visit_function_identifier(&mut self, ctx: &Function_identifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#function_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_function_definition(&mut self, ctx: &Function_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#function_body}.
	 * @param ctx the parse tree
	 */
		fn visit_function_body(&mut self, ctx: &Function_bodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#class_method_identifier}.
	 * @param ctx the parse tree
	 */
		fn visit_class_method_identifier(&mut self, ctx: &Class_method_identifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#vtable_assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_vtable_assignment(&mut self, ctx: &Vtable_assignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tac#class_method_list}.
	 * @param ctx the parse tree
	 */
		fn visit_class_method_list(&mut self, ctx: &Class_method_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> tacVisitor<'input> for T
where
	T: tacVisitorCompat<'input>
{
	fn visit_compilation_unit(&mut self, ctx: &Compilation_unitContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_compilation_unit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_source_line(&mut self, ctx: &Source_lineContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_source_line(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_label(&mut self, ctx: &LabelContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_label(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_left_hand_side(&mut self, ctx: &Left_hand_sideContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_left_hand_side(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lcall_expression(&mut self, ctx: &Lcall_expressionContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_lcall_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unary_operator(&mut self, ctx: &Unary_operatorContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_unary_operator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_binary_operator(&mut self, ctx: &Binary_operatorContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_binary_operator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_or_operator(&mut self, ctx: &Or_operatorContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_or_operator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_and_operator(&mut self, ctx: &And_operatorContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_and_operator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equals_operator(&mut self, ctx: &Equals_operatorContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_equals_operator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_operand(&mut self, ctx: &OperandContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_operand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_function_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameter_list(&mut self, ctx: &Parameter_listContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_parameter_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_parameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_control_flow(&mut self, ctx: &Control_flowContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_control_flow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_if_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_identifier(&mut self, ctx: &Function_identifierContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_function_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_definition(&mut self, ctx: &Function_definitionContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_function_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_body(&mut self, ctx: &Function_bodyContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_function_body(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_class_method_identifier(&mut self, ctx: &Class_method_identifierContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_class_method_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_vtable_assignment(&mut self, ctx: &Vtable_assignmentContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_vtable_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_class_method_list(&mut self, ctx: &Class_method_listContext<'input>){
		let result = <Self as tacVisitorCompat>::visit_class_method_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}