mod token;
use token::{Token};
mod tokenizers;

fn main() {
	let lexer = powder::Lexer::<Token>::new(
		&[
			tokenizers::tokenize_separators, 
			tokenizers::tokenize_operators,
			tokenizers::tokenize_keyword,
			tokenizers::tokenize_literal,
			tokenizers::tokenize_identifier
		],
		tokenizers::skip
	);

	let text = 
"
module MyModule;
fun Program::main(argc: i32, argv: string[]) -> i32
{
	Program.running = true;
	var x : char = '\n';
	var str : string = \"Hello\\nWorld!\";
	return 12'345.678'9;
}
";

	let mut tokens = std::vec::Vec::<(Token, usize, usize)>::new();
	lexer.tokenize_into_list(text, &mut tokens);
	for elem in tokens
	{
		println!("{:?}", elem.0);
	}
	
}

