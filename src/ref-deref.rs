//! # ref-deref
//!
//! A reference to a value:
//!
//! 1. `a: &T` means `a` cannot refer to other data, and you cannot change the data `a` refers to.
//! 2. `a: &mut T` means `a` cannot refer to other data, but you can change the data `a` refers to.
//! 3. `mut a: &T` means `a` can refer to other data, but you cannot change the data `a` refers to.
//! 4. `mut a: &mut T` means `a` can refer to other data, and you can change the data `a` refers to.
//!
//! `.` operator automatically dereferences a reference.
//!
//! `=` operator *moves* the value, which means the right hand side value is invalidated, unless it
//! implements `Copy` trait, in which case it is copied.

fn altering_reference() {
    println!("altering_reference");
    let val0 = 0;
    let val1 = 1;
    let mut a = &val0;
    dbg!(val0);
    dbg!(val1);
    dbg!(a);
    a = &val1;
    dbg!(val0);
    dbg!(val1);
    dbg!(a);
}

fn altering_referenced() {
    println!("altering_referenced");
    let mut val = 0;
    dbg!(val);
    let a = &mut val;
    dbg!(a);
    *a = 10;
    dbg!(a);
    // dbg!(a);
}

fn main() {
    // let a = vec![1, 2, 3, 4, 5, 6];
    // let b = &a;
    // let _c = *b;

    altering_reference();
    altering_referenced();
}
