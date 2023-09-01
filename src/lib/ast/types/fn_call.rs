use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct FnCall<'f> {
	pub fn_name: &'f str,
	pub params: &'f [&'f Node<'f>],
}
