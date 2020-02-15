use regex::Regex;

use super::param_item;

pub fn transpile_params(input: &str) -> String {
	let (positional, named) = split_parameter_list(input);

	let positional = positional.iter().map(|item| {
		param_item::create_function_action(item)
	}).collect::<Vec<String>>().join(",\n");

	let named = named.iter().map(|item| {
		param_item::append_default_value(
			&param_item::create_function_action(item))
	}).collect::<Vec<String>>().join(",\n");

	if positional.is_empty() || named.is_empty() {
		format!("{}{}", positional, named)
	} else {
		format!("{},\n{}", positional, named)
	}
}

/// Given a parameter list, split it into two strings,
/// the first represents the positional parameters
/// and the second consists of all named parameters
/// Either/both of them may be empty. To handle the empty
/// case, the return type is Option instead of &str
fn split_positioned_named(input: &str) -> (Option<&str>, Option<&str>) {
	lazy_static! {
		static ref RE : Regex = Regex::new(r"(?x)
			^\s*(?P<positional>[^{}]*)? # Positional part
			\s*
			(?:\{\s*(?P<named>(?:.*\n)*(?:.*)?)\s*\}\s*)? # Named part
			$").unwrap();
	}

	let mut positional = Option::None;
	let mut named = Option::None;

	for cap in RE.captures_iter(input) {
		match cap.name("positional") {
			Some(expr) => {
				let expr = expr.as_str().trim();
				positional = if expr.is_empty() {
					None
				} else {
					Option::Some(expr)
				}
			},
			None => {},
		};
		match cap.name("named") {
			Some(expr) => {
				let expr = expr.as_str().trim();
				named = if expr.is_empty() {
					None
				} else {
					Option::Some(expr)
				}
			},
			None => {},
		};
	}

	(positional, named)
}

/// Split the parameter list of a function into two vectors
/// i.e. a vector containing only the positional parameters,
/// and another vector for the named parameters.
/// It is assumed that the positional parameter part does not
/// contain any curly braces `{}`.
fn split_parameter_list(input: &str) -> (Vec<String>, Vec<String>) {
	let (positional, named) = split_positioned_named(input);
	let positional = match positional {
		Some(expr) => split_single_parameter_list(expr),
		None => vec![],
	};
	let named = match named {
		Some(expr) => split_single_parameter_list(expr),
		None => vec![],
	};

	(positional, named)
}

/// Split a single parameter list into a vector of items.
/// Each item may be in one of the following forms:
///
/// - this.param
/// - TypeName param
/// - this.param = value
/// - TypeName param = value
/// - TypeName<T> param
/// - TypeName<T> param = value
/// - TypeName<T,K> param
/// - TypeName<T,K> param = value
/// - TypeName param()
/// - TypeName param(TypeName a)
/// - TypeName param(TypeName a, TypeName b)
///
/// where the value may take various kinds of forms.
/// For simplicity, we only handle cases where value does
/// not contain comma.
/// The items are originally connected with comma (`,`),
/// and arbitrary number of spaces including `\n`.
/// The last item may end with or without comma.
///
/// Assume the function pointer parameter contains only simple
/// type names.
fn split_single_parameter_list(input: &str) -> Vec<String> {

	// Make sure input is ended with comma, so we don't have
	// to handle the tricky last item in our regex
	let input = if input.trim().ends_with(",") {
		String::from(input.trim())
	} else {
		format!("{},", input.trim())
	};

	lazy_static! {
		static ref RE : Regex = Regex::new(
			r"(?x)
				(?:\s*
					(?P<item>
						(?: # The name of the type, or just `this.`
							this\.
							|
							\w+(?: # TypeName
								<\s*(\w+\s*,)*\w*\w+\s*> # Optional template parameters
							)?\s+ # TypeName must be splitted with parameter name
						)\w+ # End of type name or `this.`
						(?:\s*=[^,]*)? # Optional default value
						|
						\w+\s+\w+\s*\((\w+\s+\w+\s*,)*\s*(\w+\s+\w+\s*)?\)
					)\s*,
				)\s*"
		).unwrap();
	}

	let mut result : Vec<String> = vec![];

	for cap in RE.captures_iter(&input) {
		match cap.name("item") {
			Some(expr) => result.push(expr.as_str().to_string()),
			None => {},
		};
	}

	result
}


#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn single_parameter_list_splitted_1() {
		assert_eq!(
			split_single_parameter_list(
				r"
  NodeMetadata meta,
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
			"),
			(
				vec![
					"NodeMetadata meta",
					"BuildOp buildOp",
					"Iterable<String> stylesPrepend"]
			)
		);
	}

	#[test]
	fn single_parameter_list_splitted_2() {
		assert_eq!(
			split_single_parameter_list(
				r"
  void f(),
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
			"),
			(
				vec![
					"void f()",
					"BuildOp buildOp",
					"Iterable<String> stylesPrepend"]
			)
		);
	}

	#[test]
	fn single_parameter_list_splitted_3() {
		assert_eq!(
			split_single_parameter_list(
				r"
  void f(int a, float b),
  BuildOp buildOp,
  Iterable<String> stylesPrepend
			"),
			(
				vec![
					"void f(int a, float b)",
					"BuildOp buildOp",
					"Iterable<String> stylesPrepend"]
			)
		);
	}

	#[test]
	fn single_parameter_list_splitted_4() {
		assert_eq!(
			split_single_parameter_list(
				r"
  void f(int a, float b),
  BuildOp buildOp,
  this.styles,
  Iterable<String> stylesPrepend
			"),
			(
				vec![
					"void f(int a, float b)",
					"BuildOp buildOp",
					"this.styles",
					"Iterable<String> stylesPrepend"]
			)
		);
	}

	#[test]
	fn single_parameter_list_splitted_5() {
		assert_eq!(
			split_single_parameter_list(
				r"
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
  void f(int a, float b),
  this.styles
			"),
			(
				vec![
					"BuildOp buildOp",
					"Iterable<String> stylesPrepend",
					"void f(int a, float b)",
					"this.styles"]
			)
		);
	}


	#[test]
	fn parameter_list_splitted_1() {
		assert_eq!(
			split_parameter_list(
				r"
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
}
			"),
			(
				vec![String::from("NodeMetadata meta")],
				vec![String::from("BuildOp buildOp"), String::from("Iterable<String> stylesPrepend")]
			)
		);
	}


	#[test]
	fn positional_named_splitted_1() {
		assert_eq!(
			split_positioned_named(
				r"
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
}
			"),
			(Option::Some("NodeMetadata meta,"), Option::Some("BuildOp buildOp,
  Iterable<String> stylesPrepend,"))
		);
	}


	#[test]
	fn params_transpiled_1() {
		assert_eq!(
			transpile_params(
				r"
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
}
			"),
			"NodeMetadata meta,
BuildOp buildOp = null,
Iterable<String> stylesPrepend = null"
		);
	}

	#[test]
	fn parameter_list_splitted_2() {
		assert_eq!(
			split_parameter_list(
				r"{
    this.block,
    this.widgets
  }
			"),
			(
				vec![],
				vec![String::from("this.block"), String::from("this.widgets")]
			)
		);
	}

	#[test]
	fn positional_named_splitted_2() {
		assert_eq!(
			split_positioned_named(
				r"{
    this.block,
    this.widgets
  }
			"),
			(Option::None, Option::Some("this.block,
    this.widgets"))
		);
	}

	#[test]
	fn params_transpiled_2() {
		assert_eq!(
			transpile_params(
				r"{
    this.block,
    this.widgets
  }
			"),
			"this.block = null,
this.widgets = null"
		);
	}

	#[test]
	fn parameter_list_splitted_3() {
		assert_eq!(
			split_parameter_list(
				r"{
    CssLength bottom,
    CssLength left,
    CssLength right,
    CssLength top,
  }
			"),
			(
				vec![],
				vec![
					String::from("CssLength bottom"),
					String::from("CssLength left"),
					String::from("CssLength right"),
					String::from("CssLength top")]
			)
		);
	}

	#[test]
	fn positional_named_splitted_3() {
		assert_eq!(
			split_positioned_named(
				r"{
    CssLength bottom,
    CssLength left,
    CssLength right,
    CssLength top,
  }
			"),
			(Option::None, Option::Some("CssLength bottom,
    CssLength left,
    CssLength right,
    CssLength top,"))
		);
	}

	#[test]
	fn params_transpiled_3() {
		assert_eq!(
			transpile_params(
				r"{
    CssLength bottom,
    CssLength left,
    CssLength right,
    CssLength top,
  }
			"),
			"CssLength bottom = null,
CssLength left = null,
CssLength right = null,
CssLength top = null"
		);
	}

	#[test]
	fn parameter_list_splitted_4() {
		assert_eq!(
			split_parameter_list(
				r"this.block, this.data, this.tsb, {this.onTap}"),
			(
				vec![
					String::from("this.block"),
					String::from("this.data"),
					String::from("this.tsb")],
				vec![String::from("this.onTap")]
			)
		);
	}

	#[test]
	fn positional_named_splitted_4() {
		assert_eq!(
			split_positioned_named(
				r"this.block, this.data, this.tsb, {this.onTap}"),
			(Option::Some("this.block, this.data, this.tsb,"), Option::Some("this.onTap"))
		);
	}

	#[test]
	fn params_transpiled_4() {
		assert_eq!(
			transpile_params(
				r"this.block, this.data, this.tsb, {this.onTap}"),
			"this.block,
this.data,
this.tsb,
this.onTap = null"
		);
	}

	#[test]
	fn parameter_list_splitted_5() {
		assert_eq!(
			split_parameter_list(
				r"void f(String key, String value)"),
			(
				vec![String::from("void f(String key, String value)")],
				vec![]
			)
		);
	}

	#[test]
	fn positional_named_splitted_5() {
		assert_eq!(
			split_positioned_named(
				r"void f(String key, String value)"),
			(Option::Some("void f(String key, String value)"), Option::None)
		);
	}

	#[test]
	fn params_transpiled_5() {
		assert_eq!(
			transpile_params(
				r"void f(String key, String value)"),
			"Action<String, String> f"
		);
	}

	#[test]
	fn parameter_list_splitted_6() {
		assert_eq!(
			split_parameter_list(
				r""),
			(
				vec![],
				vec![]
			)
		);
	}

	#[test]
	fn positional_named_splitted_6() {
		assert_eq!(
			split_positioned_named(
				r""),
			(Option::None, Option::None)
		);
	}

	#[test]
	fn params_transpiled_6() {
		assert_eq!(
			transpile_params(
				r""),
			""
		);
	}
}