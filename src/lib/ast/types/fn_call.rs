use super::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct FnCall<'f> {
	fn_name: &'f str,
	params: &'f [&'f Node<'f>],
}
