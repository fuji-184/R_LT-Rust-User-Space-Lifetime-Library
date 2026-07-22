let p;
{
    fn inner(v: &Vec<i32>) -> *const i32 {
        let r = lt!(v.as_ptr(), "l");
        r
    }
    {
        let val = lt!(vec![1, 2, 3], "l");
        p = lt!(inner(&val), "l");
    }
}
unsafe { assert!(*p == 1); }
