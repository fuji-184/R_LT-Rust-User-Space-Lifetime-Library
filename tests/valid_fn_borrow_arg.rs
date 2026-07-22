fn len(v: &Vec<i32>) -> usize { v.len() }
let val = lt!(vec![1, 2, 3], "l");
let p = lt!(val.as_ptr(), "l");
let n = len(&val);
unsafe { assert!(*p == 1 && n == 3); }
