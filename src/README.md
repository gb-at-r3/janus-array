# Janus Array

A Rust data structure for efficient dual-access patterns in hierarchical binary data.

## Problem

When parsing binary file formats with nested structures (like executable load commands, filesystem metadata, or protocol packets), you often need two types of access:

1. **Hierarchical navigation**: `file.slices[i].commands[j].elements[k]` - O(1)
2. **Offset-based lookup**: "What structure contains byte 0x47382?" - typically O(n)

Most solutions optimize for one pattern at the expense of the other, or maintain expensive auxiliary data structures.

## Solution

The Janus Array (named after the two-faced Roman god) provides both access patterns efficiently using a single unified structure:

- **Direct access**: O(1) hierarchical navigation
- **Reverse lookup**: O(log n) binary search through the hierarchy
- **Zero overhead**: No auxiliary indices or duplicated data

## Features

- Unified trait-based design supporting arbitrary nesting levels
- Built-in coordinate tracking for complex hierarchies
- Comprehensive error handling with typed error variants
- Memory efficient - the hierarchy itself serves as the search index
- Thread-safe when wrapped appropriately

## Quick Start

```rust
use janus_array::{File, DiskOffsets};

// Create hierarchical structure
let mut file = File::with_size(1024);
// ... populate with slices, commands, elements

// Direct access - O(1)
let element = &file.slices[0].commands[2].elements[5];

// Reverse lookup - O(log n)
match file.find_address(0x47382) {
    Ok(coords) => {
        println!("Found at slice: {:?}, command: {:?}, element: {:?}", 
                 coords.slice, coords.command, coords.element);
    }
    Err(e) => println!("Address not found: {:?}", e),
}
```

## Performance Characteristics

For a structure with S slices, C commands, and E elements:

- **Direct access**: O(1)
- **Reverse lookup**: O(log S + log C + log E)

In practice, since S is typically small (≤ 3 for most binary formats), this reduces to **O(log C + log E)**.

When many commands are leaf nodes (no elements), the average case approaches **O(log C)**.

## Architecture

The library uses a recursive trait-based design:

```rust
pub trait DiskOffsets {
    fn find_address(&mut self, addr: u64) -> Result<Coordinates, OffsetLayoutsError>;
    fn get_absolute_range(&self) -> Range<u64>;
    // ... other methods
}
```

Each level (File, Slice, Command, Element) implements this trait, enabling natural recursive search through the hierarchy.

## Use Cases

- **Binary file parsers**: Mach-O, ELF, PE format analysis
- **Filesystem tools**: Block-level to inode navigation  
- **Network protocol analysis**: Nested packet structures
- **Debugging tools**: Address-to-symbol resolution
- **Reverse engineering**: Mapping offsets to logical structures

## Example: Binary Format Parser

```rust
use janus_array::*;

// Parse a binary file with nested load commands
let mut file = File::with_size(file_size);

for (i, slice_data) in slice_iterator.enumerate() {
    let mut slice = Slices::default();
    slice.populate_values(slice_data.start, slice_data.end, 0, slice_data.size, i);
    
    for (j, cmd_data) in slice_data.commands.enumerate() {
        let mut command = Commands::default();
        command.populate_values(cmd_data.start, cmd_data.end, 
                               cmd_data.start - slice_data.start, cmd_data.size, j);
        slice.add_command(command);
    }
    
    file.add_slice(slice);
}

// Now you can efficiently navigate both ways
let coords = file.find_address(mystery_offset)?;
```

## Installation

Currently available as a Git dependency. Add to your `Cargo.toml`:

```toml
[dependencies]
janus-array = { git = "https://github.com/gb-at-r3/janus-array" }
```

*Will be published to crates.io as version 0.1 once stabilized.*

## Requirements

- Rust 1.70+
- No external dependencies for core functionality

## Error Handling

The library provides comprehensive error types:

```rust
pub enum OffsetLayoutsError {
    AddressOutsideCurrentScope(u64, Range<u64>),
    InconsistentStructure(u64, Range<u64>),
    InconsistentSearch,
    NotFound(u64),
    SliceIsBroken,
    CommandIsBroken,
}
```

## Contributing

Contributions welcome. Areas of interest:

- Performance optimizations for specific access patterns
- Additional hierarchy levels or structure types
- Async-friendly variants
- Serialization support

## License

MIT License. See LICENSE file for details.

## Background

Developed for reverse engineering binary file formats where both hierarchical navigation and offset-based queries are performance-critical. The design prioritizes real-world usage patterns over theoretical worst-case scenarios.

## Alternatives

Consider these alternatives for different use cases:

- **HashMap<offset, path>**: Better for sparse, non-hierarchical data
- **Interval trees**: Better for overlapping ranges
- **B-trees**: Better for disk-based storage with many updates
- **Simple arrays**: Better when you only need one access pattern

The Janus Array shines when you need both access patterns on hierarchical data with predictable structure.

## Status

**Work in Progress** - API may change before 1.0 release.
