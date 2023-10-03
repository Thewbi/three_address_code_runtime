use antlr_rust::tree::{ParseTreeVisitorCompat, ParseTree};

use crate::parser::node::Node;

use super::{tacparser::{tacContextType, Source_lineContext, AssignmentContext, ExpressionContext}, tacvisitor::tacVisitorCompat};

pub struct TacVisitorNodes {

    // debug
    pub debug_output: bool,
    pub indent: u16,
    
    // traversal
    pub return_val: Vec<Node<String>>,

}

impl Default for TacVisitorNodes {
    fn default() -> Self {
        Self {

            indent: 0u16,
            debug_output: true,

            return_val: Vec::new(),

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
        self.indent = self.indent + 1;
        for _n in 0..self.indent {
            print!("  ");
        }
        println!("{}", label);
    }

    pub fn reset_self(&mut self) {
        //self.record.clear();
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

    fn visit_source_line(&mut self, ctx: &Source_lineContext<'i>) -> Self::Return {
        self.descend_indent("visit_source_line");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

    fn visit_assignment(&mut self, ctx: &AssignmentContext<'i>) -> Self::Return {
        self.descend_indent("visit_assignment");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'i>) -> Self::Return {
        self.descend_indent("visit_expression");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

}