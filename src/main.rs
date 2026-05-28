use std::{
	fs,
	env,
	panic,
	io::ErrorKind
};
use clap::{
	Parser,
	ArgAction
};
use rustyline::{
	DefaultEditor,
	error::{
		ReadlineError,
		ReadlineError::Io
	}
};

mod scanner;

use scanner::Scanner;

#[derive(Parser)]
#[command(disable_help_flag = true, version, disable_version_flag = true)]
struct Args {
	#[arg(short, action = ArgAction::Help, hide = true)]
	help: Option<bool>,

	/// Show version and exit
	#[arg(short, action = ArgAction::Version)]
	version: Option<bool>,

	filenames: Vec<String>
}

fn main() {
	set_panic_hook();

	let args = Args::parse();

	if args.filenames.is_empty() {
		repl();

		return;
	}

	for mut filename in args.filenames {
		filename.push_str(".iara");

		let content = match fs::read_to_string(&filename) {
			Ok(content) => content,
			Err(error) => panic!("Could not read {}: {}", filename, error)
		};

		run_code(content);
	}
}

fn set_panic_hook() {
	panic::set_hook(Box::new(|panic_info| {
		if let Some(error) = panic_info.payload_as_str() {
			eprintln!("{}", error);
		}
	}));
}

fn repl() {
	let mut home_dir = match env::home_dir() {
		Some(home_dir) => home_dir,
		None => panic!("Could not find your home directory")
	};

	home_dir.push(".iara_history");

	let history_path = match home_dir.to_str() {
		Some(history_path) => history_path,
		None => panic!("Could not convert {} to string", home_dir.display())
	};

	let mut default_editor = match DefaultEditor::new() {
		Ok(default_editor) => default_editor,
		Err(error) => panic!("Could not create a new DefaultEditor: {}", error)
	};

	if let Err(error) = default_editor.load_history(history_path) && let Io(error) = error && error.kind() != ErrorKind::NotFound {
		panic!("Could not load history file located at {}: {}", history_path, error);
	}

	loop {
		let code_line = match default_editor.readline("iara > ") {
			Ok(code_line) => code_line,
			Err(ReadlineError::Eof | ReadlineError::Interrupted) => break,
			Err(error) => panic!("Could not read code line: {}", error)
		};

		if let Err(error) = default_editor.add_history_entry(&code_line) {
			panic!("Could not add entry in history file (located at {}): {}", history_path, error);
		}

		run_code(code_line);
	}

	if let Err(error) = default_editor.save_history(history_path) {
		panic!("Could not save history file (located at {}): {}", history_path, error);
	}
}

fn run_code(source_code: String) {
	let scanner = Scanner::new(source_code);

	let tokens = scanner.scan();

	for token in tokens {
		println!("{:?}", token);
	}
}