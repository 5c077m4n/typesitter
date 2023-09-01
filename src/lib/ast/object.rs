use super::node::Node;

#[derive(Debug)]
pub struct Object<'o> {
	keys: &'o [&'o str],
	values: &'o [&'o Node<'o>],
}
