let val = lt!(vec![1, 2, 3], "l");
let p = lt!(val.as_ptr(), "l");
let q = lt!(val.as_ptr(), "l");
let r = lt!(val.as_ptr(), "l");
unsafe { assert!(*p == 1 && *q == 1 && *r == 1); }
