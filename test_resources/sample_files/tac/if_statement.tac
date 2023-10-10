;if (i <= 10) goto TARGET_LABEL

;if (i > x) goto LABEL_1

;if (_t2 == 0) Goto _L1

;if (_t2 == _t3) Goto _L1

;if ((_t2 && _t3) == (_t4 && _t5)) Goto _L1

;if (true) Goto _L1
;if (false) Goto _L1

;if (_t2 == true) Goto _L1
;if (_t2 == false) Goto _L1

;if (true == _t2) Goto _L1
;if (false == _t2) Goto _L1

if (true == ( a + b ) * ( c + d )) Goto _L1