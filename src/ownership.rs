use std::sync::Arc;

struct MyCell<T> {
    value: T,
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, new_value: T) {
        // unsafe {
        //     std::ptr::write(&self.value as *const _ as *mut T, new_value);
        // }
        self.value = new_value;
    }
}

// fn set_value(source: &mut MyCell<&i32>) {
//     println!("old source: {:?}", source.get());
//     let val = 100;
//     source.set(&val);
//     println!("new source: {:?}", source.get());
// }
//
// #[test]
// fn test_my_cell() {
//     let val = 10;
//     let mut source = MyCell::new(&val);
//     set_value(&mut source);
//     println!("final source: {:?}", source.get());
//     println!("val: {:?}", val);
// }

#[derive(Debug)]
struct MyVec<T, const N: usize>([T; N]);

impl<T, const N: usize> MyVec<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self(data)
    }

    pub fn from_slice_clone(slice: &[T]) -> Self
    where
        T: Clone,
    {
        println!("\n---- from_slice_clone ----");
        assert!(slice.len() >= N, "slice is too short");
        let mut data = core::mem::MaybeUninit::<[T; N]>::uninit();
        let data_ptr = data.as_mut_ptr() as *mut T;
        for i in 0..N {
            unsafe {
                std::ptr::write(data_ptr.add(i), slice[i].clone());
                // slice[i].clone_into(&mut *data_ptr.add(i));
            }
        }
        let ret = unsafe { Self(data.assume_init()) };
        println!("---- from_slice_clone ----\n");
        ret
    }

    pub fn from_slice_copy(slice: &[T]) -> Self
    where
        T: Copy + Clone,
    {
        println!("\n---- from_slice_copy ----");
        assert!(slice.len() >= N, "slice is too short");
        let mut data = core::mem::MaybeUninit::<[T; N]>::uninit();
        let data_ptr = data.as_mut_ptr() as *mut T;
        unsafe {
            data_ptr.copy_from_nonoverlapping(slice.as_ptr(), N);
        }
        let ret = unsafe { Self(data.assume_init()) };
        println!("---- from_slice_copy ----\n");
        ret
    }
}

#[derive(Debug)]
#[repr(transparent)]
struct MyString(String);

impl MyString {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Clone for MyString {
    fn clone(&self) -> Self {
        println!("cloning my string {:?}", self);
        Self(self.0.clone())
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("dropping my string {:?}", self);
    }
}

impl<T: Clone + Copy, const N: usize> From<&[T]> for MyVec<T, N> {
    fn from(slice: &[T]) -> Self {
        Self::from_slice_copy(slice)
    }
}

impl<T: Clone, const N: usize> From<&[T]> for MyVec<T, N> {
    default fn from(slice: &[T]) -> Self {
        Self::from_slice_clone(slice)
    }
}

pub fn test_my_vec() {
    let data: [u32; 32] = [0; 32];
    // let my_vec: MyVec<i32, 3> = MyVec::from_slice_copy(&data);
    let my_vec: MyVec<u32, 32> = MyVec::from(&data[..]);
    println!("my_vec: {:?}", my_vec);

    let data = [
        MyString::from_str("hello"),
        MyString::from_str("world"),
        MyString::from_str("!!!"),
    ];
    // let my_vec: MyVec<MyString, 3> = MyVec::from_slice_clone(&data);
    let my_vec: MyVec<MyString, 3> = MyVec::from(&data[..]);
    println!("my_vec: {:?}", my_vec);

    println!("print data: {:?}", data);
    println!("print my_vec: {:?}", my_vec);
    println!("\nleaving test_my_vec\n");
}
