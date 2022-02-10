use super::node::Node;

#[derive(Debug)]
pub struct If<'i> {
	expr: &'i Node<'i>,
	block: &'i Node<'i>,
	else_block: Option<&'i Node<'i>>,
}
