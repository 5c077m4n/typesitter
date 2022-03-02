use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Array<'a> {
	item_type: &'a str,
	values: Box<[Node<'a>]>,
}
