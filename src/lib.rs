//! # Janus Array
//!
//! A Rust data structure for efficient dual-access patterns in hierarchical binary data.
//!
//! Named after the two-faced Roman god Janus, this library provides both hierarchical 
//! navigation (O(1)) and offset-based lookup (O(log n)) in a single unified structure,
//! without the overhead of maintaining separate indices.
//!
//! ## Problem
//!
//! When parsing binary file formats with nested structures (executable load commands, 
//! filesystem metadata, protocol packets), you typically need two access patterns:
//!
//! 1. **Direct navigation**: `file.slices[i].commands[j].elements[k]` - should be O(1)
//! 2. **Reverse lookup**: "What structure contains byte offset 0x47382?" - often O(n)
//!
//! Most solutions optimize for one pattern at the expense of the other, or maintain 
//! expensive auxiliary data structures.
//!
//! ## Solution
//!
//! The Janus Array provides both access patterns efficiently:
//!
//! - **Forward access**: O(1) hierarchical navigation
//! - **Reverse lookup**: O(log n) binary search through the hierarchy  
//! - **Zero overhead**: No auxiliary indices - the hierarchy itself is the search structure
//!
//! ## Quick Start
//!
//! ```rust
//! use janus_array::{File, DiskOffsets, Slices, Commands, CommandElements};
//! use std::ops::Range;
//!
//! // Create a file structure
//! let mut file = File::with_size(1024);
//!
//! // Build hierarchy manually (normally you'd parse from binary data)
//! let mut slice = Slices::default();
//! slice.populate_values(0, 512, 0, 512, 0);
//!
//! let mut command = Commands::default();
//! command.populate_values(100, 200, 100, 100, 0);
//!
//! let mut element = CommandElements::default();
//! element.populate_values(150, 180, 50, 30, 0);
//!
//! // Direct access - O(1) when you know the path
//! // let data = &file.slices[0].commands[0].elements[0];
//!
//! // Reverse lookup - O(log n) to find structure containing an offset
//! match file.find_address(175) {
//!     Ok(coords) => {
//!         println!("Offset 175 found at slice: {:?}, command: {:?}, element: {:?}", 
//!                  coords.slice, coords.command, coords.element);
//!     }
//!     Err(e) => println!("Address not found: {:?}", e),
//! }
//! ```
//!
//! ## Performance
//!
//! For a structure with S slices, C commands, and E elements:
//!
//! - **Direct access**: O(1)
//! - **Reverse lookup**: O(log S + log C + log E)
//!
//! In practice, S is typically small (≤ 3), so this reduces to **O(log C + log E)**.
//!
//! ## Architecture
//!
//! All levels implement the [`DiskOffsets`] trait, enabling recursive search:
//!
//! ```rust
//! use janus_array::{DiskOffsets, OffsetLayoutsError, Coordinates};
//! use std::ops::Range;
//!
//! pub trait DiskOffsets {
//!     fn find_address(&mut self, addr: u64) -> Result<Coordinates, OffsetLayoutsError>;
//!     fn get_absolute_range(&self) -> Range<u64>;
//!     // ... other methods
//! }
//! ```
//!
//! ## Use Cases
//!
//! - Binary file format parsers (Mach-O, ELF, PE)
//! - Filesystem tools requiring block-to-inode mapping
//! - Network protocol analysis with nested structures  
//! - Debugging tools for address-to-symbol resolution
//! - Any scenario requiring both hierarchical and offset-based access

pub mod file;
pub mod slices;
pub mod commands;
pub mod cmd_elements;
pub mod disk_offsets;
pub mod coordinates;
pub mod types;

// Main public exports
pub use file::File;
pub use slices::Slices;
pub use commands::Commands;
pub use cmd_elements::CommandElements;
pub use disk_offsets::{DiskOffsets, OffsetLayoutsError};
pub use coordinates::Coordinates;
pub use types::ChildrenArray;

/// Convenience module for importing commonly used types
///
/// ```rust
/// use janus_array::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        File, Slices, Commands, CommandElements,
        DiskOffsets, OffsetLayoutsError, Coordinates, ChildrenArray
    };
}