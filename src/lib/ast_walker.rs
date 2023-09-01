use log::*;
use tree_sitter::TreeCursor;

pub fn walk(cursor: &TreeCursor) {
    debug!("{:?}", &cursor.node().to_sexp());
}
