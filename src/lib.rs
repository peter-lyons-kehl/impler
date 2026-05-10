#![no_std]

use core::fmt::{self, Display, Formatter};

/// Default type param for [Displays02Plus]'s generic param `OTHER`. We can't use unit type `()`,
/// because Rust may add `impl` for the user-specified trait later.
///
/// Zero-sized. Like Rust's "never" type, it will be optimized out in enum
/// variants etc.
pub enum Never {}
const _: () = {
    assert!(core::mem::size_of::<Never>() == 0);
};

impl Display for Never {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub trait Displays02Trait: Display {
    type T01: Display;
    type T02: Display;
}

pub enum Displays02Plus<T01: Display = Never, T02: Display = Never, OTHER: Display = Never> {
    //@TODO separate enum, and wrap transparent
    T01(T01),
    T02(T02),
    Other(OTHER),
}

pub type Displays02<T01, T02> = Displays02Plus<T01, T02, Never>;

impl<T01: Display, T02: Display, OTHER: Display> Displays02Plus<T01, T02, OTHER> {
    pub fn new_01(v: T01) -> Self {
        Self::T01(v)
    }
    pub fn new_02(v: T02) -> Self {
        Self::T02(v)
    }

    // @TODO separate function name for each trait; OR: support one trait only - user can have blanket impl.
    //
    // @TODO inner = by impl only; inner_mut
    fn inner_ref(&self) -> &dyn Display {
        match self {
            Self::T01(inner) => inner,
            Self::T02(inner) => inner,
            Self::Other(inner) => inner,
        }
    }

    fn by_ref<R, F: FnOnce(&dyn Display) -> R>(&self, apply: F) -> R {
        apply(self.inner_ref())
    }
    /* Not possible: fn pointer can't use `impl TraitXyz`

    fn by_impl_01<A01, R>(&self, apply: fn(&impl Display, A01), a01: A01) -> R {
        apply(self.inner_ref())
    }*/
}
impl<T01: Display, T02: Display, OTHER: Display> Display for Displays02Plus<T01, T02, OTHER> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Self::by_ref(&self, |s| s.fmt(f))
    }
}
impl<T01: Display, T02: Display, OTHER: Display> Displays02Trait
    for Displays02Plus<T01, T02, OTHER>
{
    type T01 = T01;
    type T02 = T02;
}

/// Without traits like [Displays02PlusTrait], having generic param `OTHER` (rather than using its
/// default) in `impl` of [Display] for [Displays02Plus] makes calls like
/// `Displays02Plus::new_01(true)` ambiguous if that function's return type is just `impl
/// Displays02Trait` (or `impl Display`).
pub trait Displays02PlusTrait<OTHER: Display = Never>: Displays02Trait {}
impl<T01: Display, T02: Display, OTHER: Display> Displays02PlusTrait<OTHER>
    for Displays02Plus<T01, T02, OTHER>
{
}

/*impl<T01: Display, T02: Display> From<T01> for Displays02<T01, T02> {
    fn from(value: T01) -> Self {
        Self::new_01(value)
    }
}*/
// CONFLICTING:
/*impl<T01: Display, T02: Display> From<T02> for Displays02<T01, T02> {
    fn from(value: T02) -> Self {
        Self::new_02(value)
    }
}
// foreign trait:
impl<T01: Display, T02: Display> Into<Displays02<T01, T02>> for T01 {
    fn into(self) -> Displays02<T01, T02> {
        Displays02::new_01(self)
    }
}*/
/*pub trait MoveIntoDisplays02 {
    fn move_into<T01: Display, T02: Display>(self) -> Displays02<T01, T02>;
}
impl<T01: Display> MoveIntoDisplays02 for T01 {
    fn move_into<T02: Display>(self) -> Displays02<T01, T02> {
        todo!()
    }
}*/

type Displays01<T, OTHER = Never> = Displays02Plus<T, OTHER>;
pub trait Displays01PlusExt01<T01: Display> {
    //@TODO seal
    fn into_01(self) -> Displays02Plus<T01>;
}
impl<T01: Display> Displays01PlusExt01<T01> for T01 {
    fn into_01(self) -> Displays02Plus<T01> {
        Displays02Plus::new_01(self)
    }
}

pub trait Displays02PlusExt01<T01: Display, T02: Display> {
    //@TODO seal
    fn into_01(self) -> Displays02Plus<T01, T02>;
}
pub trait Displays02PlusExt02<T01: Display, T02: Display> {
    fn into_02(self) -> Displays02Plus<T01, T02>;
}
impl<T01: Display, T02: Display> Displays02PlusExt01<T01, T02> for T01 {
    fn into_01(self) -> Displays02Plus<T01, T02> {
        Displays02Plus::new_01(self)
    }
}
impl<T01: Display, T02: Display> Displays02PlusExt02<T01, T02> for T02 {
    fn into_02(self) -> Displays02Plus<T01, T02> {
        Displays02Plus::new_02(self)
    }
}

/// @TODO finish
pub trait Displays03PlusExt01<T01: Display, T02: Display, T03: Display> {
    //@TODO seal
    fn into_01(self) -> Displays02Plus<T01, T02>; //@TODO Displays03Plus
}

pub trait Displays03PlusExt02<T01: Display, T02: Display, T03: Display> {
    fn into_02(self) -> Displays02Plus<T01, T02>;
}
pub trait Displays03PlusExt03<T01: Display, T02: Display, T03: Display> {
    fn into_03(self) -> Displays02Plus<T01, T02>;
}

impl<T01: Display, T02: Display, T03: Display> Displays03PlusExt01<T01, T02, T03> for T01 {
    fn into_01(self) -> Displays02Plus<T01, T02> {
        Displays02Plus::new_01(self)
    }
}
impl<T01: Display, T02: Display, T03: Display> Displays03PlusExt02<T01, T02, T03> for T02 {
    fn into_02(self) -> Displays02Plus<T01, T02> {
        Displays02Plus::new_02(self)
    }
}
impl<T01: Display, T02: Display, T03: Display> Displays03PlusExt03<T01, T02, T03> for T03 {
    fn into_03(self) -> Displays02Plus<T01, T02> {
        todo!() //Displays03Plus::new_03(self)
    }
}
