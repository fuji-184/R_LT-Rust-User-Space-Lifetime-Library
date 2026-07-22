fn take_owned(v: Vec<i32>) {}
let val = lt!(vec![1, 2, 3], "l");
let p = lt!(val.as_ptr(), "l");
take_owned(val);
unsafe { assert!(*p == 1); }
