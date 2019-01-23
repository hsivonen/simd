# `simd`

[![Build Status](https://travis-ci.org/hsivonen/simd.svg?branch=master)](https://travis-ci.org/hsivonen/simd)
[![crates.io](https://meritbadge.herokuapp.com/simd)](https://crates.io/crates/simd)
[![docs.rs](https://docs.rs/simd/badge.svg)](https://docs.rs/simd/)

_This crate no londer builds as of Rust 1.33 nightly_ due to the [removal of compiler features](https://github.com/rust-lang/rust/pull/57416) that this crates depends on. See the [`packed_simd`](https://crates.io/crates/packed_simd) crate instead.

`simd` offers a basic interface to the SIMD functionality of CPUs. (Note: Even prior to Rust 1.33, this crate fails to build unless the target is aarch64, x86_64, i686 (i.e. SSE2 enabled; not i586) or an ARMv7 target (thumb or not) with NEON enabled.)

[Documentation](https://docs.rs/simd)
