//! Cursor based pagination for SQLx.
//!
//! It is built around [sqlx::QueryBuilder], and has the same security level.
//!
//! See [Pagination] for the API, and the examples directory in the Git repository for an example.

mod lib_page;

pub use lib_page::Pagination;