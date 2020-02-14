use std::io::Write;
use std::fs::File;
use dart2csharp;

fn main() {
	let mut file = File::create("test.cs").unwrap();
	file.write_all(dart2csharp::transpile_file("test.dart").unwrap().as_bytes()).unwrap();
}
