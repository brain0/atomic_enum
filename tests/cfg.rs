#![allow(unexpected_cfgs)] // multics is deliberately always false

use atomic_enum::atomic_enum;

#[atomic_enum]
enum MyEnum {
    Foo,
    #[cfg(target_os = "multics")]
    Bar,
    #[cfg(not(target_os = "multics"))]
    Baz
}

// Foo and Baz should both be constructible.  Bar should not be, but that can only be verified from
// a doc test.
#[test]
fn construction() {
    let _ = AtomicMyEnum::new(MyEnum::Foo);
    let _ = AtomicMyEnum::new(MyEnum::Baz);
}
