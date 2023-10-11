/*
BeginFunc test_func
    _t0 = x * x
EndFunc
*/


BeginFunc test_func
	_t0 = x * x
	_t1 = y * y
	m2 = _t0 + _t1
_L0:
	_t2 = 5 < m2
	if (_t2 == 0) Goto _L1
	m2 = m2 - x
	Goto _L0
_L1:
EndFunc

/*
BeginFunc _8
    _t0 = 137
    push _t0
    push a
    call _A.fn
    pop _t0
EndFunc

BeginFunc _4
    y = x
EndFunc

BeginFunc _8
    _t0 = 137
    push _t0
    push a
    call _A.fn
    pop 8 			; remove the topmost eight byte from the stack without placing them in any register
EndFunc
*/
