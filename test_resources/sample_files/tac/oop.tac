_A.fn:
    BeginFunc _4
        y = x
    EndFunc

main:
    BeginFunc _8
		_t0 = 137
		push _t0
		push a
		call _A.fn
		pop _t0
	EndFunc

/*
_A.fn:
 	BeginFunc _4
 		y = x
	EndFunc

main:
 	BeginFunc _8
 		_t0 = 137
		push _t0
		push a
		call _A.fn
		pop 8 			; remove the topmost eight byte from the stack without placing them in any register
 	EndFunc
*/

/*
_A.fn:
	BeginFunc _4;
		*(this + 4) = x;
		x = *(this + 8);
	EndFunc;

main:
	BeginFunc _8;
		_t0 = 137;
		push _t0;
		push a;
		call _A.fn;
		pop 8;
	EndFunc;
*/

/*
_Base.hi:
	BeginFunc _4;
		_t0 = "Base";
		push _t0;
		LCall _PrintString;
		pop 4;
	EndFunc;

Vtable Base = _Base.hi

_Derived.hi:
	BeginFunc _4;
		_t0 = "Derived";
		push _t0;
		LCall _PrintString;
		pop 4;
	EndFunc;

Vtable Derived = _Derived.hi

main:
	BeginFunc _20;
		_t0 = 4;
		push _t0;
		b = LCall _Alloc;
		pop 4;
		_t1 = Derived;
		*b = _t1;
		_t2 = *b;
		_t3 = *_t2;
		push b;
		ACall _t3;
		pop 4;
	EndFunc;

    */