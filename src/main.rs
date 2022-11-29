mod rustonomicon;

fn main() {
    let va = vec![1, 2, 3, 4];
    let vb = vec![&va[0], &va[1], &va[2], &va[3]];
    let vc = &vb;
    let vd = vc.clone();
    let ve = vc.to_owned();
    println!("{:?}", va);
    println!("{:?}", vb);
    println!("{:?}", vc);
    println!("{:?}", vd);
    println!("{:?}", ve);

    rustonomicon::vec::run_vec();
}

// #[test]
// fn lifetimes() {
//     #[derive(Debug)]
//     struct Foo;
//     impl Foo {
//         fn mutate_and_share(&mut self) -> &Self {
//             &*self
//         }

//         fn share(&self) {}
//     }

//     let mut foo = Foo;
//     let loan = foo.mutate_and_share();
//     foo.share();
//     println!("{:?}", loan);
// }

#[test]
fn higher_rank_trait_bounds() {
    struct Closure<F> {
        data: (u8, u16),
        func: F,
    }

    impl<F> Closure<F>
    where
        F: for<'a> Fn(&'a (u8, u16)) -> &'a u8,
    {
        fn call(&self) -> &u8 {
            (self.func)(&self.data)
        }
    }

    fn do_it(data: &(u8, u16)) -> &u8 {
        &data.0
    }

    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("result is {}", clo.call());
}

#[test]
fn subtyping_and_variance() {
    // Subtype(subset)

    trait Animal {
        fn snuggle(&self);
        fn eat(&self);
    }

    trait Cat: Animal {
        fn meow(&self);
    }

    trait Dog: Animal {
        fn bark(&self);
    }

    fn _love(pet: &dyn Animal) {
        pet.snuggle();
    }

    struct Cat0;
    struct Dog0;

    impl Animal for Cat0 {
        fn snuggle(&self) {
            println!("cat0 snuggle");
        }

        fn eat(&self) {
            println!("cat0 eat");
        }
    }

    impl Cat for Cat0 {
        fn meow(&self) {
            println!("cat0 meow");
        }
    }

    impl Animal for Dog0 {
        fn snuggle(&self) {
            println!("dog0 snuggle");
        }

        fn eat(&self) {
            println!("dog0 eat");
        }
    }

    impl Dog for Dog0 {
        fn bark(&self) {
            println!("dog0 bark");
        }
    }

    // fn evil_feeder(pet: &mut impl Animal) {
    //     let spike: &mut dyn Dog = &mut Dog0;
    //     *pet = spike;
    // }

    // Variance, a set of rules governing how subtyping should compose.
    // Variance defines situations where subtyping should be disabled.
    // Variance is a property that type constructors have with respect to their arguments.
    // A type constructor in Rust is any generic type with unbound arguments.
    // Vec is a type constructor that takes a type T and returns Vec<T>.
    // & and &mut are type constructors that takes two inputs: a lifetime, and a type to point to.
    //
    // A type constructor F's variance is how the subtyping of tis inputs affects the subtyping
    // of its outpus. Given types Sub and Super, where Sub is a subtype of Super.
    //
    // * F is convariant if F<Sub> is a subtype of F<Super> (subtyping "passes through")
    // * F is contravariant if F<Super> is a subtype of F<Sub> (sutyping is "inverted")
    // * F is invariant otherwise (no subtyping relationship exists)
    //
    // If F has multiple type parameters, we can talk about the individual variances by saying
    // that, for example, F<T, U> is convariant over T and invariant over U.
}

#[test]
fn borrow() {
    let mut x = [1, 2, 3];
    // let a = &mut x[0];
    // let b = &mut x[1];
    // println!("{} {}", a, b);

    fn split_at_mut<T>(arr: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        let len = arr.len();
        let ptr = arr.as_mut_ptr();

        unsafe {
            assert!(mid <= len);
            (
                core::slice::from_raw_parts_mut(ptr, mid),
                core::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let (a, b) = split_at_mut(&mut x, 1);
    println!("{:?}", a);
    println!("{:?}", b);
}

#[test]
fn coercions() {
    // Thedot operator will perform a lot of magic to convert types. It will perform
    // auto-referencing, auto-dereferencing, and coercion until types match.
}

// Rut doesn't require the variable to be mutalbe to perform a delayed initialization if every
// branch assigns exactly once.
#[test]
fn maybe_uninit() {
    use std::mem::{self, MaybeUninit};

    const SIZE: usize = 10;

    let x = {
        // Create an uninitialised array of `MaybeUninit`. The `assume_init` is safe because
        // the type we care claiming to have initialised here is a bunch of `MaybeUninit`s,
        // which do not require initialisation.
        let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };

        // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
        // assignment instead of `ptr::write` does not cause the old uninitialised
        // value to be dropped.
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32));
            // wrong, leading to drop of unitialised data
            // unsafe { *x[i].as_mut_ptr() = Box::new(i as u32); }
            unsafe {
                std::ptr::write(x[i].as_mut_ptr(), Box::new(i as u32 + 3));
            }
        }

        unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
    };

    dbg!(x);

    #[derive(Debug)]
    struct Demo {
        field: bool,
    }

    let mut uninit = MaybeUninit::<Demo>::uninit();
    let f_ptr = unsafe { core::ptr::addr_of_mut!((*uninit.as_mut_ptr()).field) };
    unsafe {
        f_ptr.write(true);
    }
    let init = unsafe { uninit.assume_init() };

    dbg!(init);
}

#[test]
fn leaking() {
    fn _drain() {
        // drain is a collections API that moves data out of the container without
        // consuming the container.
        let mut vec = vec![Box::new(0); 4];
        {
            let mut drainer = vec.drain(..);

            // pull out two elements and immediately drop them
            drainer.next();
            drainer.next();

            // get rid of drainer, but don't call its destructor
            std::mem::forget(drainer);
        }

        // Oops, vec[0] was dropped, we're reading a pointer into free'd memory!
        println!("{}", vec[0]);
    }
}
