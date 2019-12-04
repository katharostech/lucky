use lucky::cli;

#[cfg(not(feature = "doc-gen"))]
pub fn main() {
    // TODO: Run function should allow passing in the arguments manually so we can write
    // tests for it.
    cli::run();
}

#[cfg(feature = "doc-gen")]
pub fn main() {
    cli::doc_gen();
}
