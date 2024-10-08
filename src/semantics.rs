use crate::flags::Options;
use crate::lexer::Span;
use crate::message::Context;
use crate::parser::ast::Program;
use std::collections::HashMap;

mod assign;
mod expression;
mod program;
mod statement;

pub struct FuncSig {
	return_type: String,
	param_types: Vec<String>,
}

pub struct VarSig {
	data_type: String,
	mutable: bool,
	span: Span,
	used: i64,
	changed: i64,
}

impl std::fmt::Display for FuncSig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"({}) -> {}",
			self.param_types.join(", "),
			self.return_type
		)
	}
}

pub struct Scope {
	functions: HashMap<String, FuncSig>,
	variables: HashMap<String, VarSig>,
}

impl Scope {
	fn new() -> Scope {
		Scope {
			functions: HashMap::new(),
			variables: HashMap::new(),
		}
	}
}

pub struct Analyzer<'a> {
	context: &'a Context<'a>,
	scopes: Vec<Scope>,
	func_stack: Vec<String>,
	loops: i64,
	flags: &'a Options,
}

impl<'a> Analyzer<'a> {
	const INT: &'static str = "int";
	const VOID: &'static str = "void";
	const FUNC_MAIN: &'static str = "main";

	pub fn run(ast: &Program, context: &'a Context, flags: &'a Options) -> Analyzer<'a> {
		let mut analyzer = Analyzer {
			context: context,
			scopes: vec![Scope::new()],
			func_stack: vec![],
			loops: 0,
			flags: flags,
		};

		analyzer.set_function(
			&String::from("print"),
			vec![Analyzer::INT.to_string()],
			Analyzer::VOID,
		);

		ast.analyze(&mut analyzer);
		analyzer
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new());
	}

	pub fn pop_scope(&mut self) -> Scope {
		self.scopes.pop().unwrap()
	}

	pub fn get_function(&self, name: &String) -> Option<&FuncSig> {
		for scope in &self.scopes {
			let func = scope.functions.get(name);
			match func {
				None => {}
				_ => {
					return func;
				}
			}
		}

		None
	}

	pub fn get_current_function(&self) -> Option<(&FuncSig, &String)> {
		match self.func_stack.last() {
			None => None,
			Some(func) => Some((self.get_function(func).unwrap(), func)),
		}
	}

	pub fn set_function(&mut self, name: &String, params: Vec<String>, return_type: &str) {
		let scope = self.scopes.last_mut().unwrap();
		scope.functions.insert(
			name.to_string(),
			FuncSig {
				return_type: return_type.to_string(),
				param_types: params,
			},
		);
	}

	pub fn valid_return_type(&self, return_type: &String) -> bool {
		["int", "void"].iter().any(|&s| s == return_type)
	}

	pub fn get_variable(&self, name: &String, all_scopes: bool) -> Option<&VarSig> {
		if all_scopes {
			for scope in &self.scopes {
				let var = scope.variables.get(name);
				match var {
					None => {}
					_ => {
						return var;
					}
				}
			}
			None
		} else {
			self.scopes.last().unwrap().variables.get(name)
		}
	}

	pub fn set_variable(&mut self, name: &String, data_type: &str, mutable: bool, span: Span) {
		let scope = self.scopes.last_mut().unwrap();
		scope.variables.insert(
			name.to_string(),
			VarSig {
				data_type: data_type.to_string(),
				mutable: mutable,
				span: Span {
					lo: span.lo,
					hi: span.hi,
				},
				used: 0,
				changed: 0,
			},
		);
	}

	pub fn change_variable(&mut self, name: &String) {
		for scope in &mut self.scopes {
			let var = scope.variables.get_mut(name);
			match var {
				None => {}
				Some(value) => {
					value.changed += 1;
				}
			}
		}
	}

	pub fn use_variable(&mut self, name: &String) {
		for scope in &mut self.scopes {
			let var = scope.variables.get_mut(name);
			match var {
				None => {}
				Some(value) => {
					value.used += 1;
				}
			}
		}
	}

	pub fn valid_data_type(&self, data_type: &String) -> bool {
		["int"].iter().any(|&s| s == data_type)
	}
}
