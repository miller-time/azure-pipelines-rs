//! Azure Pipelines RS
//!
//! This is a Rust representation of the Azure Pipeline YAML Schema.
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/?view=azure-pipelines>

/// The Azure Pipeline type definitions
pub mod core;

/// Support for pipeline templates
pub mod templates;
