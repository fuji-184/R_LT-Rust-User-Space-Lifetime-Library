let mut p = std::ptr::null();
loop {
    let val = lt!(vec![1, 2, 3], "l");
    if true {
        p = lt!(val.as_ptr(), "l");
    }
}
