let val = lt!(vec![1, 2, 3], "l");
std::thread::scope(|s| {
    let p = lt!(val.as_ptr(), "l");
    s.spawn(|| unsafe { assert!(*p == 1); });
});
