let val = lt!(vec![1, 2, 3], "l");
for _ in 0..3 {
    let p = lt!(val.as_ptr(), "l");
    unsafe { assert!(*p == 1); }
}
let p = lt!(val.as_ptr(), "l");
unsafe { assert!(*p == 1); }
