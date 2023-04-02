mod token;
use token::{Token};
mod lexers;

fn main() {
	let lexer = powder::Lexer::<Token>::new(
		&[
			lexers::tokenize_separators, 
			lexers::tokenize_operators,
			lexers::tokenize_keyword,
			lexers::tokenize_literal,
			lexers::tokenize_identifier
		],
		lexers::skip
	);

	let text = 
"
module MyModule;
fun main(argc: i32, argv: string[]) -> i32
{
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

