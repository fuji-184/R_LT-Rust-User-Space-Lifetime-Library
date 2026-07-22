let p;
{
    let val = lt!(vec![1, 2, 3], "x");
    {
        p = lt!(val.as_ptr(), "x");
    }
}
unsafe { assert!(*p == 1); }
