fn bad_wrap(v: &Vec<i32>) -> *const i32 {
    let r = lt!(v.as_ptr(), "l");
    r
}
let p;
{
    let val = lt!(vec![1, 2, 3], "l");
    p = bad_wrap(&val);
}
unsafe { println!("{}", *p); }
