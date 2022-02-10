use super::node::Node;

#[derive(Debug)]
pub struct Array<'a> {
	values: &'a [&'a Node<'a>],
	item_type: &'a str,
}
