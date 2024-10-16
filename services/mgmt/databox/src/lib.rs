#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(feature = "package-2022-12")]
pub mod package_2022_12;
#[cfg(feature = "package-2023-03")]
pub mod package_2023_03;
#[cfg(feature = "package-2023-12")]
pub mod package_2023_12;
#[cfg(feature = "package-2024-02-preview")]
pub mod package_2024_02_preview;
#[cfg(all(feature = "default_tag", feature = "package-2023-12"))]
pub use package_2023_12::*;
