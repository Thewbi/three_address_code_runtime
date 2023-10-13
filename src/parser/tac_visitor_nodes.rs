use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::GenericToken;
use antlr_rust::tree::{ParseTreeVisitorCompat, ParseTree, LeafNode};

use crate::parser::tac_line_type::TacLineType;
use crate::parser::tacparser::{Binary_operatorContext, Function_definitionContextAttrs};
use crate::parser::node::Node;

use super::tacparser::{Or_operatorContext, And_operatorContext, Equals_operatorContext, Left_hand_sideContext, Function_callContext, OperandContext, LabelContext, If_statementContext, Control_flowContext, PredicateContext, Parameter_listContext, ParameterContext, Function_definitionContext, Class_method_identifierContext};
use super::{tacparser::{tacContextType, Source_lineContext, AssignmentContext, ExpressionContext, Compilation_unitContext}, tacvisitor::tacVisitorCompat, tac_line::TacLine};
use std::cell::Ref;
use std::borrow::Cow;

use std::rc::Rc;

use antlr_rust::tree::NoError;

pub struct TacVisitorNodes {

    // result
    pub idx: u32,
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

            idx: u32::default(),
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

    #[allow(dead_code)]
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

    fn visit_compilation_unit(&mut self, ctx: &Compilation_unitContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_compilation_unit");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

    fn visit_source_line(&mut self, ctx: &Source_lineContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_source_line");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        self.idx = self.idx + 1u32;
        self.line.idx = self.idx;
        self.lines.push(self.line.clone());
        self.line.clear();
        

        visit_children_result
    }

    fn visit_label(&mut self, ctx: &LabelContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_label");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        self.line.label = visit_children_result[0].value.clone();

        visit_children_result
    }

    fn visit_assignment(&mut self, ctx: &AssignmentContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_assignment");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        let token: Ref<'_, GenericToken<Cow<'_, str>>> = ctx.start();

        // self.line = TacLine::default();
        self.line.line_type = TacLineType::ASSIGNMENT;
        self.line.source_file = self.source_file.clone();
        self.line.line = token.line;
        self.line.column = token.column;
        self.line.lhs = visit_children_result[0].value.clone();
        self.line.lhs_deref = visit_children_result[0].deref;

        if visit_children_result.len() > 2 {

            if visit_children_result[0].expression 
            {
                self.line.expression_1 = Some(Box::new(visit_children_result[0].clone()));
            }

            if visit_children_result[2].expression 
            {
                //log::info!("LUL: {:?}\n", visit_children_result[2]);

                self.line.expression_2 = Some(Box::new(visit_children_result[2].clone()));
            }

            // if visit_children_result[0].expression && visit_children_result[2].expression
            // {
            //     log::info!("1\n");

            //     self.line.expression_1 = Some(Box::new(visit_children_result[0].clone()));
            //     self.line.expression_2 = Some(Box::new(visit_children_result[2].clone()));
            // }
            // else if visit_children_result[2].expression 
            // {
            //     self.line.expression_1 = Some(Box::new(visit_children_result[2].clone()));
            // }
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

        // if visit_children_result.len() > 3 {

        //     if visit_children_result[3].expression 
        //     {
        //         self.line.expression_2 = Some(Box::new(visit_children_result[3].clone()));
        //     }

        // }

        visit_children_result
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_expression");
        let mut visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        if visit_children_result.len() == 1usize 
        {
            // // if the value is a number, return it
            // let parse_result = visit_children_result[0].value.parse::<u16>();
            // if parse_result.is_ok() {
            //     let mut literal_expression: Node<String> = Node::new(visit_children_result[0].value.clone());
            //     literal_expression.expression = true;

            //     //return vec![binary_tree.left(visit_children_result[2].clone())];
            //     return vec![literal_expression];
            // }

            if visit_children_result[0].expression 
            {
                return vec![visit_children_result[0].clone()];
            } 
            else 
            {
                let mut literal_expression: Node<String> = Node::new(visit_children_result[0].value.clone());

                literal_expression.expression = true;

                // set the expression flag to false because the evaluator implementation expects a value
                // of false for nodes that carry literals (nodes that contain no further children)
                //literal_expression.expression = false;
                
                return vec![literal_expression];
            }

        } 
        else if visit_children_result.len() == 2usize 
        {
            //log::info!("test\n");

            let op_as_string: &String = &visit_children_result[0].value;
            let lhs_as_string: &String = &visit_children_result[1].value;

            // if self.debug_output {
            //     println!("lhs: {} op: {}", lhs_as_string, op_as_string);
            // }

            let mut op_node: Node<String> = Node::new(op_as_string.clone());
            op_node.expression = true;

            return vec![op_node.left(visit_children_result[1].clone())];
        }
        else if visit_children_result.len() == 3usize 
        {
            if "(".eq(&visit_children_result[0].value) && ")".eq(&visit_children_result[2].value)
            {
                visit_children_result.remove(2);
                visit_children_result.remove(0);

                return visit_children_result;
            }
            // else if ".".eq(&visit_children_result[0].value)
            // {
            //     let sign: &str = visit_children_result[1].value.as_str();
            //     let mut offset: i16 = number_literal_to_u16(&visit_children_result[2].value) as i16;
            //     if sign == "-"
            //     {
            //         offset *= -1i16;
            //     }

            //     log::trace!("sign: {}, offset: {}", sign, offset);

            //     let binary_tree: Node<String> = Node::new(offset.to_string());
            //     return vec![binary_tree];
            // } 
            else 
            {
                // lhs << rhs
                let lhs_as_string: &String = &visit_children_result[0].value;
                let op_as_string: &String = &visit_children_result[1].value;
                let rhs_as_string: &String = &visit_children_result[2].value;

                // if self.debug_output {
                //     println!("lhs: {} op: {} rhs: {}", lhs_as_string, op_as_string, rhs_as_string);
                // }
                let mut op_node: Node<String> = Node::new(op_as_string.clone());
                op_node.expression = true;

                return vec![op_node.left(visit_children_result[0].clone()).right(visit_children_result[2].clone())];
            }

        }

        visit_children_result
    }

    fn visit_binary_operator(&mut self, ctx: &Binary_operatorContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_binary_operator");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        //let lhs_as_string: &String = &visit_children_result[0].value;
        //log::info!("lhs: {}\n", lhs_as_string);

        return visit_children_result;
    }

    fn visit_or_operator(&mut self, ctx: &Or_operatorContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_or_operator");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        let first_as_string: &String = &visit_children_result[0].value;
        let second_as_string: &String = &visit_children_result[1].value;

        log::info!("first_as_string: {}\n", first_as_string);
        log::info!("second_as_string: {}\n", second_as_string);

        let mut op_node: Node<String> = Node::new(String::from("||"));
        op_node.expression = true;

        return vec![op_node];
    }

    fn visit_and_operator(&mut self, ctx: &And_operatorContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_and_operator");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        let first_as_string: &String = &visit_children_result[0].value;
        let second_as_string: &String = &visit_children_result[1].value;

        log::info!("first_as_string: {}\n", first_as_string);
        log::info!("second_as_string: {}\n", second_as_string);

        let mut op_node: Node<String> = Node::new(String::from("&&"));
        op_node.expression = true;

        return vec![op_node];
    }

    fn visit_equals_operator(&mut self, ctx: &Equals_operatorContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_equals_operator");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        let first_as_string: &String = &visit_children_result[0].value;
        let second_as_string: &String = &visit_children_result[1].value;

        //log::info!("first_as_string: {}\n", first_as_string);
        //log::info!("second_as_string: {}\n", second_as_string);

        let mut op_node: Node<String> = Node::new(String::from("=="));
        op_node.expression = true;

        return vec![op_node];
    }

    fn visit_left_hand_side(&mut self, ctx: &Left_hand_sideContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_left_hand_side");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        if visit_children_result.len() == 1usize
        {
            return visit_children_result;
        }
        else if visit_children_result.len() == 2usize
        {
            let first_as_string: &String = &visit_children_result[0].value;
            let second_as_string: &String = &visit_children_result[1].value;

            log::info!("first_as_string: {}\n", first_as_string);
            log::info!("second_as_string: {}\n", second_as_string);

            let mut op_node: Node<String> = Node::new(second_as_string.clone());
            op_node.deref = true;

            return vec![op_node];
        }
        else if visit_children_result.len() == 4usize
        {
            log::info!("4\n");

            log::info!("1: {:?} 2: {:?} 3: {:?} 4: {:?}", visit_children_result[0], visit_children_result[1], visit_children_result[2], visit_children_result[3]);
        
            if "*".eq(&visit_children_result[0].value) && "(".eq(&visit_children_result[1].value) && ")".eq(&visit_children_result[3].value)
            {
                let mut op_node: Node<String> = visit_children_result[2].clone();
                op_node.deref = true;

                return vec![op_node];
            }
        }

        visit_children_result
    }

    fn visit_function_call(&mut self, ctx: &Function_callContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_function_call");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        let function_name: &String = &visit_children_result[0].value.to_lowercase();

        //log::info!("function_name: {}\n", function_name);

        if "sqrt".eq(function_name)
        {
            //log::info!("child_1: {:?}\n", visit_children_result[1]);

            let mut op_node: Node<String> = visit_children_result[1].clone();
            op_node.expression = true;
            op_node.value = String::from("sqrt");
            op_node.left = Some(Box::new(visit_children_result[1].clone()));

            //log::info!("op_node: {:?}\n", op_node);

            return vec![op_node];
        }
        else if "sizeof".eq(function_name)
        {
            //log::info!("child_1: {:?}\n", visit_children_result[1]);

            let mut op_node: Node<String> = visit_children_result[1].clone();
            op_node.expression = true;
            op_node.value = String::from("sizeof");
            op_node.left = Some(Box::new(visit_children_result[1].clone()));

            //log::info!("op_node: {:?}\n", op_node);

            return vec![op_node];
        }
        else if "break".eq(function_name)
        {
            self.line.line_type = TacLineType::BREAK;
        }
        else if "print".eq(function_name)
        {
            self.line.line_type = TacLineType::PRINT;

            //log::info!("child_1: {:?}\n", visit_children_result[1]);

            // let mut op_node: Node<String> = visit_children_result[1].clone();
            // op_node.expression = true;
            // op_node.value = String::from("print");
            // op_node.left = Some(Box::new(visit_children_result[1].clone()));

            // log::info!("op_node: {:?}\n", op_node);

            self.line.expression_1 = Some(Box::new(visit_children_result[1].clone()));

            return visit_children_result;
        }
        else if "push".eq(function_name)
        {
            self.line.line_type = TacLineType::PUSH;

            self.line.expression_1 = Some(Box::new(visit_children_result[1].clone()));
        }
        else if "pop".eq(function_name)
        {
            self.line.line_type = TacLineType::POP;

            self.line.expression_1 = Some(Box::new(visit_children_result[1].clone()));
        }

        visit_children_result
    }

    fn visit_parameter_list(&mut self, ctx: &Parameter_listContext<'i>) -> Self::Return {
        self.descend_indent("visit_parameter_list");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        log::info!("visit_parameter_list: {:?}\n", visit_children_result[0]);

        visit_children_result
    }

    fn visit_parameter(&mut self, ctx: &ParameterContext<'i>) -> Self::Return {
        self.descend_indent("visit_parameter");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        log::info!("visit_parameter: {:?}\n", visit_children_result[0]);

        self.line.parameter_list.push(visit_children_result[0].value.clone());

        visit_children_result
    }

    fn visit_operand(&mut self, ctx: &OperandContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_operand");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        //log::info!("visit_operand: {:?}\n", visit_children_result[0]);

        visit_children_result
    }

    fn visit_control_flow(&mut self, ctx: &Control_flowContext<'i>) -> Self::Return
    { 
        self.descend_indent("visit_if_statement");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        if self.line.line_type == TacLineType::IF_STATEMENT
        {
            self.line.expression_1 = Some(Box::new(visit_children_result[0].clone()));
            self.line.target_label = visit_children_result[2].value.clone();
        }
        else if visit_children_result[0].value.to_lowercase().eq("goto")
        {
            self.line.line_type = TacLineType::GOTO;
            self.line.target_label = visit_children_result[1].value.clone();
        }
        else if visit_children_result[0].value.to_lowercase().eq("call")
        {
            self.line.line_type = TacLineType::CALL;
            self.line.target_label = visit_children_result[1].value.clone();
        }
        else if visit_children_result[0].value.to_lowercase().eq("return")
        {
            self.line.line_type = TacLineType::RETURN;

            if visit_children_result.len() > 1
            {
                self.line.expression_1 = Some(Box::new(visit_children_result[1].clone()));
            }
        }

        visit_children_result
    }

    fn visit_if_statement(&mut self, ctx: &If_statementContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_if_statement");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        self.line.line_type = TacLineType::IF_STATEMENT;

        let predicate_node: Node<String> = visit_children_result[2].clone();//Some(Box::new(visit_children_result[0].clone()));

        return vec![predicate_node];
    }

    fn visit_predicate(&mut self, ctx: &PredicateContext<'i>) -> Self::Return
    {
        self.descend_indent("visit_predicate");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        if visit_children_result.len() == 1 && visit_children_result[0].expression
        {
            return vec![visit_children_result[0].clone()];
        }
        else 
        {
            let lhs_as_string: &String = &visit_children_result[0].value;
            let op_as_string: &String = &visit_children_result[1].value;
            let rhs_as_string: &String = &visit_children_result[2].value;

            // if self.debug_output {
            //     println!("lhs: {} op: {} rhs: {}", lhs_as_string, op_as_string, rhs_as_string);
            // }
            let mut op_node: Node<String> = Node::new(op_as_string.clone());
            op_node.expression = true;

            return vec![op_node.left(visit_children_result[0].clone()).right(visit_children_result[2].clone())];
        }

        //visit_children_result
    }

    fn visit_function_definition(&mut self, ctx: &Function_definitionContext<'i>) -> Self::Return {

        let token: Ref<'_, GenericToken<Cow<'_, str>>> = ctx.start();

        let identifier_rc: Rc<LeafNode<'_, tacContextType, NoError>> = ctx.IDENTIFIER().unwrap();
        let sym = identifier_rc.symbol;
        let identifier = sym.text.clone();

        let mut local_line = TacLine::default();
        local_line.line_type = TacLineType::FUNC_DEFINITION;
        local_line.source_file = self.source_file.clone();
        local_line.label = identifier.to_string();
        local_line.line = token.line;
        local_line.column = token.column;
        
        self.idx = self.idx + 1u32;
        local_line.idx = self.idx;
        self.lines.push(local_line);

        




        self.descend_indent("visit_function_definition");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        //log::info!("visit_function_definition: {:?}\n", visit_children_result[0].value);

        






        self.line.clear();

        self.line.line_type = TacLineType::FUNC_DEFINITION_END;
        self.line.source_file = self.source_file.clone();
        self.line.line = token.line;
        self.line.column = token.column;

        self.idx = self.idx + 1u32;
        self.line.idx = self.idx;
        self.lines.push(self.line.clone());
        self.line.clear();

        visit_children_result
    }

    fn visit_class_method_identifier(&mut self, ctx: &Class_method_identifierContext<'i>) -> Self::Return 
    {
        self.descend_indent("visit_class_method_identifier");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        log::info!("visit_class_method_identifier 0: {:?}\n", visit_children_result[0]);
        log::info!("visit_class_method_identifier 1: {:?}\n", visit_children_result[1]);
        log::info!("visit_class_method_identifier 2: {:?}\n", visit_children_result[2]);

        let token: Ref<'_, GenericToken<Cow<'_, str>>> = ctx.start();

        self.line.source_file = self.source_file.clone();
        self.line.line = token.line;
        self.line.column = token.column;

        self.line.line_type = TacLineType::CLASS_METHOD;
        self.line.class = visit_children_result[0].value.clone();
        self.line.label = visit_children_result[2].value.clone();

        visit_children_result
    }

}