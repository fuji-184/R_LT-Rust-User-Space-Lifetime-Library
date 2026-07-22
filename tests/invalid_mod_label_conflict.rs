let p;
{
    let val = lt!(vec![1, 2, 3], "shared");
    {
        p = lt!(val.as_ptr(), "shared");
    }
}
