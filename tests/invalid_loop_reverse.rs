let mut p = std::ptr::null();
for _ in 0..3 {
    p = lt!(val.as_ptr(), "l");
    let val = lt!(vec![1, 2, 3], "l");
    unsafe { assert!(*p == 1); }
}
