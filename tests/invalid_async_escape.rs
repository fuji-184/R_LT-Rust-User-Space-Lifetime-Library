let p;
{
    let val = lt!(vec![1, 2, 3], "l");
    p = lt!(val.as_ptr(), "l");
    async { unsafe { assert!(*p == 1); } };
}
