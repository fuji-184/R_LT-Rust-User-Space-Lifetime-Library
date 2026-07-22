let p;
{
    let val = lt!(vec![1, 2, 3], "outer");
    {
        let inner = lt!(vec![4, 5, 6], "inner");
        p = lt!(inner.as_ptr(), "inner");
    }
}
unsafe { assert!(*p == 4); }
