fn consume(a: Vec<i32>, b: Vec<i32>) {}
let a = lt!(vec![1, 2, 3], "x");
let b = lt!(vec![4, 5, 6], "x");
let pa = lt!(a.as_ptr(), "x");
let pb = lt!(b.as_ptr(), "x");
consume(a, b);
unsafe { assert!(*pa == 1 && *pb == 4); }
