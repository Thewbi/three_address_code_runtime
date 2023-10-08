
_t1 = x

_t0 = 5             ; expected: (lhs: "_t0", type: ASSIGNMENT, expr_1: Some(Node["5" expr:true LHS:None RHS:None]), expr_2: None, source_file:test_resources/sample_files/tac/assignments.tac, line:1, column:0)



_t2 = _t0 + _t1
t2 = b * t1

t1 = -c

t1 = a * ( b + c )
t1 = ( a + b ) * c
t1 = ( a + b ) * ( c + d )
t1 = ( a + ( b ) ) * ( c + d )