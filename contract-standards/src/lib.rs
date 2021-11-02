#[macro_use]
pub mod fungible_token;
#[macro_use]
pub mod pause;
#[macro_use]
pub mod storage_management;

/// Note:  
/// Because of how `#[macro_use]` works, this module must be
/// at root and must come _after_ the referenced macros
/// are defined.  
/// Ie. This should be the last thing at the root of the project.
pub mod macros {

    /// Based on implementations
    /// for [`crate::fungible_token::FungibleToken`].
    pub mod fungible_token {
        /// Based on impl
        /// of [`crate::fungible_token::FungibleTokenCore`]  
        /// for [`crate::fungible_token::FungibleToken`].
        pub mod extern_fungible_token {
            pub use extern_fungible_token;
        }

        /// Based on impl
        /// of [`crate::fungible_token::resolver::FungibleTokenResolver`]  
        /// for [`crate::fungible_token::FungibleToken`].
        pub mod extern_impl_ft_resolver {
            pub use extern_impl_ft_resolver;
        }

        /// Based on impl
        /// of [`crate::storage_management::StorageManagement`]  
        /// for [`crate::fungible_token::FungibleToken`].
        pub mod extern_impl_storage_management {
            pub use extern_impl_storage_management;
        }
    }

    /// Based on implementations
    /// for [`crate::pause::Pause`].
    pub mod pause {
        /// Based on impl
        /// of [`crate::pause::PauseCore`]  
        /// for [`crate::pause::Pause`].
        pub mod extern_pause {
            pub use extern_pause;
        }

        /// Based on impl
        /// of [`crate::fungible_token::FungibleTokenCore`]  
        /// for [`crate::pause::Pause`].
        pub mod extern_pause_fungible_token {
            pub use extern_pause_fungible_token;
        }
    }
}
