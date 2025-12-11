use hashbrown::HashMap;

pub type Node = usize;
pub type Label = String;

pub type Node2LabelMapping = HashMap<Node, Label>;
pub type Label2NodeMapping = HashMap<Label, Node>;
