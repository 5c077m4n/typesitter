use super::node::Node;

#[derive(Debug)]
pub struct Object<'o> {
	keys: Box<Vec<&'o str>>,
	values: Box<Vec<&'o Node<'o>>>,
}
