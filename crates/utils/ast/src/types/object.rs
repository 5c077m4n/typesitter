use super::node::Node;
use std::collections::BTreeMap;

pub type Object<'o> = BTreeMap<&'o str, Node<'o>>;
