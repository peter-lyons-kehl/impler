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
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
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

//type Displays01<T, OTHER = Never> = Displays02Plus<T, OTHER>;
type Displays02<T01, T02> = Displays02Plus<T01, T02, Never>;

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
/*trait Displays02PlusTrait<OTHER: Display = Never>: Displays02Trait {}
impl<T01: Display, T02: Display, OTHER: Display> Displays02PlusTrait<OTHER>
    for Displays02Plus<T01, T02, OTHER>
{
}*/

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

pub fn ret_disp() -> impl Display {
    let _ = if true {
        Displays02::new_01(true) //@TODO:
                                 // -extension method for T01, T02... - blanket for all Sized
                                 // -extension method for Result<..., ...> success
                                 // -extension method for Result<..., ...> error
    } else {
        Displays02::new_02("hi")
    };
    if true {
        Displays02::new_01(1.2)
    } else {
        let value = 1;
        Displays02::new_02(DisplayFromFn::new(move |f| write!(f, "hi {value}")))
    }
}

pub mod import_selected_ext {
    use crate::Displays01PlusExt01;
    use core::fmt::Display;

    pub fn ret_result_displ() -> Result<(), impl Display> {
        ret_result_displ2trait()
    }
    pub fn ret_result_displ2trait() -> Result<(), impl Display> {
        //pub fn ret_result_displ2trait() -> Result<(), impl Displays02PlusTrait> {
        let result_1 = Err(if true {
            // @TODO Displays02: take a generic param like Display8, Display16, Display32...
            // - all implement a tiny trait DisplayFixed
            //
            // then have blanket:
            //
            // impl<F, DF: DisplayFixed + From<F>> From<F> for Displays02<DF> { forward-here }

            //Displays01::new_01("oh")
            "oh".into_01()

            // problem:
            //
            //Displays02Plus::new_01("oh")

            //Displays02Plus::<_, bool>::new_01("oh")

            // @TODO:
            // - extension method for T01, T02... - blanket for all Sized
            // - extension method for Result<..., ...> success
            // - extension method for Result<..., ...> error
        } else {
            //Displays02Plus::new_02(true)

            //Displays01::new_01("hi")
            "hi".into_01()

            // problem:
            //
            //Displays02Plus::new_01("hi")
        });
        let _ = result_1?;

        let result_2 = Err(if true {
            //Displays01::new_01("hu")
            "hu".into_01()
            //Displays02Plus::new_01("hu")
        } else {
            //Displays02Plus::new_02(false)
            if false {
                //Displays01::new_01("bye")
                "bye".into_01()
                //Displays02Plus::new_01("bye")
            } else {
                "bye".into_01()
            }

            //let value = 1;
            //
            // DisplayFromFn::new(move |f| write!(f, "hi {value}"))
        });
        //let _ = result_2?;
        //
        //Ok(())
        result_2
    }
}

#[repr(transparent)]
pub struct DisplayFromFn<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>>(F);
impl<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>> DisplayFromFn<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}
impl<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>> Display for DisplayFromFn<F> {
    fn fmt(&self, fm: &mut Formatter<'_>) -> fmt::Result {
        self.0(fm)
    }
}
pub fn display_from_fn(
    f: impl Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>,
) -> impl Display {
    DisplayFromFn::new(f)
}
