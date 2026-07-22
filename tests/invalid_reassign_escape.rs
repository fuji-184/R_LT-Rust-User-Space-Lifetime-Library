let val = lt!(vec![1, 2, 3], "l");
let ptr;
{
    let inner = lt!(vec![4, 5, 6], "l");
    ptr = lt!(inner.as_ptr(), "l");
}
unsafe { assert!(*ptr == 4); }
