mod parser;
mod common;

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

use crate::parser::tac_visitor_nodes::TacVisitorNodes;
use crate::parser::tacparser::Compilation_unitContextAll;
use crate::parser::tacparser::tacContextType;

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

    load_segment_from_source_code(&mut symbol_table);

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

fn load_segment_from_source_code(/*segments: &mut Vec<Segment>,*/ symbol_table: &mut HashMap<String, u32>) /*-> [u8; RAMEND as usize] */
{
/*
    //
    // Phase - load token into a hashmap
    //

    log::info!("**********************************************\n");
    log::info!("Phase - load token into a hashmap\n");
    log::info!("**********************************************\n");

    let mut token_storage: HashMap<isize, String> = HashMap::new();
    let mut token_value_storage: HashMap<String, isize> = HashMap::new();

    let mut token_file_path: String = String::new();
    token_file_path.push_str("src/parser/assembler_lexer.tokens");

    // open the file in read-only mode (ignoring errors).
    let file = File::open(token_file_path).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {

        // ignore errors.
        let line = line.unwrap();

        // DEBUG show the line and its number.
        log::trace!("{}. {}\n", index + 1, line);

        // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
        let collection: Vec<&str> = line.split('=').collect::<Vec<_>>();

        let token:&str = collection[0];
        let token_idx:i16 = collection[1].parse::<i16>().unwrap();
        let token_idx_as_usize:isize = token_idx.into();

        // at the end of the token file, the individual characters are repeated but
        // the purpose of the token map is to just contain the token labels/names and not
        // the individual characters so break on the first duplicate
        if token_storage.contains_key(&token_idx_as_usize) {
            break;
        }
        token_storage.insert(token_idx_as_usize, token.to_string());

        if token_value_storage.contains_key(&token.to_string()) {
            break;
        }
        token_value_storage.insert(token.to_string(), token_idx_as_usize);
    }
*/
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
    source_code_file_path.push_str("test_resources/sample_files/tac/function_definition.tac"); // test
    //source_code_file_path.push_str("test_resources/sample_files/tac/goto.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/if_statement.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/label.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/print.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/push_pop.tac");
    //source_code_file_path.push_str("test_resources/sample_files/tac/return.tac");

    let srcdir = PathBuf::from(&source_code_file_path);
    log::info!("absolute path: {:?}\n", fs::canonicalize(&srcdir));

    let data = fs::read_to_string(&source_code_file_path).expect("Unable to read file");
    log::trace!("\n{}\n", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    parse_and_encode(/*segments, */input_stream, source_code_file_path.clone()/*, symbol_table */)

}

fn parse_and_encode(/*segments: &mut Vec<Segment>, */
    input_stream: InputStream<&str>, 
    source_file: String,
/*symbol_table: &mut HashMap<String, u32>*/) /*-> [u8; RAMEND as usize] */
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
    //visitor.record.clear();

    let _visitor_result = visitor.visit(&*root);

    log::info!("*************************************************************\n");
    log::info!("Phase - DEBUG - Output all lines after digestion\n");
    log::info!("*************************************************************\n");

    log::info!("\n");
    for line in visitor.lines.iter_mut() {
        log::info!("{}\n", line);
        log::info!("\n");
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
