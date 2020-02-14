use regex::Regex;

/// Given a stirng representing an item in parameter list
/// Add default value (null) to it if it doesn't have yet
/// If there is @required, replace it with a /* TODO: check null */
fn append_default_value<'a>(input: &'a str) -> String {
	lazy_static! {
		static ref RE : Regex = Regex::new(r"(?P<nospace>\S)(?P<trailing>\s*)$").unwrap();
	}

	let input = if !input.contains("=") {
		RE.replace_all(input, "$nospace = null$trailing").to_string()
	} else {
		String::from(input)
	};
	str::replace(&input, "@required", "/* TODO: check null */")
}

/// If a function parameter is a function, replace it with an
/// Action or Function type parameter
fn create_function_action<'a>(input: &'a str) -> String {
	lazy_static! {
		static ref RE : Regex = Regex::new(r"(?x)
			^(?P<leading>\s*)(?P<rtype>\w+)\s+(?P<fname>\w+)\s*\( # Function return type and name
				\s*(?P<params>(\w+\s+\w+\s*,)*\s*(\w+\s+\w+)?)?\s* # Function parameter list
			\)(?P<trailing>\s*)$
		").unwrap();
		static ref REP : Regex = Regex::new(r"(?P<type>\w+)\s+(?P<name>\w+)").unwrap();
	}

	match RE.captures(input) {
		Some(cap) => {
			let return_type = cap.name("rtype").unwrap().as_str();
			let func_name = cap.name("fname").unwrap().as_str();
			let leading = cap.name("leading").unwrap().as_str();
			let trailing = cap.name("trailing").unwrap().as_str();
			let params = match cap.name("params") {
				Some(expr) => REP.replace_all(expr.as_str(), "$type").to_string(),
				None => String::from("")
			};
			if return_type == "void" {
				return if params.is_empty() {
					format!("{}Action {}{}", leading, func_name, trailing)
				} else {
					format!("{}Action<{}> {}{}", leading, params, func_name, trailing)
				};
			}
			return if params.is_empty() {
				format!("{}Function<{}> {}{}", leading, return_type, func_name, trailing)
			} else {
				format!("{}Function<{}, {}> {}{}", leading, params, return_type, func_name, trailing)
			}
		},
		None => String::from(input)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_function_action_1() {
		assert_eq!(
			create_function_action(r"
  void f(int a, float b)
  "),
			r"
  Action<int, float> f
  ");
	}

	#[test]
	fn is_function_action_2() {
		assert_eq!(
			create_function_action(r"
  void f(int a)
  "),
			r"
  Action<int> f
  ");
	}

	#[test]
	fn is_function_action_3() {
		assert_eq!(
			create_function_action(r"
  void f()
  "),
			r"
  Action f
  ");
	}

	#[test]
	fn is_function_action_4() {
		assert_eq!(
			create_function_action(r"
  int f(int a, float b)
  "),
			r"
  Function<int, float, int> f
  ");
	}

	#[test]
	fn is_function_action_5() {
		assert_eq!(
			create_function_action(r"
  int f(int a)
  "),
			r"
  Function<int, int> f
  ");
	}

	#[test]
	fn is_function_action_6() {
		assert_eq!(
			create_function_action(r"
  int f()
  "),
			r"
  Function<int> f
  ");
	}

	#[test]
	fn has_default_value_1() {
		assert_eq!(
			append_default_value(r"
  Iterable stylesPrepend
  "),
			r"
  Iterable stylesPrepend = null
  ");
	}

	#[test]
	fn has_default_value_2() {
		assert_eq!(
			append_default_value(r"
  Iterable<String> stylesPrepend
  "),
			r"
  Iterable<String> stylesPrepend = null
  ");
	}

	#[test]
	fn has_default_value_3() {
		assert_eq!(
			append_default_value(r"
  double stylesPrepend = 0.0
  "),
			r"
  double stylesPrepend = 0.0
  ");
	}

	#[test]
	fn has_default_value_4() {
		assert_eq!(
			append_default_value(r"
  @required Iterable<String> stylesPrepend
  "),
			r"
  /* TODO: check null */ Iterable<String> stylesPrepend = null
  ");
	}

	#[test]
	fn has_default_value_5() {
		assert_eq!(
			append_default_value(r"
  @required this.styles
  "),
			r"
  /* TODO: check null */ this.styles = null
  ");
	}

}