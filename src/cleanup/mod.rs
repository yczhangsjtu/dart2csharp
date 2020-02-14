use regex::Regex;
use std::borrow::Cow;

pub fn remove_import<'a>(input: &'a str) -> Cow<'a, str> {
	lazy_static! {
		static ref RE : Regex = Regex::new(r"(^import\s+.*\n)*(import\s+.*)(\s*\n)*").unwrap();
	}

	return RE.replace_all(input, "");
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn import_removed_1() {
		assert_eq!(
			remove_import(r"import 'package:flutter/widgets.dart';
import 'package:html/dom.dart' as dom;

NodeMetadata lazySet(
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
})"),
			r"NodeMetadata lazySet(
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
})"
		);
	}

	#[test]
	fn import_removed_2() {
		assert_eq!(
			remove_import(r"import 'package:flutter/widgets.dart';
import 'package:html/dom.dart' as dom;
NodeMetadata lazySet(
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
})"),
			r"NodeMetadata lazySet(
  NodeMetadata meta, {
  BuildOp buildOp,
  Iterable<String> stylesPrepend,
})"
		);
	}

	#[test]
	fn import_removed_3() {
		assert_eq!(
			remove_import(r"import 'package:flutter/widgets.dart';
import 'package:html/dom.dart' as dom;
"),
			r""
		);
	}

	#[test]
	fn import_removed_4() {
		assert_eq!(
			remove_import(r"import 'package:flutter/widgets.dart';
import 'package:html/dom.dart' as dom;"),
			r""
		);
	}

	#[test]
	fn import_removed_5() {
		assert_eq!(
			remove_import(r"class{}
import 'package:flutter/widgets.dart';
import 'package:html/dom.dart' as dom;"),
			r"class{}
"
		);
	}

	#[test]
	fn import_removed_6() {
		assert_eq!(
			remove_import(r"class{}
import 'package:flutter/widgets.dart';
import 'package:html/dom.dart' as dom;
class{}
import 'package:html/dom.dart' as dom;
class{}"),
			r"class{}
class{}
class{}"
		);
	}
}