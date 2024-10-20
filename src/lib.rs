//! The externcfn macro allows you to define any number of functions with a familiar Rust syntax while seamlessly converting them into C-compatible functions using the C ABI. These functions can be both const and unsafe, accept any number of arguments, and have an optional return type with the function logic defined inside a block. Additionally, the macro supports attaching various attributes like cfg, inline, and doc to the function.
//!
//! The macro simplifies the creation of C functions by maintaining the ease and clarity of Rust's function definition syntax. It ensures that the generated functions are syntactically correct and directly compatible with other languages that utilize the C ABI.
//!
//!

#[macro_export]
macro_rules! externcfn {
    // Einzelne reguläre Funktion mit Attributen
    ($(#[$meta:meta])* $v:vis $(unsafe)? fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[no_mangle]
        $v $(unsafe)? extern "C" fn $name($($args)*) $(-> $ret)? $body
    };

    // Einzelne konstante Funktion mit Attributen
    ($(#[$meta:meta])* $v:vis $(unsafe)? const fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[no_mangle]
        $v $(unsafe)? const extern "C" fn $name($($args)*) $(-> $ret)? $body
    };

    // Batch-Definitionen für reguläre Funktionen mit Attributen
    ($($(#[$meta:meta])* $v:vis $(unsafe)? fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block)*) => {$(
        externcfn! { $(#[$meta])* $v $(unsafe)? fn $name($($args)*) $(-> $ret)? $body }
    )*};

    // Batch-Definitionen für konstante Funktionen mit Attributen
    ($($(#[$meta:meta])* $v:vis $(unsafe)? const fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block)*) => {$(
        externcfn! { $(#[$meta])* $v $(unsafe)? const fn $name($($args)*) $(-> $ret)? $body }
    )*};
}
