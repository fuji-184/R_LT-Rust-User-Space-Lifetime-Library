let mut val = lt!(vec![1, 2, 3], "x");
let p = lt!(NonNull::new(&mut val), "x");
unsafe { println!("{}", *p.as_ptr()); }