use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Object<'o> {
	keys: Vec<&'o str>,
	values: Vec<Node<'o>>,
}
