//! # ratcl
//!
//! `ratcl` allows you to create complex ratatui layouts with a simple API.

use ratatui::{buffer::Buffer, layout::Rect};

/// Defines the `LayoutCell` alias.
pub trait LayoutCell: Fn(Rect, &mut Buffer) {}

/// Implements the `LayoutCell` trait for its respective type.
impl <Type: Fn(Rect, &mut Buffer)> LayoutCell for Type {}
