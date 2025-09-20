use crate::cmd_elements::CommandElements;
use crate::slices::Slices;
use crate::commands::Commands;

#[derive(Debug, Clone)]
pub enum ChildrenArray {
    File(Vec<Slices>),
    Slice(Vec<Commands>),
    Commands(Vec<CommandElements>),
}
