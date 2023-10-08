use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::GenericToken;
use antlr_rust::tree::{ParseTreeVisitorCompat, ParseTree};

use crate::parser::tac_line_type::TacLineType;
use crate::{parser::node::Node, common::number_literal_parser::number_literal_to_u16};
use crate::common::number_literal_parser::is_number_literal_u16;

use super::{tacparser::{tacContextType, Source_lineContext, AssignmentContext, ExpressionContext, Compilation_unitContext}, tacvisitor::tacVisitorCompat, tac_line::TacLine};
use std::cell::Ref;
use std::borrow::Cow;

pub struct TacVisitorNodes {

    // result
    pub lines: Vec<TacLine>,
    pub line: TacLine,

    // debug
    pub debug_output: bool,
    pub indent: u16,
    
    // traversal
    pub return_val: Vec<Node<String>>,

    // the source file that this visitor visits
    pub source_file: String,

}

impl Default for TacVisitorNodes {
    fn default() -> Self {
        Self {

            lines: Vec::new(),
            line: TacLine::default(),

            indent: 0u16,
            debug_output: true,

            return_val: Vec::new(),

            source_file: String::default(),

        }
    }
}

impl<'i> TacVisitorNodes {

    pub fn ascend_indent(&mut self) {
        if !self.debug_output {
            return;
        }

        self.indent = self.indent - 1;
    }

    pub fn descend_indent(&mut self, label: &str) {
        if !self.debug_output {
            return;
        }
        
        for _n in 0..self.indent {
            print!("  ");
        }
        println!("{}", label);
        
        self.indent = self.indent + 1;
    }

    pub fn reset_self(&mut self) {
        //self.record.clear();
        self.indent = 0;
        self.return_val.clear();
    }

}

impl<'i> ParseTreeVisitorCompat<'i> for TacVisitorNodes {
    type Node = tacContextType;
    type Return = Vec<Node<String>>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.return_val
    }

    fn visit_terminal(
        &mut self,
        node: &antlr_rust::tree::TerminalNode<'i, Self::Node>,
    ) -> Self::Return {
        let terminal_text = node.get_text();
        log::trace!("'{}'", terminal_text);

        if terminal_text.eq(",") {
            return vec![];
        }

        let result_node = Node::<String>::new(terminal_text);

        return vec![result_node];
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'i, Self::Node>,
    ) -> Self::Return {
        Self::Return::default()
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        // https://stackoverflow.com/questions/40792801/what-is-the-best-way-to-concatenate-vectors-in-rust
        // let c: Vec<String> = aggregate
        //     .iter()
        //     .cloned()
        //     .chain(next.iter().cloned())
        //     .collect(); // Cloned


        // let c: Vec<Node<String>> = aggregate
        //     .iter()
        //     .cloned()
        //     .chain(next.iter().cloned())
        //     .collect(); // Cloned

        // c

        let mut c: Vec<Node<String>> = Vec::new();
        c.append(&mut aggregate.clone());
        c.append(&mut next.clone());

        c
    }

    #[allow(dead_code, unused)]
    fn should_visit_next_child(
        &self,
        node: &<Self::Node as antlr_rust::parser::ParserNodeType<'i>>::Type,
        current: &Self::Return,
    ) -> bool {
        true
    }
}

impl<'i> tacVisitorCompat<'i> for TacVisitorNodes {

    fn visit_compilation_unit(&mut self, ctx: &Compilation_unitContext<'i>) -> Self::Return {
        self.descend_indent("visit_compilation_unit");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

    fn visit_source_line(&mut self, ctx: &Source_lineContext<'i>) -> Self::Return {
        self.descend_indent("visit_source_line");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        self.lines.push(self.line.clone());
        self.line.clear();

        visit_children_result
    }

    fn visit_assignment(&mut self, ctx: &AssignmentContext<'i>) -> Self::Return {
        self.descend_indent("visit_assignment");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        let token: Ref<'_, GenericToken<Cow<'_, str>>> = ctx.start();

        self.line = TacLine::default();
        self.line.line_type = TacLineType::ASSIGNMENT;
        self.line.source_file = self.source_file.clone();
        self.line.line = token.line;
        self.line.column = token.column;
        self.line.lhs = visit_children_result[0].value.clone();

        if visit_children_result.len() > 2 {

            // expression (= operator + operands)
            if visit_children_result[2].expression 
            {
                self.line.expression_1 = Some(Box::new(visit_children_result[2].clone()));
            }
            // else 
            // {
            //     let param_1: &String = &visit_children_result[2].value;

            //     if is_number_literal_u16(param_1) 
            //     {
            //         param_1_as_number = number_literal_to_u16(&param_1);
            //         tac_line.dest = param_1_as_number;
            //     }
            // }

        }

        if visit_children_result.len() > 3 {

            if visit_children_result[3].expression 
            {
                self.line.expression_2 = Some(Box::new(visit_children_result[3].clone()));
            }

        }

        visit_children_result
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'i>) -> Self::Return {
        self.descend_indent("visit_expression");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        if visit_children_result.len() == 1usize {

            // // if the value is a number, return it
            // let parse_result = visit_children_result[0].value.parse::<u16>();
            // if parse_result.is_ok() {
            //     let mut literal_expression: Node<String> = Node::new(visit_children_result[0].value.clone());
            //     literal_expression.expression = true;

            //     //return vec![binary_tree.left(visit_children_result[2].clone())];
            //     return vec![literal_expression];
            // }

            let mut literal_expression: Node<String> = Node::new(visit_children_result[0].value.clone());
            literal_expression.expression = true;
            return vec![literal_expression];
        }

        visit_children_result
    }

}