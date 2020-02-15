use regex::Captures;
use regex::Regex;
use std::borrow::Cow;
use crate::utils;

/// Process parameter list of function
mod param_list;
mod param_item;

/// Transpile function headers. Specifically, to transform the
/// dart-style positional and named parameters into C# style.
/// Remove the pair of curly brace indicating the range of named
/// parameters and provide default values if not exists yet.
/// Transform functional parameter into Action and Function.
/// This will ignore the function headers without any function
/// body, because it is hard to differentiate them from function
/// invocations.
pub fn transpile_func_head<'a>(input: &'a str) -> Cow::<'a, str> {

	lazy_static! {
		static ref RE : Regex = Regex::new(r"(?x)(?m)
			^(?P<leading_space>\s*)
			(?:
				(?P<rtype>\w+) # Return type
				\s+
			)? # No return type for initializer
			(?P<fname>\w+) # Function name

			\s*
			\(\s* # Start of parameter list
				(?P<params>
					(?:
						[^()]*
						(?:\([^()]*\)[^()]*)*  # Contains several () or not
						\n
					)* # Lines
					(?:
						[^()]*
						(?:\([^()]*\)[^()]*)*  # Contains several () or not
					)? # Line without eol
				)
			\s*\) # End of parameter list

      (?P<trailing>\s*(?::|=>|\{)) # To differentiate function header from funtion invocation
		"
		).unwrap();
	}

	RE.replace_all(input, |cap: &Captures| -> String {
		let leading_space = cap.name("leading_space").unwrap().as_str();
		let return_type = cap.name("rtype");
		let func_name = match cap.name("fname") {
			Some(name) => name.as_str(),
			None => panic!("No function name!")
		};
    if utils::is_keyword(func_name) {
      return cap.get(0).unwrap().as_str().to_string();
    }
		let is_public = !func_name.starts_with("_");
		let params = match cap.name("params") {
			Some(content) => content.as_str(),
			None => panic!("No parameter list!")
		};
    let trailing = cap.name("trailing").unwrap().as_str();
		return format!("{}{}{}{}({}){}",
			leading_space,
			if is_public {"public "} else {""},
			if let Some(typename) = return_type {
				format!("{} ", typename.as_str())
			} else {
				String::from("")
			},
			func_name,
			param_list::transpile_params(params),
      trailing
		)
	})
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn correct_func_head_1() {
		assert_eq!(
			transpile_func_head(r"NodeMetadata lazySet(
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
}) {"),
			r"public NodeMetadata lazySet(NodeMetadata meta,
BuildOp buildOp = null,
Iterable<String> stylesPrepend = null) {"
		);
	}

	#[test]
	fn correct_func_head_2() {
		assert_eq!(
			transpile_func_head(r"
  final TextBlock block;
  final Iterable<Widget> widgets;

  BuiltPieceSimple({
    this.block,
    this.widgets
  }) : assert((block == null) != (widgets == null));"),
			r"
  final TextBlock block;
  final Iterable<Widget> widgets;

  public BuiltPieceSimple(this.block = null,
this.widgets = null) : assert((block == null) != (widgets == null));"
		);
	}

	#[test]
	fn correct_func_head_3() {
		assert_eq!(
			transpile_func_head(r"CssMargin copyWith({
    CssLength bottom,
    CssLength left,
    CssLength right,
    CssLength top,
  }) =>
      CssMargin()
        ..bottom = bottom ?? this.bottom
        ..left = left ?? this.left
        ..right = right ?? this.right
        ..top = top ?? this.top;"),

			r"public CssMargin copyWith(CssLength bottom = null,
CssLength left = null,
CssLength right = null,
CssLength top = null) =>
      CssMargin()
        ..bottom = bottom ?? this.bottom
        ..left = left ?? this.left
        ..right = right ?? this.right
        ..top = top ?? this.top;"
		);
	}

	#[test]
	fn correct_func_head_4() {
		assert_eq!(
			transpile_func_head(r"DataBit(this.block, this.data, this.tsb, {this.onTap})
      : assert(block != null),
        assert(data != null),
        assert(tsb != null);"),

			r"public DataBit(this.block,
this.data,
this.tsb,
this.onTap = null)
      : assert(block != null),
        assert(data != null),
        assert(tsb != null);"
		);
	}

	#[test]
	fn correct_func_head_51() {
		assert_eq!(
			transpile_func_head(r"
  void styles(void f(String key, String value)) {
    _stylesFrozen = true;
    if (_styles == null) return;

    final iterator = _styles.iterator;
    while (iterator.moveNext()) {
      final key = iterator.current;
      if (!iterator.moveNext()) return;
      f(key, iterator.current);
    }
  }"),
			r"
  public void styles(Action<String, String> f) {
    _stylesFrozen = true;
    if (_styles == null) return;

    final iterator = _styles.iterator;
    while (iterator.moveNext()) {
      final key = iterator.current;
      if (!iterator.moveNext()) return;
      f(key, iterator.current);
    }
  }"
		);
	}

	#[test]
	fn correct_func_head_52() {
		assert_eq!(
			transpile_func_head(r"
  void styles(int f(String key, String value)) {
    _stylesFrozen = true;
    if (_styles == null) return;

    final iterator = _styles.iterator;
    while (iterator.moveNext()) {
      final key = iterator.current;
      if (!iterator.moveNext()) return;
      f(key, iterator.current);
    }
  }"),
			r"
  public void styles(Function<String, String, int> f) {
    _stylesFrozen = true;
    if (_styles == null) return;

    final iterator = _styles.iterator;
    while (iterator.moveNext()) {
      final key = iterator.current;
      if (!iterator.moveNext()) return;
      f(key, iterator.current);
    }
  }"
		);
	}

	#[test]
	fn correct_func_head_6() {
		assert_eq!(
			transpile_func_head(r"
  DataBit rebuild({
    String data,
    VoidCallback onTap,
    TextStyleBuilders tsb,
  }) =>
      DataBit(
        block,
        data ?? this.data,
        tsb ?? this.tsb,
        onTap: onTap ?? this.onTap,
      );"),
			r"
  public DataBit rebuild(String data = null,
VoidCallback onTap = null,
TextStyleBuilders tsb = null) =>
      DataBit(
        block,
        data ?? this.data,
        tsb ?? this.tsb,
        onTap: onTap ?? this.onTap,
      );"
		);
	}

	#[test]
	fn correct_func_head_7() {
		assert_eq!(
			transpile_func_head(r"
  BuiltPieceSimple({
    this.block,
    this.widgets,
  }) : assert((block == null) != (widgets == null));"),
			r"
  public BuiltPieceSimple(this.block = null,
this.widgets = null) : assert((block == null) != (widgets == null));"
		);
	}

	#[test]
	fn correct_func_head_8() {
		assert_eq!(
			transpile_func_head(r"
class BuiltPieceSimple extends BuiltPiece {
  final TextBlock block;
  final Iterable<Widget> widgets;

  BuiltPieceSimple({
    this.block,
    this.widgets,
  }) : assert((block == null) != (widgets == null));

  bool get hasWidgets => widgets != null;
}"),
			r"
class BuiltPieceSimple extends BuiltPiece {
  final TextBlock block;
  final Iterable<Widget> widgets;

  public BuiltPieceSimple(this.block = null,
this.widgets = null) : assert((block == null) != (widgets == null));

  bool get hasWidgets => widgets != null;
}"
		);
	}
}