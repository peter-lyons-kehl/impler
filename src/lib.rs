#![no_std]

use core::fmt::{self, Display as Trait /* <- @TODO macro param*/, Formatter};

/// Default type param for [Displays02Plus]'s generic param `PLUS`. We can't use unit type `()`,
/// because Rust may add `impl` for the user-specified trait later.
///
/// Zero-sized. Like Rust's "never" type, it will be optimized out in enum
/// variants etc.
pub enum Never {}
const _: () = {
    assert!(core::mem::size_of::<Never>() == 0);
};

fn any<T>() -> T {
    #![allow(unreachable_code)]
    #[cfg(feature = "no-panic")]
    let _ = loop {};
    #[cfg(not(feature = "no-panic"))]
    let __ = unreachable!();
}

impl Trait for Never {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        any()
    }
}

enum TypeInner<T01: Trait = Never, T02: Trait = Never, T03: Trait = Never, PLUS: Trait = Never> {
    //@TODO separate enum, and wrap transparent
    T01(T01),
    T02(T02),
    T03(T03),
    Other(PLUS),
}

#[repr(transparent)]
pub struct Type<T01: Trait = Never, T02: Trait = Never, T03: Trait = Never, PLUS: Trait = Never>(
    TypeInner<T01, T02, T03, PLUS>,
);
pub type Type01<T01> = Type<T01>;
pub type Type01Plus<T01, PLUS> = Type<T01, Never, Never, PLUS>;
pub type Type02<T01, T02> = Type<T01, T02>;
pub type Type02Plus<T01, T02, PLUS> = Type<T01, T02, Never, PLUS>;

//type Displays01<T, PLUS = Never> = Displays02Plus<T, PLUS>;
//type Displays02<T01, T02> = Type<T01, T02, Never>;

impl<T01: Trait, T02: Trait, T03: Trait, PLUS: Trait> Type<T01, T02, T03, PLUS> {
    pub fn new_01(v: T01) -> Self {
        Self(TypeInner::T01(v))
    }
    pub fn new_02(v: T02) -> Self {
        Self(TypeInner::T02(v))
    }
    pub fn new_03(v: T03) -> Self {
        Self(TypeInner::T03(v))
    }

    // @TODO separate function name for each trait; OR: support one trait only - user can have blanket impl.
    //
    // @TODO inner = by impl only; inner_mut
    fn inner_ref(&self) -> &dyn Trait {
        match self {
            Self(TypeInner::T01(inner)) => inner,
            Self(TypeInner::T02(inner)) => inner,
            Self(TypeInner::T03(inner)) => inner,
            Self(TypeInner::Other(inner)) => inner,
        }
    }

    fn by_ref<R, F: FnOnce(&dyn Trait) -> R>(&self, apply: F) -> R {
        apply(self.inner_ref())
    }
    /* Not possible: fn pointer can't use `impl TraitXyz`

    fn by_impl_01<A01, R>(&self, apply: fn(&impl Display, A01), a01: A01) -> R {
        apply(self.inner_ref())
    }*/
}
impl<T01: Trait, T02: Trait, T03: Trait, PLUS: Trait> Trait for Type<T01, T02, T03, PLUS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Self::by_ref(&self, |s| s.fmt(f))
    }
}

trait Displays01PlusExt01<T01: Trait> {
    //@TODO seal
    fn into_01(self) -> Type<T01>;
}
impl<T01: Trait> Displays01PlusExt01<T01> for T01 {
    fn into_01(self) -> Type<T01> {
        Type::new_01(self)
    }
}

pub mod ext_01 {
    use super::{Trait, Type, Type01};
    pub trait _01<T01: Trait> {
        //@TODO seal
        fn into_01(self) -> Type01<T01>;
    }
    impl<T01: Trait> _01<T01> for T01 {
        fn into_01(self) -> Type01<T01> {
            Type::new_01(self)
        }
    }
}
pub mod ext_01plus {
    use super::{Trait, Type, Type01Plus};
    pub trait _01<T01: Trait, PLUS: Trait> {
        //@TODO seal
        fn into_01(self) -> Type01Plus<T01, PLUS>;
    }
    impl<T01: Trait, PLUS: Trait> _01<T01, PLUS> for T01 {
        fn into_01(self) -> Type01Plus<T01, PLUS> {
            Type::new_01(self)
        }
    }
    pub trait Plus<PLUS: Trait>: Trait {}
    impl<T01: Trait, PLUS: Trait> Plus<PLUS> for Type01Plus<T01, PLUS> {}
}
pub mod ext_02 {
    use super::{Trait, Type};
    pub trait _01<T01: Trait, T02: Trait> {
        //@TODO seal
        fn into_01(self) -> Type<T01, T02>;
    }
    pub trait _02<T01: Trait, T02: Trait> {
        //@TODO seal
        fn into_02(self) -> Type<T01, T02>;
    }
    impl<T01: Trait, T02: Trait> _01<T01, T02> for T01 {
        fn into_01(self) -> Type<T01, T02> {
            Type::new_01(self)
        }
    }
    impl<T01: Trait, T02: Trait> _02<T01, T02> for T02 {
        fn into_02(self) -> Type<T01, T02> {
            Type::new_02(self)
        }
    }
}

trait Displays02PlusExt01<T01: Trait, T02: Trait> {
    //@TODO seal
    fn into_01(self) -> Type<T01, T02>;
}
trait Displays02PlusExt02<T01: Trait, T02: Trait> {
    fn into_02(self) -> Type<T01, T02>;
}
impl<T01: Trait, T02: Trait> Displays02PlusExt01<T01, T02> for T01 {
    fn into_01(self) -> Type<T01, T02> {
        Type::new_01(self)
    }
}
impl<T01: Trait, T02: Trait> Displays02PlusExt02<T01, T02> for T02 {
    fn into_02(self) -> Type<T01, T02> {
        Type::new_02(self)
    }
}

/// @TODO finish
trait Displays03PlusExt01<T01: Trait, T02: Trait, T03: Trait> {
    //@TODO seal
    fn into_01(self) -> Type<T01, T02>; //@TODO Displays03Plus
}

trait Displays03PlusExt02<T01: Trait, T02: Trait, T03: Trait> {
    fn into_02(self) -> Type<T01, T02>;
}
trait Displays03PlusExt03<T01: Trait, T02: Trait, T03: Trait> {
    fn into_03(self) -> Type<T01, T02>;
}

impl<T01: Trait, T02: Trait, T03: Trait> Displays03PlusExt01<T01, T02, T03> for T01 {
    fn into_01(self) -> Type<T01, T02> {
        Type::new_01(self)
    }
}
impl<T01: Trait, T02: Trait, T03: Trait> Displays03PlusExt02<T01, T02, T03> for T02 {
    fn into_02(self) -> Type<T01, T02> {
        Type::new_02(self)
    }
}
impl<T01: Trait, T02: Trait, T03: Trait> Displays03PlusExt03<T01, T02, T03> for T03 {
    fn into_03(self) -> Type<T01, T02> {
        todo!() //Displays03Plus::new_03(self)
    }
}

/// Like [core::convert::From], but NOT reflective, so that we don't get conflicts concerning
/// [Type] and friends.
///
/// Just like with [core::convert::From], prefer to implement [ImplFrom] over [IntoImpl], as there
/// is a blanket `impl` of [IntoImpl] for any type that implements [ImplFrom].
pub trait ImplFrom<F> {
    //@TODO seal?
    fn impl_from(f: F) -> Self;
}
/// Like [core::convert::Into], but NOT reflective, so that we don't get conflicts concerning
/// [Type] and friends.
///
/// Just like with [core::convert::From], prefer to implement [ImplFrom] over [IntoImpl], as there
/// is a blanket `impl` of [IntoImpl] for any type that implements [ImplFrom].
pub trait IntoImpl<I> {
    //@TODO seal?
    fn into_impl(self) -> I;
}
impl<F, I: ImplFrom<F>> IntoImpl<I> for F {
    fn into_impl(self) -> I {
        I::impl_from(self)
    }
}
impl<T01: Trait, T02: Trait, T03: Trait, PLUS: Trait, FROM> ImplFrom<FROM>
    for Type<T01, T02, T03, PLUS>
where
    PLUS: From<FROM>,
{
    fn impl_from(f: FROM) -> Self {
        Self(TypeInner::Other(f.into()))
    }
}

impl<T01: Trait, T02: Trait, T03: Trait, PLUS: Trait> From<PLUS> for Type<T01, T02, T03, PLUS> {
    fn from(v: PLUS) -> Self {
        Self(TypeInner::Other(v))
    }
}

pub fn ret_disp() -> impl Trait {
    let _local = if true {
        // Possible: local use, type different to the return type below
        Type02::new_01(true)
    } else {
        Type02::new_02("hi")
    };
    if true {
        // NOT possible - OK:
        //
        //return _local;
    }
    if true {
        Type02::new_01(1.2)
    } else {
        let value = 1;
        Type02::new_02(DisplayFromFn::new(move |f| write!(f, "hi {value}")))
    }
}

pub mod import_ext_01 {
    use crate::ext_01::*;
    use core::fmt::Display;

    pub fn ret_result_displ() -> Result<(), impl Display> {
        ret_result_displ2trait()
    }
    pub fn ret_result_displ2trait() -> Result<(), impl Display> {
        let result_1 = Err(if true {
            //Displays01::new_01("oh")
            "oh".into_01()
            // @TODO:
            // - extension method for Result<..., ...> success
            // - extension method for Result<..., ...> error
        } else {
            "hi".into_01()
        });
        let _ = result_1?;

        let result_2 = Err(if true {
            //Displays01::new_01("hu")
            "hu".into_01()
        } else {
            if false {
                //Displays01::new_01("bye")
                "bye".into_01()
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
pub mod import_ext_01plus {
    use crate::{ext_01plus::*, IntoImpl};
    use core::fmt::Display;

    pub fn ret_result_displ() -> Result<(), impl Display> {
        ret_result_displ2trait()
    }
    pub fn ret_result_displ2trait() -> Result<(), impl Plus<bool>> {
        let result_1 = Err(false);
        let _ = result_1?; // question mark automatically calls .into()

        let result_2 = Err(if true {
            "hu".into_01()
        } else {
            if false {
                true.into_impl()
                // can NOT return just: true
            } else {
                "bye".into_01()
            }
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
impl<F: Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>> Trait for DisplayFromFn<F> {
    fn fmt(&self, fm: &mut Formatter<'_>) -> fmt::Result {
        self.0(fm)
    }
}
pub fn display_from_fn(
    f: impl Fn(&mut Formatter<'_>) -> Result<(), core::fmt::Error>,
) -> impl Trait {
    DisplayFromFn::new(f)
}
