let p;
{
    let val = lt!(vec![1, 2, 3], "l");
    p = lt!(val.as_ptr(), "l");
    std::thread::spawn(|| unsafe { assert!(*p == 1); });
}
