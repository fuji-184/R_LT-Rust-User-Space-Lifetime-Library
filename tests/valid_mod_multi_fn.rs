mod lib {
    pub fn make(v: &Vec<i32>) -> *const i32 {
        let p = lt!(v.as_ptr(), "l");
        p
    }
    pub fn take(v: Vec<i32>) -> *const i32 {
        let p = lt!(v.as_ptr(), "l");
        p
    }
}
let val = lt!(vec![1, 2, 3], "l");
let p = lib::make(&val);
{
    let other = lt!(vec![4, 5, 6], "l");
    let q = lib::take(other);
}
unsafe { assert!(*p == 1); }
