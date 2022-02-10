use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Object<'o> {
	keys: Box<Vec<&'o str>>,
	values: Box<Vec<&'o Node<'o>>>,
}
