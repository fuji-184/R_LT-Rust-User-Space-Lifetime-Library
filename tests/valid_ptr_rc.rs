let rc = lt!(std::rc::Rc::new(42), "l");
let p = lt!(std::rc::Rc::as_ptr(&rc), "l");
unsafe { assert_eq!(*p, 42); }
