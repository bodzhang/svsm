// SPDX-License-Identifier: MIT
//
// Copyright (c) 2024-2025

fn main() {
    println!("cargo:rustc-link-arg=-Tuser/lib/module.lds");
    println!("cargo:rustc-link-arg=-no-pie");
}
