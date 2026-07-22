let ptr;
{
    let val = lt!(vec![1, 2, 3], "lt1");
    ptr = lt!(val.as_ptr(), "lt1");
}
unsafe { println!("{}", *ptr); }
