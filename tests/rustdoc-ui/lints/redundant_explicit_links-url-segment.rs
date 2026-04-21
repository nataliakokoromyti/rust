#![deny(rustdoc::redundant_explicit_links)]

/// [B](struct.B.html)
//~^ ERROR redundant explicit link target
pub struct A;

pub struct B;
