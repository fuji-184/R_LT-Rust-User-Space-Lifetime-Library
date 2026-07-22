fn id(x: i32) -> i32 { x }
fn wrap(v: &Vec<i32>) -> (&Vec<i32>, *const i32) { (v, v.as_ptr()) }
let val = lt!(vec![1, 2, 3], "a");
let p = lt!(val.as_ptr(), "a");
let n = lt!(id(val.len()), "b");
unsafe { assert!(*p == 1 && n == 3); }
