#[cfg(test)]
mod tests {
	use macros::test_with_logger;

	#[allow(clippy::assertions_on_constants)]
	#[test_with_logger]
	pub fn sanity() {
		log::info!("success");
		assert!(true);
	}
}
