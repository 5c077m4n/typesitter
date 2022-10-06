use super::{node::Node, type_annotation::TypeAnnotation, var_decl::VarDecl};

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FnType {
	Arrow,
	Classic,
}

#[cfg_attr(feature = "js_bind", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FnDecl<'f> {
	pub fn_type: FnType,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub name: Option<Vec<&'f [u8]>>,
	pub input_params: Vec<VarDecl<'f>>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub return_type: Option<TypeAnnotation<'f>>,
	#[cfg_attr(feature = "js_bind", serde(borrow))]
	pub body: Box<Node<'f>>,
}

impl FnDecl<'_> {
	pub fn get_name(&self) -> Option<String> {
		self.name.as_ref().map(|name_ls| {
			name_ls
				.iter()
				.map(|&name| std::str::from_utf8(name).unwrap())
				.collect::<Vec<_>>()
				.join(".")
		})
	}
}
