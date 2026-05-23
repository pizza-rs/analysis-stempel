//! Build script for pizza-analysis-stempel.
//!
//! Generates a Rust source file containing the embedded Polish stemming table.

fn main() {
    println!("cargo:rerun-if-changed=data/");
}
