let val = lt!(vec![1, 2, 3], "l");
let p = lt!(val.as_ptr(), "l");
unsafe { assert!(*p == 1); }
println!("val: {:?}", &val);
