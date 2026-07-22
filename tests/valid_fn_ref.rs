fn peek(v: &Vec<i32>) -> i32 { v[0] }
let val = lt!(vec![1, 2, 3], "l");
let p = lt!(val.as_ptr(), "l");
let x = peek(&val);
unsafe { assert!(*p == 1 && x == 1); }
