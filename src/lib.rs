pub mod api;
pub mod configuration;
pub mod constants;
pub mod models;
pub mod services;

#[macro_use]
extern crate actix_web;

#[macro_export]
macro_rules! name_of {
    ($name:ident in $ty:ty) => {
        {
            #[allow(dead_code)]
            fn dummy(v: $ty) {
                let _ = &v.$name;
            }
            stringify!($name)
        }
    };

    ($name:ident) => {
        {
            let _ = &$name;
            stringify!($name)
        }
    };

    ($name:ident:: $variant:ident in $ty:ty) => {
        {
            #[allow(dead_code)]
            fn dummy(v: $ty) {
                let _ = &v.$name::$variant;
            }
            stringify!($variant)
        }
    };

    ($name:ident:: $variant:ident) => {
        {
            let _ = &$name::$variant;
            stringify!($variant)
        }
    };
}