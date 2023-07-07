pub mod vec;
pub mod vec2;

pub fn index<T>(idx: usize, arr: &[T]) -> Option<&T> {
    if idx < arr.len() {
        unsafe { Some(<[T]>::get_unchecked(arr, idx)) }
    } else {
        None
    }
}

#[test]
fn indexing_test() {
    let v = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(index(3, &v), Some(&4));
}
