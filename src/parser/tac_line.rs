use super::{node::Node, tac_line_type::TacLineType};

use std::fmt;

/// This struct is the model for a line in a assembler source code file
#[derive(Debug, Clone)]
pub struct TacLine {

    pub idx: u32,

    pub lhs: String,
    pub lhs_deref: bool,

    pub line_type: TacLineType,

    pub label: String,

    pub expression_1: Option<Box<Node<String>>>,
    pub expression_2: Option<Box<Node<String>>>,

    pub parameter_list: Vec<String>,

    pub target_label: String,

    pub class: String,

    pub source_file: String,
    pub line: isize,
    pub column: isize,

}

impl TacLine {

    #[allow(dead_code)]
    pub fn new(
        idx: u32,
        lhs: String,
        lhs_deref: bool,
        line_type: TacLineType,
        label: String,
        target_label: String,
        class: String,
        source_file: String,
        line: isize,
        column: isize,
    ) -> TacLine {
        TacLine {
            idx: idx,
            lhs: lhs,
            lhs_deref: lhs_deref,
            line_type: line_type,
            label: label,
            parameter_list: Vec::default(),
            expression_1: None,
            expression_2: None,
            target_label: target_label,
            class: class,
            source_file: source_file,
            line: line,
            column: column,
        }
    }

    pub fn clear(&mut self) {
        self.idx = u32::default();
        self.lhs = String::default();
        self.lhs_deref = false;
        self.line_type = TacLineType::UNKNOWN;
        self.label = String::default();
        self.expression_1 = None;
        self.expression_2 = None;
        self.parameter_list = Vec::default();
        self.target_label = String::default();
        self.class = String::default();
        self.source_file = String::default();
        self.line = isize::default();
        self.column = isize::default();
    }

}

impl Default for TacLine {
    fn default() -> Self {
        Self {
            idx: u32::default(),
            lhs: String::default(),
            lhs_deref: false,
            line_type: TacLineType::UNKNOWN,
            label: String::default(),
            expression_1: None,
            expression_2: None,
            parameter_list: Vec::default(),
            target_label: String::default(),
            class: String::default(),
            source_file: String::default(),
            line: isize::default(),
            column: isize::default(),
        }
    }
}

impl fmt::Display for TacLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(idx: {:?}, lhs: {:?}, lhs_deref: {:?}, type: {:?}, label: {:?}, expr_1: {:?}, expr_2: {:?}, parameter_list: {:?}, target_label: {:?}, class: {:?}, source_file:{}, line:{}, column:{})", 
            self.idx,
            self.lhs,
            self.lhs_deref,
            self.line_type,
            self.label, 
            self.expression_1, self.expression_2,
            self.parameter_list,
            self.target_label,
            self.class,
            self.source_file, self.line, self.column)
    }
}
