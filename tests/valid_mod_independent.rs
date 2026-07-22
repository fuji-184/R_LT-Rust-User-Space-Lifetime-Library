{ let a = lt!(vec![1, 2, 3], "x"); let pa = lt!(a.as_ptr(), "x"); unsafe { assert!(*pa == 1); } }
{ let b = lt!(vec![4, 5, 6], "y"); let pb = lt!(b.as_ptr(), "y"); unsafe { assert!(*pb == 4); } }
