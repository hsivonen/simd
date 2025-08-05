# `simd`

_This crate is __OBSOLETE__. It no londer builds as of Rust 1.33 nightly_ due to the [removal of compiler features](https://github.com/rust-lang/rust/pull/57416) that this crates depends on. See `core::simd` instead (nightly-only as of 2025-08-05). On ARMv7, be sure to check the status of a [performance regression](https://github.com/rust-lang/portable-simd/issues/146) relative to this crate.

`simd` offers a basic interface to the SIMD functionality of CPUs. (Note: Even prior to Rust 1.33, this crate fails to build unless the target is aarch64, x86_64, i686 (i.e. SSE2 enabled; not i586) or an ARMv7 target (thumb or not) with NEON enabled.)

[Documentation](https://docs.rs/simd)
