use crate::{common::number_literal_parser::{number_literal_to_u32, is_number_literal_u16}, parser::node::Node};

use std::collections::HashMap;

pub struct Evaluator {

}

impl Evaluator {

    pub fn new() -> Evaluator {
        Evaluator {
        }
    }

    pub fn evaluate(&mut self, symbol_table : &HashMap<String, u32>, variable_table : &HashMap<String, u32>,
            expression: &Option<Box<Node<String>>>) -> u32 {
        
        log::trace!("evaluate {:?}", expression);

        // if the option is empty, return 0
        if expression.is_none() 
        {
            return u32::default();
        }

        // retrieve value from option
        let expr = expression.clone().unwrap();

        match expr.value.as_str() {

            "*" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return lhs_evaluated_value * rhs_evaluated_value;
            }

            "/" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return lhs_evaluated_value / rhs_evaluated_value;
            }

            "+" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return lhs_evaluated_value + rhs_evaluated_value;
            }

            "-" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return lhs_evaluated_value - rhs_evaluated_value;
            }

            "<<" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return lhs_evaluated_value << rhs_evaluated_value;
            }

            ">>" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return lhs_evaluated_value >> rhs_evaluated_value;
            }

            ">" => {
                panic!("Not implemented yet!")
            }

            ">=" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return if lhs_evaluated_value >= rhs_evaluated_value { 1 } else { 0 };
            }

            "<" => {
                panic!("Not implemented yet!")
            }

            "<=" => {
                let lhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.left);
                let rhs_evaluated_value: u32 = self.evaluate(symbol_table, variable_table, &expr.right);
                return if lhs_evaluated_value <= rhs_evaluated_value { 1 } else { 0 };
            }

            _ => { /*panic!("Unknown!") */ },

        }

        // if the expression contains a direct value and is not an operator, return that value
        if is_number_literal_u16(&expr.value)
        {
            return number_literal_to_u32(&expr.value);
        }

        let symbol_table_entry_option = symbol_table.get(&expr.value);
        if !symbol_table_entry_option.is_none()
        {
            let symbol_table_value: &u32 = symbol_table_entry_option.unwrap();
            return *symbol_table_value;
        }

        let variable_table_entry_option = variable_table.get(&expr.value);
        if !variable_table_entry_option.is_none()
        {
            let variable_table_value: &u32 = variable_table_entry_option.unwrap();
            return *variable_table_value;
        }

        panic!("symbol not found in any table! '{}'", expr.value);

        // u32::default()
    }

}