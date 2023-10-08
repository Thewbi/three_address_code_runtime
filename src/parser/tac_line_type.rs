use std::fmt;

#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum TacLineType {

    ASSIGNMENT,

    #[default]
    UNKNOWN,

}

impl TacLineType {

    pub fn to_string(&self) -> String { 

        match self {
            TacLineType::ASSIGNMENT => String::from("ASSIGNMENT"),
            _ => String::from("UNKNOWN"),
        }

    }

}
