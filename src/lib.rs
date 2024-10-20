//! The externcfn macro allows you to define any number of functions with a familiar Rust syntax while seamlessly converting them into C-compatible functions using the C ABI. These functions can be both const and unsafe, accept any number of arguments, and have an optional return type with the function logic defined inside a block. Additionally, the macro supports attaching various attributes like cfg, inline, and doc to the function.
//!
//! The macro simplifies the creation of C functions by maintaining the ease and clarity of Rust's function definition syntax. It ensures that the generated functions are syntactically correct and directly compatible with other languages that utilize the C ABI.
//!
//!

#[macro_use]
extern crate externcfn_attr;

#[macro_export]
macro_rules! externcfn {
    // Einzelne reguläre Funktion mit Attributen
    ($(#[$meta:meta])* $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[no_mangle]
        $v extern "C" fn $name($($args)*) $(-> $ret)? $body
    };

    // Einzelne reguläre unsafe Funktion mit.Attributen
    ($(#[$meta:meta])* $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[no_mangle]
        $v unsafe extern "C" fn $name($($args)*) $(-> $ret)? $body
    };

    // Einzelne konstante Funktion mit Attributen
    ($(#[$meta:meta])* $v:vis const fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[no_mangle]
        $v const extern "C" fn $name($($args)*) $(-> $ret)? $body
    };

    // Einzelne konstante unsafe Funktion mit.Attributen
    ($(#[$meta:meta])* $v:vis const unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[no_mangle]
        $v const unsafe extern "C" fn $name($($args)*) $(-> $ret)? $body
    };

    // Batch-Definitionen für reguläre Funktionen mit Attributen
    ($($(#[$meta:meta])* $v:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block)*) => {$(
        externcfn! { $(#[$meta])* $v fn $name($($args)*) $(-> $ret)? $body }
    )*};

    // Batch-Definitionen für reguläre Funktionen mit Attributen
    ($($(#[$meta:meta])* $v:vis unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block)*) => {$(
        externcfn! { $(#[$meta])* $v unsafe fn $name($($args)*) $(-> $ret)? $body }
    )*};

    // Batch-Definitionen für konstante Funktionen mit Attributen
    ($($(#[$meta:meta])* $v:vis const fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block)*) => {$(
        externcfn! { $(#[$meta])* $v const fn $name($($args)*) $(-> $ret)? $body }
    )*};

    // Batch-Definitionen für konstante Funktionen mit Attributen
    ($($(#[$meta:meta])* $v:vis const unsafe fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block)*) => {$(
        externcfn! { $(#[$meta])* $v const unsafe fn $name($($args)*) $(-> $ret)? $body }
    )*};
}

pub use externcfn_attr::externcfnattr;
