use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct TextPart {
	pub start : usize,
	pub end : usize,
	pub text : String,
}

/// Element is the basic unit of parsing. The most simple Element is implemented
/// directly by regular expression. More complex Element can be implemented by
/// composing simpler Element.
pub trait Element {
	type Detail;
	/// Given a text and a position, find out if it satisfies the pattern
	/// of this element. If it satisfies, parse it into Detail.
	fn find_at(&self, text: &str, start: usize) -> Option<(TextPart, Self::Detail)>;
}

#[derive(Debug)]
pub struct BasicElement {
	regex : Regex
}

impl BasicElement {
	pub fn new(regex : Regex) -> BasicElement {
		BasicElement {
			regex
		}
	}
}

impl Element for BasicElement {
	type Detail = TextPart;

	fn find_at(&self, text: &str, start: usize) -> Option<(TextPart, TextPart)> {
		if start >= text.len() {
			return None
		}
		if let Some(mat) = self.regex.find(&text[start..]) {
			let group = self.regex.captures(mat.as_str()).unwrap().get(1).unwrap();
			Some((TextPart {
				start: start + mat.start(),
				end: start + mat.end(),
				text: mat.as_str().to_string()
			}, TextPart {
				start: start + mat.start() + group.start(),
				end: start + mat.start() + group.end(),
				text: group.as_str().to_string()
			}))
		} else {
			None
		}
	}
}
