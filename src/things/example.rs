pub fn example() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    let r1 =  &mut s;
    // let r2 = &mut s; // can't do this only one borrow at a time.
    r1.push_str(", another item");
    r1.push_str(", another another item");
    println!("{s}")
}

