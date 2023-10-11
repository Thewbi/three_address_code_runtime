#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
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
            TacLineType::BREAK => String::from("BREAK"),
            TacLineType::PRINT => String::from("PRINT"),
            _ => String::from("UNKNOWN"),
        }

    }

}
