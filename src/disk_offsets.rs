use std::ops::Range;

use crate::{coordinates::Coordinates, types::ChildrenArray};

pub trait DiskOffsets{

    // Functions to implement:
    fn set_start_abs_address(&mut self, start_abs:u64);
    fn set_start_rel_address(&mut self, start_rel:u64);
    fn set_end_abs_address(&mut self, end_abs:u64);
    fn set_end_rel_address(&mut self, end_rel:u64);
    fn set_absolute_range_explicit(&mut self, range:Range<u64>);
    fn set_relative_range_explicit(&mut self, range:Range<u64>);
    fn set_absolute_range_implicit(&mut self);
    fn set_relative_range_implicit(&mut self);
    fn set_ordinal(&mut self, ordinal:usize);
    fn get_absolute_range(&self)->Range<u64>;
    fn get_relative_range(&self)->Range<u64>;
    fn has_children(&self)->bool;
    fn get_children(&mut self)->Option<ChildrenArray>;
    fn sort_children(&mut self);
    fn get_max_abs_address(&self)->u64;
    fn get_min_abs_address(&self)->u64;
    fn find_address(&mut self, absolute_address:u64) -> Result<Coordinates, OffsetLayoutsError>;
    //

    fn populate_values(&mut self, start_abs:u64, end_abs:u64, start_rel:u64, end_rel:u64, ordinal:usize){
        self.set_absolutes(start_abs, end_abs); 
        self.set_relatives(start_rel, end_rel);
        self.set_ordinal(ordinal);
    }

    fn set_absolutes(&mut self, start_abs:u64, end_abs:u64){
        self.set_start_abs_address(start_abs);
        self.set_end_abs_address(end_abs);

        self.set_absolute_range_implicit();
    }

    fn set_relatives(&mut self,start_rel:u64, end_rel:u64){
        self.set_start_rel_address(start_rel);
        self.set_end_rel_address(end_rel);

        self.set_relative_range_implicit();
    }

    fn contains_absolute_address(&self,an_address:u64)->bool{
        let range=self.get_absolute_range();
        range.contains(&an_address)
    }

    

}


#[derive(Clone,Debug)]
pub enum OffsetLayoutsError{
    AddressOutsideCurrentScope(u64,Range<u64>),
    InconsistentStructure(u64,Range<u64>),
    InconsistentSearch,
    NotFound(u64),
    SliceIsBroken,
    CommandIsBroken,
}






