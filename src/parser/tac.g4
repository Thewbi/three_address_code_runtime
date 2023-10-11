parser grammar tac;
options { tokenVocab=tac_lexer; }


compilation_unit :
	NEWLINE* 
    source_line ( NEWLINE* source_line )* NEWLINE* 
    EOF 
	;
	
source_line :
	( label COLON )? assignment
	|
	( label COLON )? control_flow
	|
	( label COLON )? function_call
	|
	( label COLON )
	|
	( class_method_identifier COLON )
	|
	function_definition
	|
	vtable_assignment
	;
	
label :
	IDENTIFIER
	;
	
assignment :
	left_hand_side EQUALS expression
	|
	left_hand_side EQUALS lcall_expression
	;
	
left_hand_side :
    expression
    ;

lcall_expression :
	LCALL ( function_identifier | class_method_identifier )
	;

predicate :
	expression equals_operator ( TRUE | FALSE )
	|
	( TRUE | FALSE ) equals_operator expression
	|
	expression
	|
	TRUE 
	| 
	FALSE
	;
	
expression :
	OPENING_BRACKET expression CLOSEING_BRACKET
    |
	expression binary_operator expression
	|
	unary_operator expression
	|
    operand
	;
	
unary_operator :
	AMPERSAND
	|
	ASTERISK
	|
	MINUS
	|
	PLUS
	;
	
binary_operator :
	'+'
	|
	'-'
	|
	'*'
	|
	'/'
	|
	or_operator
	|
	and_operator
	|
	equals_operator
	|
	LT
	|
	GT
	|
	LTE
	|
	GTE
	;
	
or_operator :
    BAR BAR
    ;
	
and_operator :
    AMPERSAND AMPERSAND
    ;
	
equals_operator :
	EQUALS EQUALS
	;
	
operand :
    NUMBER
	|
	IDENTIFIER
	|
	STRING
	|
	function_call
	|
	class_method_identifier
	;
	
function_call :
	SIZEOF expression
	|
	SQRT expression
	|
	PUSH expression
	|
	POP ( IDENTIFIER | NUMBER )
	|
	PRINT STRING ( COMMA parameter_list )?
	|
	BREAK
	;
	
//parameter_list :
//	parameter
//	|
//	parameter_list COMMA parameter 
//	;

parameter_list :
	parameter
	|
	parameter COMMA parameter_list
	;
	
parameter :
	expression
	;
	
control_flow :
	( if_statement )? GOTO IDENTIFIER
	|
	( CALL | LCALL | ACALL ) function_identifier
	|
	( CALL | LCALL | ACALL ) class_method_identifier
	|
	RETURN ( expression )?
	;
	
if_statement :
	IF OPENING_BRACKET predicate CLOSEING_BRACKET
	;

function_identifier :
	IDENTIFIER
	;
	
function_definition :
	BEGINFUNC IDENTIFIER
		function_body NEWLINE?
	ENDFUNC
	;

function_body :
	NEWLINE* 
    source_line ( NEWLINE* source_line )* NEWLINE* 
	;
	
//function_body :
//	( 
//		( label COLON )
//		|
//		source_line
//		|
//		NEWLINE
//	)+
//	;

//
// TAC for Objects
//
// https://web.stanford.edu/class/archive/cs/cs143/cs143.1128/lectures/13/Slides13.pdf
//

// class A {
//     void fn(int x) {
//         int y;
//         y = x;
//     }
// }
//
// int main() {
//     A a;
//     a.fn(137);
// }

class_method_identifier :
	IDENTIFIER DOT IDENTIFIER
	;

vtable_assignment :
	VTABLE IDENTIFIER EQUALS class_method_list
	;

class_method_list :
	class_method_identifier
	|
	class_method_list COMMA class_method_identifier
	;


