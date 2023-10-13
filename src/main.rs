mod parser;
mod common;
mod evaluator;

use std::fs;
use std::rc::Rc;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::InputStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use env_logger::{Builder, Target};
use log::LevelFilter;

use dyn_fmt::AsStrFormatExt;

use crate::evaluator::evaluator::Evaluator;
use crate::parser::tac_line::TacLine;
use crate::parser::tac_visitor_nodes::TacVisitorNodes;
use crate::parser::tacparser::Compilation_unitContextAll;
use crate::parser::tacparser::tacContextType;

//
// Make sure that your PATH environment variable only contains a single rust installation
// where rustc
// C:\Users\U5353\.cargo\bin\rustc.exe
//
// rustup update
// rustup update stable-x86_64-pc-windows-msvc
// cargo update -p libc
// cargo update
//

//
// First, build the grammar
// cargo run --bin build_parser
// the generated files are placed into src/parser
//
// antlr lab
//
// cargo clean
// cargo build
// cargo run --bin build_parser
// cargo run --bin three_address_code_runtime
//
// cargo run
//
// cargo fmt
fn main() {
    //println!("Hello, world!");

    init_logging();
    log_start();

    // the symbol table
    let mut symbol_table: HashMap<String, u32> = HashMap::new();

    load_segment_from_source_code();

    // let token_factory: antlr_rust::token_factory::ArenaFactory<'_, antlr_rust::token_factory::CommonTokenFactory, antlr_rust::token::GenericToken<_>> = ArenaCommonFactory::default();
    // let mut _lexer: parser::assembler_lexerlexer::assembler_lexer<'_, InputStream<&str>> = parser::assembler_lexerlexer::assembler_lexer::new_with_token_factory(
    //     input_stream,
    //     &token_factory,
    // );
    // let token_source: CommonTokenStream<'_, parser::assembler_lexerlexer::assembler_lexer<'_, InputStream<&str>>> = CommonTokenStream::new(_lexer);
    // let mut parser: parser::assembler_parserparser::assembler_parser<'_, CommonTokenStream<'_, parser::assembler_lexerlexer::assembler_lexer<'_, InputStream<&str>>>, antlr_rust::DefaultErrorStrategy<'_, assembler_parserContextType>> = parser::assembler_parserparser::assembler_parser::new(token_source);

    // let result = parser.asm_file();
    // assert!(result.is_ok());
    // let root: Rc<Asm_fileContextAll> = result.unwrap();

    // let mut visitor: NodeAssemblerVisitor = NodeAssemblerVisitor::default();
    // visitor.source_file = source_file.clone();
    // visitor.record.clear();

    // let _visitor_result = visitor.visit(&*root);

    log_end();
}

fn load_segment_from_source_code() /*-> [u8; RAMEND as usize] */
{
    //
    // Phase - read the .asm file
    //

    log::info!("**********************************************\n");
    log::info!("Phase - read the .tac file\n");
    log::info!("**********************************************\n");

    //
    // read the .asm file which will be the input to the assembler
    //
    // check files here: http://lab.antlr.org/
    // (erase the entire content in the lexer tab, paste the grammar into the parser tab,
    // use 'asm_file' as a start symbol)
    //

    let mut source_code_file_path: String = String::new();
    //source_code_file_path.push_str("test_resources/sample_files/tac/assignments.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/break.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/call_statement.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/function_definition.tac"); // test
    //source_code_file_path.push_str("test_resources/sample_files/tac/goto.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/if_statement.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/label.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/print.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/push_pop.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/return.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/oop.tac");

    source_code_file_path.push_str("test_resources/sample_files/tac/loop_application.tac");

    let srcdir = PathBuf::from(&source_code_file_path);
    log::info!("absolute path: {:?}\n", fs::canonicalize(&srcdir));

    let data = fs::read_to_string(&source_code_file_path).expect("Unable to read file");
    log::trace!("\n{}\n", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    parse_and_encode(input_stream, source_code_file_path.clone())

}

fn parse_and_encode(
    input_stream: InputStream<&str>, 
    source_file: String)
{
    //
    // Phase - AST Creation (Grammar Lexing and Parsing)
    //

    log::info!("*************************************************\n");
    log::info!("Phase - AST Creation (Grammar Lexing and Parsing)\n");
    log::info!("*************************************************\n");

    let token_factory: antlr_rust::token_factory::ArenaFactory<'_, antlr_rust::token_factory::CommonTokenFactory, antlr_rust::token::GenericToken<_>> = ArenaCommonFactory::default();
    let mut _lexer: parser::tac_lexerlexer::tac_lexer<'_, InputStream<&str>> = parser::tac_lexerlexer::tac_lexer::new_with_token_factory(
        input_stream,
        &token_factory,
    );
    let token_source: CommonTokenStream<'_, parser::tac_lexerlexer::tac_lexer<'_, InputStream<&str>>> = CommonTokenStream::new(_lexer);
    let mut parser: parser::tacparser::tac<'_, CommonTokenStream<'_, parser::tac_lexerlexer::tac_lexer<'_, InputStream<&str>>>, antlr_rust::DefaultErrorStrategy<'_, tacContextType>> = parser::tacparser::tac::new(token_source);

    let result = parser.compilation_unit();
    assert!(result.is_ok());
    let root: Rc<Compilation_unitContextAll> = result.unwrap();

    log::info!("*************************************************\n");
    log::info!("Phase - AST Visiting - First Phase - Create Lines and digest expression trees\n");
    log::info!("*************************************************\n");

    // node visitor
    let mut visitor: TacVisitorNodes = TacVisitorNodes::default();
    visitor.source_file = source_file.clone();

    let _visitor_result = visitor.visit(&*root);

    log::info!("*************************************************************\n");
    log::info!("Phase - DEBUG - Output all lines after digestion\n");
    log::info!("*************************************************************\n");

    log::info!("\n");
    for line in visitor.lines.iter_mut() {
        log::info!("{}\n", line);
        log::info!("\n");
    }

    log::info!("*************************************************************\n");
    log::info!("Phase - construct symbol table\n");
    log::info!("*************************************************************\n");

    // the symbol table
    let mut symbol_table: HashMap<String, u32> = HashMap::new();

    for line in visitor.lines.iter_mut() {
        if !line.label.is_empty() 
        {
            symbol_table.insert(line.label.clone(), line.idx - 1u32);
        }
    }

    // DEBUG - output symbol table
    for (label, idx) in &symbol_table {
        println!("Label: {} -> idx: {:#04X} {}d", label, idx, idx);
    }

    log::info!("*************************************************************\n");
    log::info!("Phase - execute\n");
    log::info!("*************************************************************\n");

    let mut variable_table: HashMap<String, u32> = HashMap::new();

    let mut evaluator: Evaluator = Evaluator::new();
    let mut pc: usize = 0usize;
    let mut done: bool = false;
    while !done 
    {
        if pc >= visitor.lines.len()
        {
            done = true;
            continue;
        }

        let curr_line: &TacLine = &visitor.lines[pc];

        // DEBUG output the current line
        log::trace!("{}\n", curr_line);

        match curr_line.line_type 
        {
            parser::tac_line_type::TacLineType::ASSIGNMENT => 
            {
                log::trace!("assignment!");

                // DEBUG
                //log::info!("lhs: {}\n", curr_line.lhs);

                // check if lhs exists in the list of variables, otherwise insert it with a default value
                let _lhs_value: u32 = *variable_table.entry(curr_line.lhs.clone()).or_insert(0u32);

                // evaluate rhs
                let rhs_value: u32 = evaluator.evaluate(&symbol_table, &variable_table, &curr_line.expression_2);

                //log::info!("{} = {}\n", &lhs_value, rhs_value);

                variable_table.insert(curr_line.lhs.clone(), rhs_value);

                pc = pc + 1usize;
            }, 

            parser::tac_line_type::TacLineType::IF_STATEMENT => 
            {
                log::trace!("if statement!");

                let predicate: u32 = evaluator.evaluate(&symbol_table, &variable_table, &curr_line.expression_1);
                if predicate != 0u32
                {
                    // if the option is empty, return 0
                    if curr_line.target_label.is_empty() 
                    {
                        panic!("no target label specified!");
                    }

                    let idx_option = symbol_table.get(&curr_line.target_label);
                    if idx_option.is_none()
                    {
                        panic!("unknown target label used!");
                    }

                    let idxx: u32 = *idx_option.unwrap();
                    pc = idxx as usize; 
                }
                else
                {
                    pc = pc + 1usize;
                }
            },

            parser::tac_line_type::TacLineType::PRINT => 
            {
                let format_string: String = curr_line.expression_1.as_ref().unwrap().value.clone();

                // resolve variable name to it's value
                let elem_key: &String = &curr_line.parameter_list[0];
                let elem_value: Option<&u32> = variable_table.get(elem_key);

                // dynamic format from crate https://crates.io/crates/dyn-fmt
                let test: String = format_string.format(&elem_value);
                
                // print the value
                log::info!("{}\n", test);

                pc = pc + 1usize;
            },

            _ => 
            {
                log::error!("unknown line type!\n");
                done = true;
            },
        }
    }
    log::info!("end of program reached\n");

    // DEBUG - output variable table
    for (variable, value) in &variable_table {
        log::info!("variableValue: {} = {} ({:#04X})\n", variable, value, value);
    }

}

fn init_logging() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        // https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
        .format(|buf, record| {
            //writeln!(
            write!(
                buf,
                //"{}:{} {} [{}] - {}",
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                //chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn log_start() {
    log::trace!("Application starts ...\n");
    log::debug!("Application starts ...\n");
    log::info!("Application starts ...\n");
    log::warn!("Application starts ...\n");
    log::error!("Application starts ...\n");
}

fn log_end() {
    log::trace!("Application terminates.\n");
    log::debug!("Application terminates.\n");
    log::info!("Application terminates.\n");
    log::warn!("Application terminates.\n");
    log::error!("Application terminates.\n");
}
