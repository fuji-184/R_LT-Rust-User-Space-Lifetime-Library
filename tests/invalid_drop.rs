let val = lt!(vec![1, 2, 3], "lt1");
let ptr = lt!(val.as_ptr(), "lt1");
drop(val);
unsafe { println!("{}", *ptr); }
