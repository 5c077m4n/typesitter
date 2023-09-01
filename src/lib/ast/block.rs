use super::statement::Statement;

pub struct Block<'b> {
	statement_list: &'b [&'b Statement<'b>],
}
