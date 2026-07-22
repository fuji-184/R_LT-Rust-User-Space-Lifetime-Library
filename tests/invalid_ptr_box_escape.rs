let p;
{
    let boxed = lt!(Box::new(42), "l");
    p = lt!(&*boxed, "l");
}
unsafe { assert_eq!(*p, 42); }
