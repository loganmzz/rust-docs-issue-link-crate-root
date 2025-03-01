//! Test links:
//!
//! * To external crate: [`serde`]
//! * To current crate: [`rust_docs_issue_link_crate_root_sub`]
//! * To parent crate: [`rust_docs_issue_link_crate_root`]
pub fn bar() {
    rust_docs_issue_link_crate_root::foo();
}