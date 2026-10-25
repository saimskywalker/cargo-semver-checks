#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

// These items live in a private module, so only their exported symbols are part of the
// interface under test and the Rust-path-based variadic lints do not report them.
mod abi_only {
    // A free function exported with `#[no_mangle]`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn no_mangle_function(_value: i32, _: ...) {}

    // A free function exported with `#[export_name]`.
    #[unsafe(export_name = "export_name_symbol")]
    pub unsafe extern "C" fn export_name_function(_value: i32, _: ...) {}

    pub struct Provider;

    // An exported inherent method.
    impl Provider {
        #[unsafe(export_name = "inherent_method_symbol")]
        pub unsafe extern "C" fn inherent_method(_value: i32, _: ...) {}
    }

    pub trait Hooks: Sized {
        unsafe extern "C" fn fixed(value: i32);
        unsafe extern "C" fn variadic(value: i32, _: ...);
    }

    // The trait method signatures stay the same, but the exported symbol `hook`
    // moves between the non-variadic and the variadic implementation.
    impl Hooks for Provider {
        unsafe extern "C" fn fixed(_value: i32) {}

        #[unsafe(export_name = "hook")]
        unsafe extern "C" fn variadic(_value: i32, _: ...) {}
    }

    // Exported functions whose variadic status does not change should not be reported.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn unchanged_fixed(_value: i32) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn unchanged_variadic(_value: i32, _: ...) {}

    // The opposite change should only be reported by the other directional lint.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn opposite_direction(_value: i32) {}
}
