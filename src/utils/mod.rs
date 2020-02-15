pub fn is_keyword(word: &str) -> bool {
	return word == "if"
		  || word == "while"
		  || word == "for"
		  || word == "final"
		  || word == "class";
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_keyword_test() {
		assert!(is_keyword("if"));
		assert!(is_keyword("while"));
		assert!(is_keyword("for"));
		assert!(is_keyword("final"));
		assert!(is_keyword("class"));
	}

	#[test]
	fn not_keyword_test() {
		assert!(!is_keyword("function"));
		assert!(!is_keyword("def"));
		assert!(!is_keyword("list"));
		assert!(!is_keyword("set"));
		assert!(!is_keyword("i f"));
	}
}