let mut val = lt!(vec![1, 2, 3], "x");
let p = lt!(NonNull::new(&mut val), "x");
drop(val);
unsafe { println!("{}", *p.as_ptr()); }