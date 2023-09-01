use super::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Array<'a> {
	item_type: &'a str,
	values: Box<[Node<'a>]>,
}
