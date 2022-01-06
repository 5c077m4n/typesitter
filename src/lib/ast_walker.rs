use anyhow::Result;
use log::*;
use tree_sitter::TreeCursor;

pub fn walk(cursor: &mut TreeCursor) -> Result<()> {
    let node = cursor.node();
    debug!("{:#?}", &node);

    while cursor.goto_first_child() {
        walk(cursor)?;
    }
	while cursor.goto_next_sibling() {
        walk(cursor)?;
	}

	Ok(())
}
