#[macro_use] extern crate lazy_static;

use std::io::Read;
use std::borrow::Cow;
use std::fs::File;
use std::io;

mod cleanup;
mod funcs;

pub fn transpile_file<'a>(filename: &str) -> Result<String, io::Error> {
	let mut file = File::open(filename)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(transpile(&contents).to_string())
}

pub fn transpile<'a>(input: &'a str) -> Cow<'a, str> {
	let result = cleanup::remove_import(input);
	return result;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_transpile() {
		assert_eq!(
			transpile(r"import 'package:flutter/widgets.dart';
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
}