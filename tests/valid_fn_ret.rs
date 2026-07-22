fn wrap(val: &Vec<i32>) -> *const i32 {
    let p = lt!(val.as_ptr(), "l");
    p
}
let v = lt!(vec![1, 2, 3], "l");
let p = wrap(&v);
unsafe { assert!(*p == 1); }
