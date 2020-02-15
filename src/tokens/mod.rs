use crate::eregex::BasicElement;
use crate::eregex::Element;
use crate::eregex::TextPart;
use regex::Regex;

pub struct Word {
	element: BasicElement
}

impl Word {
	fn new() -> Word {
		let re = Regex::new(r"^\s*\b(\w+)\b").unwrap();
		Word { element: BasicElement::new(re) }
	}
}

impl Element for Word {
	type Detail = TextPart;
	fn find_at(&self, text: &str, start: usize) -> Option<(TextPart, TextPart)> {
		self.element.find_at(text, start)
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn find_word() {
		assert_eq!(
			Word::new().find_at("  hello world ", 0),
			Some((TextPart {
				start: 0,
				end: 7,
				text: String::from("  hello")
			}, (TextPart {
				start: 2,
				end: 7,
				text: String::from("hello")
			})))
		);

		assert_eq!(
			Word::new().find_at("  hello world ", 2),
			Some((TextPart {
				start: 2,
				end: 7,
				text: String::from("hello")
			}, (TextPart {
				start: 2,
				end: 7,
				text: String::from("hello")
			}))));

		assert_eq!(
			Word::new().find_at("  hello world ", 7),
			Some((TextPart {
				start: 7,
				end: 13,
				text: String::from(" world")
			}, (TextPart {
				start: 8,
				end: 13,
				text: String::from("world")
			}))));
	}
}