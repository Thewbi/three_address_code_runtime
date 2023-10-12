#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
#[allow(non_camel_case_types)]
pub enum TacLineType {

    ASSIGNMENT,

    IF_STATEMENT,

    CALL,

    GOTO,

    BREAK,

    PRINT,

    PUSH,

    POP,

    RETURN,

    FUNC_DEFINITION,

    FUNC_DEFINITION_END,

    CLASS_METHOD,

    #[default]
    UNKNOWN,

}

impl TacLineType {

    #[allow(dead_code)]
    pub fn to_string(&self) -> String { 

        match self {
            TacLineType::ASSIGNMENT => String::from("ASSIGNMENT"),
            TacLineType::IF_STATEMENT => String::from("IF_STATEMENT"),
            TacLineType::CALL => String::from("CALL"),
            TacLineType::GOTO => String::from("GOTO"),
            TacLineType::BREAK => String::from("BREAK"),
            TacLineType::PRINT => String::from("PRINT"),
            TacLineType::PUSH => String::from("PUSH"),
            TacLineType::POP => String::from("POP"),
            TacLineType::RETURN => String::from("RETURN"),
            TacLineType::FUNC_DEFINITION => String::from("FUNC_DEFINITION"),
            TacLineType::FUNC_DEFINITION_END => String::from("FUNC_DEFINITION_END"),
            TacLineType::CLASS_METHOD => String::from("CLASS_METHOD"),
            _ => String::from("UNKNOWN"),
        }

    }

}
