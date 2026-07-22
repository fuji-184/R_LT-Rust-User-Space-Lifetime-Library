let val3 = lt!(vec![1, 2, 3], "x");
{
    let val2 = lt!(vec![4, 5, 6], "x");
    {
        let val1 = lt!(vec![7, 8, 9], "x");
        let p1 = lt!(val1.as_ptr(), "x");
        let p2 = lt!(val2.as_ptr(), "x");
        let p3 = lt!(val3.as_ptr(), "x");
        unsafe { assert!(*p1 == 7 && *p2 == 4 && *p3 == 1); }
    }
    let p2 = lt!(val2.as_ptr(), "x");
    let p3 = lt!(val3.as_ptr(), "x");
    unsafe { assert!(*p2 == 4 && *p3 == 1); }
}
let p3 = lt!(val3.as_ptr(), "x");
unsafe { assert!(*p3 == 1); }
