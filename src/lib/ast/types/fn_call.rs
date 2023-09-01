use super::node::Node;

#[derive(Debug)]
pub struct FnCall<'f> {
	fn_name: &'f str,
	params: &'f [&'f Node<'f>],
}
