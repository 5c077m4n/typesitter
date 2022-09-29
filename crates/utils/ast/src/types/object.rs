use std::collections::BTreeMap;

use super::node::Node;

pub type Object<'o> = BTreeMap<&'o str, Node<'o>>;
