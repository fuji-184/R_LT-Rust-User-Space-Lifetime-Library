fn make_ptr(v: &Vec<i32>) -> *const i32 {
    let r = lt!(v.as_ptr(), "l");
    r
}
let val = lt!(vec![1, 2, 3], "l");
let p = make_ptr(&val);
unsafe { assert!(*p == 1); }
