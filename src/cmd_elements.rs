use std::ops::Range;

use crate::coordinates::Coordinates;
use crate::types::ChildrenArray;
use crate::disk_offsets::DiskOffsets;
use crate::disk_offsets::OffsetLayoutsError;

use crate::cmd_elements;

// Assumptions and Conventions:
// we adopt natural Rust's indexing method (first object index is 0)

#[derive(Clone,Debug)]
pub struct CommandElements{
    pub start_abs_address: u64,
    pub end_abs_address: u64,
    pub start_rel_address: u64,
    pub end_rel_address: u64,

    pub absolute_range:Range<u64>,
    pub relative_range:Range<u64>,

    pub my_ordinal:usize,
}

impl Default for CommandElements{
    fn default () -> Self {
        let empty_range: Range<u64> = 0..0;
        Self { 
            start_abs_address: 0, 
            end_abs_address: 0, 
            start_rel_address: 0, 
            end_rel_address: 0, 
            absolute_range: empty_range.clone(), 
            relative_range: empty_range, 
            my_ordinal: 0 }
    }
}

impl CommandElements {
    fn new () -> Self {
        let empty_range: Range<u64> = 0..0;
        Self { 
            start_abs_address: 0, 
            end_abs_address: 0, 
            start_rel_address: 0, 
            end_rel_address: 0, 
            absolute_range: empty_range.clone(), 
            relative_range: empty_range, 
            my_ordinal: 0 
        }
    }

    fn populate_values(&mut self, start_abs:u64, end_abs:u64, start_rel:u64, end_rel:u64, ordinal:usize){
        self.set_absolutes(start_abs, end_abs); 
        self.set_relatives(start_rel, end_rel);
        self.set_ordinal(ordinal);
    }

    fn set_absolutes(&mut self, start_abs:u64, end_abs:u64){
        self.start_abs_address  =   start_abs; 
        self.end_abs_address    =   end_abs; 

        self.absolute_range = self.start_abs_address..self.end_abs_address;
    }

    fn set_relatives(&mut self, start_rel:u64, end_rel:u64){
        self.start_rel_address  =   start_rel; 
        self.end_rel_address    =   end_rel;

        self.relative_range = self.start_rel_address..self.end_rel_address;
    }

    fn set_ordinal(&mut self, ordinal:usize){
        self.my_ordinal=ordinal;
    }

    fn contains_address(self,an_address:u64)->bool{
        self.absolute_range.contains(&an_address)
    }

}



impl DiskOffsets for CommandElements {
    fn set_start_abs_address(&mut self, start_abs:u64){
        self.start_abs_address = start_abs; 
    }

    fn set_start_rel_address(&mut self, start_rel:u64){
        self.start_rel_address = start_rel; 
    }

    fn set_end_abs_address(&mut self, end_abs:u64){
        self.end_abs_address = end_abs; 
    }

    fn set_end_rel_address(&mut self, end_rel:u64){
        self.end_rel_address = end_rel; 
    }

    fn set_absolute_range_explicit(&mut self, range:Range<u64>){
        self.absolute_range=range;
    }

    fn set_absolute_range_implicit(&mut self){
        self.absolute_range = self.start_abs_address..self.end_abs_address;
    }

    fn set_relative_range_explicit(&mut self, range:Range<u64>){
        self.relative_range=range;
    }

    fn set_relative_range_implicit(&mut self){
        self.relative_range = self.start_rel_address..self.end_rel_address;
    }

    fn set_ordinal(&mut self, ordinal:usize){
        self.my_ordinal=ordinal;
    }

    fn get_absolute_range(&self)->Range<u64> {
        self.absolute_range.clone()
    }

    fn get_relative_range(&self)->Range<u64> {
        self.relative_range.clone()
    }

    fn get_max_abs_address(&self)->u64 {
        self.end_abs_address
    }

    fn get_min_abs_address(&self)->u64 {
        self.start_abs_address
    }

    fn has_children(&self)->bool{
        false
    }

    fn get_children(&mut self)->Option<ChildrenArray> {
        None
    }

    fn sort_children(&mut self){
        // by definition, an element has no children.
    }

    fn find_address(&mut self, absolute_address:u64) -> Result<Coordinates, OffsetLayoutsError>{
        let absolute_range = self.get_absolute_range();
        if absolute_range.contains(&absolute_address){
            let mut retval = Coordinates::new();
            retval.set_element(self.my_ordinal);
            return Ok(retval);
        } else {
            Err(OffsetLayoutsError::AddressOutsideCurrentScope(absolute_address,absolute_range))
        }
    }

}
