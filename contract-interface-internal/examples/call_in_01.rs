//! Example of defining an contract to be called by consumer contracts.
//! (the consumer contracts still need to define their CallOut's)

use contract_interface_internal as interface;
use interface::{called_in, CalledIn};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

/// (Trait3 Doc)
#[called_in]
pub trait Trait3< //
        'trait_lt,
        TraitType: std::fmt::Debug,
        const TRAIT_CONST: bool
>: Clone
{
    /// (TRAIT_INTERNAL_CONST Doc)
    const TRAIT_INTERNAL_CONST: bool;

    /// (TraitInternalTypeA Doc)
    type TraitInternalTypeA: Clone + near_sdk::serde::de::DeserializeOwned;

    /// (TraitInternalTypeB Doc)
    type TraitInternalTypeB;

    /// (method_a Doc)
    fn method_a< //
        'method_lt,
        MethodTypeA,
        MethodTypeB
    >(
        &mut self,
        _my_string: String,
        _my_m: TraitType,
        // (_my_y, _my_bool): (MethodTypeA, bool),
        // _my_y2: &'method_lt MethodTypeA,

        // TODO: attribute to implicitly consider
        // Self (aka. _State) as implementing the trait itself
        _my_type_a: Self::TraitInternalTypeA,
    ) -> MethodTypeB
    where
        Self: Trait3<'trait_lt, TraitType, TRAIT_CONST>,
        TraitType: 'trait_lt,
        MethodTypeA: Default,
    {
        unimplemented!()
    }
}

#[called_in]
pub trait Trait4 {
    fn method_a(&mut self, my_bool: bool) {
        unimplemented!()
    }
}

// #[called_in]
impl Trait4 for Struct {
    fn method_a(&mut self, _my_bool: bool) {
        unimplemented!()
    }
}
//
impl CalledIn<interface::Json, interface::Json> for trait_4::method_a::CalledIn<Struct> {
    type State = Struct;
    type Args = trait_4::method_a::Args<Self::State>;
    type Return = trait_4::method_a::Return<()>;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let () = <Self::State as Trait4>::method_a(state, args.my_bool);
            None
        };
        Self::called_in(method_wrapper);
    }
}
//
#[no_mangle]
#[allow(unused_imports)]
pub extern "C" fn my_exported_method() {
    pub type Trait4Exported = trait_4::method_a::CalledIn<Struct>;
    Trait4Exported::exposed_called_in()
}

pub trait Trait {
    /// (Original method_a documentation)
    fn method_a(&mut self, my_string: String);

    /// (Original method_b documentation)
    fn method_b(&mut self, my_string: String, my_bool: bool) -> bool;
}

// created by macro
///
///
/// (Original Trait documentation)
pub mod _trait {
    ///
    ///
    /// (Original method_a documentation)
    pub mod method_a {
        use near_sdk::serde::Deserialize;
        use std::marker::PhantomData;

        ///
        ///
        /// (Original method_a documentation)
        #[derive(Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args<State> {
            pub my_string: String,
            pub _state: PhantomData<State>,
        }

        ///
        ///
        /// /// (Original method_a documentation)
        pub type Return = ();

        ///
        ///
        /// (Original method_a documentation)
        pub struct CalledIn<State> {
            _trait_param: (),
            _method_param: (),
            _state_param: PhantomData<State>,
        }
    }

    ///
    ///
    /// (Original method_b documentation)
    pub mod method_b {
        use near_sdk::serde::Deserialize;
        use std::marker::PhantomData;

        ///
        ///
        /// (Original method_b documentation)
        #[derive(Deserialize)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Args {
            pub my_string: String,
            pub my_bool: bool,
        }

        ///
        ///
        /// (Original method_b documentation)
        pub type Return = bool;

        ///
        ///
        /// (Original method_b documentation)
        pub struct CalledIn<State> {
            _trait_param: (),
            _method_param: (),
            _state_param: PhantomData<State>,
        }
    }
}

// specific
/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct {
    a: u8,
    b: u16,
    c: u32,
}

// specific (where the CalledIn "derive" must happen)
// #[CalledIn]
/// (Trait impl Doc)
impl Trait for Struct {
    fn method_a(&mut self, _my_string: String) {
        unimplemented!()
    }
    fn method_b(&mut self, _my_string: String, _my_bool: bool) -> bool {
        unimplemented!()
    }
}
// created by macro
impl CalledIn<interface::Json, interface::Json> for _trait::method_a::CalledIn<Struct> {
    type State = Struct;
    type Args = _trait::method_a::Args<Self::State>;
    type Return = _trait::method_a::Return;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let () = <Self::State as Trait>::method_a(state, args.my_string);
            None
        };
        Self::called_in(method_wrapper);
    }
}
// created by macro
impl CalledIn<interface::Json, interface::Json> for _trait::method_b::CalledIn<Struct> {
    type State = Struct;
    type Args = _trait::method_b::Args;
    type Return = _trait::method_b::Return;
    type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

    fn exposed_called_in() {
        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
            let res = <Self::State as Trait>::method_b(state, args.my_string, args.my_bool);
            Some(res)
        };
        Self::called_in(method_wrapper);
    }
}

// must be created by macro (or by hand)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_a_manual() {
    #[allow(unused_imports)]
    _trait::method_a::CalledIn::<Struct>::exposed_called_in()
}

// must be created by macro (or by hand)
// #[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn method_b_manual() {
    #[allow(unused_imports)]
    _trait::method_b::CalledIn::<Struct>::exposed_called_in()
}

/// (Original Struct documentation)
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Struct2 {
    a: u8,
    b: u16,
    c: u32,
}

#[near_bindgen]
impl Trait for Struct2 {
    fn method_a(&mut self, _my_string: String) {
        unimplemented!()
    }
    fn method_b(&mut self, _my_string: String, _my_bool: bool) -> bool {
        unimplemented!()
    }
}
