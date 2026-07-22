let val = lt!(vec![1, 2, 3], "l");
async {
    let p = lt!(val.as_ptr(), "l");
    unsafe { assert!(*p == 1); }
};
