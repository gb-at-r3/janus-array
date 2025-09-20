use std::ops::Range;

use crate::slices::Slices;
use crate::types::ChildrenArray;
use crate::disk_offsets::{DiskOffsets, OffsetLayoutsError};
use crate::coordinates::Coordinates;

#[derive(Clone, Debug)]
pub struct File {
    pub start_abs_address: u64,
    pub end_abs_address: u64,
    pub start_rel_address: u64, // Always 0 for File (root level)
    pub end_rel_address: u64,   // Same as end_abs_address

    pub absolute_range: Range<u64>,
    pub relative_range: Range<u64>, // Same as absolute_range for File

    pub my_ordinal: usize, // Always 0 for File (only one file per structure)

    pub slices: Option<Vec<Slices>>,
}

impl Default for File {
    fn default() -> Self {
        let empty_range: Range<u64> = 0..0;
        Self {
            start_abs_address: 0,
            end_abs_address: 0,
            start_rel_address: 0,
            end_rel_address: 0,
            absolute_range: empty_range.clone(),
            relative_range: empty_range,
            my_ordinal: 0,
            slices: None,
        }
    }
}

impl File {
    pub fn new() -> Self {
        let empty_range: Range<u64> = 0..0;
        Self {
            start_abs_address: 0,
            end_abs_address: 0,
            start_rel_address: 0,
            end_rel_address: 0,
            absolute_range: empty_range.clone(),
            relative_range: empty_range,
            my_ordinal: 0,
            slices: None,
        }
    }

    /// Create a new File with specified size
    pub fn with_size(file_size: u64) -> Self {
        let range = 0..file_size;
        Self {
            start_abs_address: 0,
            end_abs_address: file_size,
            start_rel_address: 0,
            end_rel_address: file_size,
            absolute_range: range.clone(),
            relative_range: range,
            my_ordinal: 0,
            slices: None,
        }
    }

    /// Add a slice to the file
    pub fn add_slice(&mut self, slice: Slices) {
        match &mut self.slices {
            Some(slices) => slices.push(slice),
            None => self.slices = Some(vec![slice]),
        }
    }
}

impl DiskOffsets for File {
    fn set_start_abs_address(&mut self, start_abs: u64) {
        self.start_abs_address = start_abs;
    }

    fn set_start_rel_address(&mut self, start_rel: u64) {
        self.start_rel_address = start_rel;
    }

    fn set_end_abs_address(&mut self, end_abs: u64) {
        self.end_abs_address = end_abs;
    }

    fn set_end_rel_address(&mut self, end_rel: u64) {
        self.end_rel_address = end_rel;
    }

    fn set_absolute_range_explicit(&mut self, range: Range<u64>) {
        self.absolute_range = range;
    }

    fn set_absolute_range_implicit(&mut self) {
        self.absolute_range = self.start_abs_address..self.end_abs_address;
    }

    fn set_relative_range_explicit(&mut self, range: Range<u64>) {
        self.relative_range = range;
    }

    fn set_relative_range_implicit(&mut self) {
        // For File, relative range is same as absolute range (it's the root)
        self.relative_range = self.start_abs_address..self.end_abs_address;
    }

    fn set_ordinal(&mut self, ordinal: usize) {
        self.my_ordinal = ordinal;
    }

    fn get_absolute_range(&self) -> Range<u64> {
        self.absolute_range.clone()
    }

    fn get_relative_range(&self) -> Range<u64> {
        self.relative_range.clone()
    }

    fn get_max_abs_address(&self) -> u64 {
        self.end_abs_address
    }

    fn get_min_abs_address(&self) -> u64 {
        self.start_abs_address
    }

    fn has_children(&self) -> bool {
        match &self.slices {
            Some(slices) => !slices.is_empty(),
            None => false,
        }
    }

    fn get_children(&mut self) -> Option<ChildrenArray> {
        if self.has_children() {
            self.sort_children();
            let children = self.slices.clone()?;
            Some(ChildrenArray::File(children))
        } else {
            None
        }
    }

    fn sort_children(&mut self) {
        if let Some(children) = &mut self.slices {
            children.sort_by_key(|s| s.start_abs_address);
        }
    }

    fn find_address(&mut self, absolute_address: u64) -> Result<Coordinates, OffsetLayoutsError> {
        let absolute_range = self.get_absolute_range();
        
        if !absolute_range.contains(&absolute_address) {
            return Err(OffsetLayoutsError::AddressOutsideCurrentScope(
                absolute_address,
                absolute_range,
            ));
        }

        let mut retval = Coordinates::new();
        // File level doesn't set any coordinate (it's the root)

        if self.has_children() {
            let mut children = if let Some(ChildrenArray::File(v)) = self.get_children() {
                v
            } else {
                return Err(OffsetLayoutsError::InconsistentSearch);
            };

            let mut start: usize = 0;
            let mut end: usize = children.len() - 1;

            while start <= end {
                let mid = (start + end) / 2;
                let range = children[mid].get_absolute_range();
                
                if range.contains(&absolute_address) {
                    // Found the slice! Delegate the search recursively
                    match children[mid].find_address(absolute_address) {
                        Ok(coords) => {
                            // Merge coordinates from child search
                            if let Some(s) = coords.slice {
                                retval.set_slice(s);
                            } else {
                                return Err(OffsetLayoutsError::SliceIsBroken);
                            }
                            if let Some(c) = coords.command {
                                retval.set_command(c);
                            }
                            if let Some(e) = coords.element {
                                retval.set_element(e);
                            }
                        }
                        Err(_) => return Err(OffsetLayoutsError::SliceIsBroken),
                    }
                    return Ok(retval);
                } else {
                    let min = children[mid].get_min_abs_address();
                    let max = children[mid].get_max_abs_address();
                    
                    if absolute_address < min {
                        if mid == 0 {
                            break;
                        }
                        end = mid - 1;
                    } else if absolute_address > max {
                        start = mid + 1;
                    } else {
                        return Err(OffsetLayoutsError::InconsistentStructure(
                            absolute_address,
                            range,
                        ));
                    }
                }
            }
            
            // If we get here, address wasn't found in any slice
            Err(OffsetLayoutsError::NotFound(absolute_address))
        } else {
            // File has no slices - this shouldn't happen in practice
            Err(OffsetLayoutsError::InconsistentStructure(
                absolute_address,
                absolute_range,
            ))
        }
    }
}