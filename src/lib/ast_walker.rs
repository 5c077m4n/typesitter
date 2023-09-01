use anyhow::Result;
use log::*;
use tree_sitter::TreeCursor;

pub fn walk(cursor: &mut TreeCursor, split_code: &[&str]) -> Result<()> {
	let node = cursor.node();
	debug!(
		"{:#?} => {:?}",
		&node,
		split_code
			.get(node.start_position().row)
			.unwrap()
			.get(node.start_position().column..node.end_position().column)
	);

	while cursor.goto_first_child() {
		walk(cursor, split_code)?;
	}
	while cursor.goto_next_sibling() {
		walk(cursor, split_code)?;
	}

	Ok(())
}
