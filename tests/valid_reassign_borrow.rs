let val = lt!(vec![1, 2, 3], "l");
let other = lt!(vec![4, 5, 6], "l");
let ptr = lt!(val.as_ptr(), "l");
let ptr = lt!(other.as_ptr(), "l");
unsafe { assert!(*ptr == 4); }
