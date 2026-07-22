let a = lt!(String::from("hello"), "lt_a");
let b = lt!(vec![1, 2, 3], "lt_b");
let pa = lt!(a.as_ptr(), "lt_a");
let pb = lt!(b.as_ptr(), "lt_b");
unsafe { println!("{} {}", *pa, *pb); }
