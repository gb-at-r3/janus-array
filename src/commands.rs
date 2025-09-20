use std::ops::Range;
use crate::disk_offsets::{DiskOffsets, OffsetLayoutsError};
use crate::cmd_elements::CommandElements;
use crate::types::ChildrenArray;
use crate::coordinates::Coordinates;

#[derive(Clone,Debug)]
pub struct Commands{
    pub start_abs_address: u64,
    pub end_abs_address: u64,
    pub start_rel_address: u64,
    pub end_rel_address: u64,

    pub absolute_range:Range<u64>,
    pub relative_range:Range<u64>,

    pub my_ordinal:usize,

    pub elements:Option<Vec<CommandElements>>
}

impl Default for Commands{
    fn default () -> Self {
        let empty_range: Range<u64> = 0..0;
        Self { 
            start_abs_address: 0, 
            end_abs_address: 0, 
            start_rel_address: 0, 
            end_rel_address: 0, 
            absolute_range: empty_range.clone(), 
            relative_range: empty_range, 
            my_ordinal: 0,
            elements: None,
        }
    }
}

impl Commands{
    fn new() -> Self {
        let empty_range: Range<u64> = 0..0;
        Self { 
            start_abs_address: 0, 
            end_abs_address: 0, 
            start_rel_address: 0, 
            end_rel_address: 0, 
            absolute_range: empty_range.clone(), 
            relative_range: empty_range, 
            my_ordinal: 0,
            elements: None,
        }
    }


}

impl DiskOffsets for Commands {
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
        match &self.elements {
            Some(cmd_elements) => cmd_elements.len()>0,
            None =>false
        }
    }

    fn get_children(&mut self)->Option<ChildrenArray> {
        
        if self.has_children() {
            self.sort_children();
            let children = self.elements.clone()?;
            Some(ChildrenArray::Commands(children))
        } else {
            None
        }
        
    }

    fn sort_children(&mut self) {
        if let Some(children) = &mut self.elements {
            children.sort_by_key(|c| c.start_abs_address);
        }
    }

    fn find_address(&mut self, absolute_address:u64) -> Result<Coordinates, OffsetLayoutsError>{
        let absolute_range = self.get_absolute_range();
        if absolute_range.contains(&absolute_address){
            let mut retval = Coordinates::new();
            retval.set_command(self.my_ordinal);
            if self.has_children(){
                let mut children = if let Some(ChildrenArray::Commands(v)) = self.get_children() {
                    v
                } else {
                    return Err(OffsetLayoutsError::InconsistentSearch);
                };

                let mut start:usize = 0;
                let mut end: usize = children.len() - 1 ;

                while start <= end {
                    let mid = (start + end) / 2;
                    let range = &children[mid].get_absolute_range();
                    if range.contains(&absolute_address) {
                        match children[mid].find_address(absolute_address) {
                            Ok(coords) => {
                                if let Some(e) = coords.element {
                                    retval.set_element(e);
                                }
                            }
                            Err(_) => return Err(OffsetLayoutsError::CommandIsBroken),
                        }
                        return Ok(retval);
                    } else {
                        let min = &children[mid].get_min_abs_address();
                        let max = &children[mid].get_max_abs_address();
                        let mid_range = &children[mid].get_absolute_range();
                        if absolute_address < *min {
                            if mid == 0 {
                                break;
                            }
                            end = mid - 1;
                        } else if absolute_address > *max {
                            start = mid + 1;
                        } else {
                            return Err(OffsetLayoutsError::InconsistentStructure(absolute_address,mid_range.clone()));
                        }
                    }
                }
            }
            Ok(retval)
        } else {
            Err(OffsetLayoutsError::AddressOutsideCurrentScope(absolute_address,absolute_range))
        }
    }
    
}
