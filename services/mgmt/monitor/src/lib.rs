#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-08-scheduledqueryrules")]
pub mod package_2021_08_scheduledqueryrules;
#[cfg(feature = "package-2023-10-01-preview")]
pub mod package_2023_10_01_preview;
#[cfg(feature = "package-preview-2023-01")]
pub mod package_preview_2023_01;
#[cfg(feature = "package-preview-2023-09")]
pub mod package_preview_2023_09;
#[cfg(feature = "package-preview-2024-01")]
pub mod package_preview_2024_01;
#[cfg(all(feature = "default_tag", feature = "package-2021-08-scheduledqueryrules"))]
pub use package_2021_08_scheduledqueryrules::*;
