use super::node::Node;

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Array<'a> {
	item_type: &'a str,
	values: Vec<Node<'a>>,
}
