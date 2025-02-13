//! libc - Raw FFI bindings to platforms' system libraries
#![crate_name = "libc"]
#![crate_type = "rlib"]
#![allow(
    renamed_and_removed_lints, // Keep this order.
    unknown_lints, // Keep this order.
    bad_style,
    overflowing_literals,
    improper_ctypes,
    // This lint is renamed but we run CI for old stable rustc so should be here.
    redundant_semicolon,
    redundant_semicolons,
    unused_macros,
    unused_macro_rules,
    // FIXME(1.0): temporarily allow dead_code to fix CI:
    // - https://github.com/rust-lang/libc/issues/3740
    // - https://github.com/rust-lang/rust/pull/126456
    dead_code,
)]
#![cfg_attr(libc_deny_warnings, deny(warnings))]
// Attributes needed when building as part of the standard library
#![cfg_attr(feature = "rustc-dep-of-std", feature(link_cfg, no_core))]
#![cfg_attr(feature = "rustc-dep-of-std", allow(internal_features))]
// Enable extra lints:
#![cfg_attr(feature = "extra_traits", deny(missing_debug_implementations))]
#![deny(missing_copy_implementations, safe_packed_borrows)]
#![cfg_attr(not(feature = "rustc-dep-of-std"), no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]

#[macro_use]
mod macros;

cfg_if! {
    if #[cfg(feature = "rustc-dep-of-std")] {
        extern crate rustc_std_workspace_core as core;
    }
}

pub use core::ffi::c_void;

cfg_if! {
    // This configuration comes from `rust-lang/rust` in `library/core/src/ffi/mod.rs`.
    if #[cfg(all(
        not(windows),
        // FIXME(ctest): just use `target_vendor` = "apple"` once `ctest` supports it
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "visionos",
        )),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "csky",
            target_arch = "hexagon",
            target_arch = "msp430",
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "riscv64",
            target_arch = "riscv32",
            target_arch = "s390x",
            target_arch = "xtensa",
        )
    ))] {
        pub type c_char = u8;
    } else {
        pub type c_char = i8;
    }
}

cfg_if! {
    if #[cfg(windows)] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod windows;
        pub use crate::windows::*;

        prelude!();
    } else if #[cfg(target_os = "fuchsia")] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod fuchsia;
        pub use crate::fuchsia::*;

        prelude!();
    } else if #[cfg(target_os = "switch")] {
        mod fixed_width_ints;
        pub use fixed_width_ints::*;

        mod switch;
        pub use switch::*;

        prelude!();
    } else if #[cfg(target_os = "vxworks")] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod vxworks;
        pub use crate::vxworks::*;

        prelude!();
    } else if #[cfg(target_os = "solid_asp3")] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod solid;
        pub use crate::solid::*;

        prelude!();
    } else if #[cfg(unix)] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod unix;
        pub use crate::unix::*;

        prelude!();
    } else if #[cfg(target_os = "hermit")] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod hermit;
        pub use crate::hermit::*;

        prelude!();
    } else if #[cfg(target_os = "teeos")] {
        mod fixed_width_ints;
        pub use fixed_width_ints::*;

        mod teeos;
        pub use teeos::*;

        prelude!();
    } else if #[cfg(target_os = "trusty")] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod trusty;
        pub use crate::trusty::*;

        prelude!();
    } else if #[cfg(all(target_env = "sgx", target_vendor = "fortanix"))] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod sgx;
        pub use crate::sgx::*;

        prelude!();
    } else if #[cfg(any(target_env = "wasi", target_os = "wasi"))] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod wasi;
        pub use crate::wasi::*;

        prelude!();
    } else if #[cfg(target_os = "xous")] {
        mod fixed_width_ints;
        pub use crate::fixed_width_ints::*;

        mod xous;
        pub use crate::xous::*;

        prelude!();
    } else {
        // non-supported targets: empty...
    }
}
