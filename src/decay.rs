pub trait Decay {
    type Output;
}

impl<T> Decay for T {
    default type Output = T;
}

impl<'a, T> Decay for &'a T {
    type Output = T;
}

impl<'a, T> Decay for &'a mut T {
    type Output = T;
}

trait TypeEq<U: ?Sized> {
    const VALUE: bool;
}

impl<T: ?Sized, U: ?Sized> TypeEq<U> for T {
    default const VALUE: bool = false;
}

impl<T: ?Sized> TypeEq<T> for T {
    const VALUE: bool = true;
}

pub struct Decable;

#[test]
fn decay_test() {
    println!(
        "Decay::Output = {}",
        std::any::type_name::<<Decable as Decay>::Output>()
    );
    println!(
        "Decay::Output = {}",
        std::any::type_name::<<&Decable as Decay>::Output>()
    );
    println!(
        "Decay::Output = {}",
        std::any::type_name::<<&mut Decable as Decay>::Output>()
    );
    println!("Decay::Output = {}", std::any::type_name::<&mut Decable>());
    println!("{}", <<Decable as Decay>::Output as TypeEq<Decable>>::VALUE);

    assert_eq!(<<Decable as Decay>::Output as TypeEq<Decable>>::VALUE, true);
    assert_eq!(
        <<&Decable as Decay>::Output as TypeEq<Decable>>::VALUE,
        true
    );
    assert_eq!(
        <<&mut Decable as Decay>::Output as TypeEq<Decable>>::VALUE,
        true
    );
}
