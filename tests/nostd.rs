// Purpose: This integration test checks that atomic_enum can be used in
// a no_std environment.

#![no_std]

use core::sync::atomic::Ordering;

use atomic_enum::atomic_enum;

#[atomic_enum]
#[derive(PartialEq, Eq)]
enum FooBar {
    Foo,
    Bar,
}

#[test]
fn test_no_std_use() {
    let fb = AtomicFooBar::new(FooBar::Foo);
    let prev = fb.compare_and_swap(FooBar::Foo, FooBar::Bar, Ordering::SeqCst);
    assert_eq!(prev, FooBar::Foo);

    let prev_fail = fb.compare_and_swap(FooBar::Foo, FooBar::Bar, Ordering::SeqCst);
    assert_eq!(prev_fail, FooBar::Bar);
}
