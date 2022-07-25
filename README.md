[![cargo version](https://img.shields.io/crates/v/atomic_enum_2021.svg)](https://crates.io/crates/atomic_enum_2021) 
[![docs.rs version](https://img.shields.io/docsrs/atomic_enum_2021)](https://docs.rs/atomic_enum_2021/latest/atomic_enum_2021/)
# atomic_enum_2021

 An attribute to create an atomic wrapper around a C-style enum.

 Internally, the generated wrapper uses an `AtomicUsize` to store the value.
 The atomic operations have the same semantics as the equivalent operations
 of `AtomicUsize`.

Forked and maintained by `l1npengtul` to remove warnings when compiling for modern versions of Rust.  

 # Example
 ```
 # use atomic_enum::atomic_enum;
 # use std::sync::atomic::Ordering;
 #[atomic_enum]
 #[derive(PartialEq)]
 enum CatState {
     Dead = 0,
     BothDeadAndAlive,
     Alive,
 }

 let state = AtomicCatState::new(CatState::Dead);
 state.store(CatState::Alive, Ordering::Relaxed);

 assert_eq!(state.load(Ordering::Relaxed), CatState::Alive);
 ```

 This attribute does not use or generate any unsafe code.

# MSRV
Rust 2021 Edition (1.56)