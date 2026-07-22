mod holder {
    let val = lt!(vec![1, 2, 3], "l");
}
let p = lt!(holder::val.as_ptr(), "l");
unsafe { assert!(*p == 1); }
// note: analysis cleans up mod scope before outer borrow is created
