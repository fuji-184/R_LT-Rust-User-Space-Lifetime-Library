fn get_ptr(v: &Vec<i32>) -> *const i32 { v.as_ptr() }
let val = lt!(vec![1, 2, 3], "l");
{
    let p = lt!(get_ptr(&val), "l");
    unsafe { assert!(*p == 1); }
}
