let val = lt!(vec![1, 2, 3], "l");
let p = lt!(val.as_ptr(), "l");
if true {
    drop(val);
}
unsafe { assert!(*p == 1); }
