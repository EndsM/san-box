use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        pub struct $name(u32);

        impl $name {
            pub const fn new(raw: u32) -> Self {
                Self(raw)
            }

            pub const fn get(self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            fn from(raw: u32) -> Self {
                Self::new(raw)
            }
        }

        impl From<$name> for u32 {
            fn from(id: $name) -> u32 {
                id.get()
            }
        }

        impl Display for $name{
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

define_id!(RegionId);
