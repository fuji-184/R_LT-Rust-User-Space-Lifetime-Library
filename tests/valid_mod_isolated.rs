{
    let a = lt!(vec![1, 2, 3], "g1");
    let pa = lt!(a.as_ptr(), "g1");
    unsafe { assert!(*pa == 1); }
}
{
    let b = lt!(vec![4, 5, 6], "g2");
    let pb = lt!(b.as_ptr(), "g2");
    unsafe { assert!(*pb == 4); }
}
