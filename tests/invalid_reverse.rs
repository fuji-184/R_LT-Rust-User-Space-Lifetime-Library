let ptr = lt!(val.as_ptr(), "lt1");
let val = lt!(vec![1, 2, 3], "lt1");
unsafe { println!("{}", *ptr); }
