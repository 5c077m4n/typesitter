use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Array<'a> {
	values: &'a [&'a Node<'a>],
	item_type: &'a str,
}
