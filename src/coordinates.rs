#[derive(Debug,Clone)]
pub struct Coordinates{
    pub slice:Option<usize>,
    pub command:Option<usize>,
    pub element:Option<usize>,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self { slice: None, command: None, element: None }
    }
}

impl Coordinates {
    pub fn new() -> Self {
        Self { slice: None, command: None, element: None }
    }

    pub fn set_slice(&mut self, a_slice:usize){
        self.slice=Some(a_slice)
    }

    pub fn set_command(&mut self, a_command:usize){
        self.command=Some(a_command)
    }

    pub fn set_element(&mut self, an_element:usize){
        self.element=Some(an_element)
    }

}