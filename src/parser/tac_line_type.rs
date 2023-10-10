use std::fmt;

#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum TacLineType {

    ASSIGNMENT,

    IF_STATEMENT,

    CALL,

    GOTO,

    #[default]
    UNKNOWN,

}

impl TacLineType {

    pub fn to_string(&self) -> String { 

        match self {
            TacLineType::ASSIGNMENT => String::from("ASSIGNMENT"),
            TacLineType::IF_STATEMENT => String::from("IF_STATEMENT"),
            TacLineType::CALL => String::from("CALL"),
            TacLineType::GOTO => String::from("GOTO"),
            _ => String::from("UNKNOWN"),
        }

    }

}
