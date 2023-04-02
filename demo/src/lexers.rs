use super::token::{Token};

pub fn skip(text: &str) -> usize {
	let mut skip = 0;
	for ch in text.chars()
	{
		if ch.is_whitespace() { skip = skip + 1; }
		else { break; }
	}
	return skip;
}

pub fn tokenize_separators(text: &str) -> Option<(Token, usize)> {
	for elem in SEPARATORS {
		if !text.starts_with(elem.0) { continue; }	// MATCH TOKEN PATTERN

		return Some((elem.1.clone(), elem.0.len()));
	}
	return None;
}

pub fn tokenize_operators(text: &str) -> Option<(Token, usize)> {
	for elem in OPERATORS {
		if !text.starts_with(elem.0) { continue; }	// MATCH TOKEN PATTERN

		return Some((elem.1.clone(), elem.0.len()));
	}
	return None;
}

pub fn tokenize_keyword(text: &str) -> Option<(Token, usize)> {
	for elem in KEYWORDS {
		if !text.starts_with(elem.0) { continue; }	// MATCH TOKEN PATTERN

		match text[elem.0.len()..].chars().nth(0) {	// VALIDATE NEXT CHARACTER

			None => Some((elem.1.clone(), elem.0.len())),

			Some(ch) => 
			if !ch.is_alphabetic() && !ch.is_digit(10) {
				return Some((elem.1.clone(), elem.0.len()))
			}
			else {
				return None
			}
		};
	}
	return None;
}

pub fn tokenize_literal(text: &str) -> Option<(Token, usize)> {
	if let Some((token, consumed)) = tokenize_boolean(text) { return Some((token, consumed)) }
	if let Some((token, consumed)) = tokenize_number(text) { return Some((token, consumed)) }
	if let Some((token, consumed)) = tokenize_char(text) { return Some((token, consumed)) }
	if let Some((token, consumed)) = tokenize_string(text) { return Some((token, consumed)) }
	
	return None;
}

pub fn tokenize_identifier(text: &str) -> Option<(Token, usize)> {
	let mut consumed = 0;
	for ch in text.chars() {
		if ch == '_' || ch.is_alphabetic() || (consumed != 0 && ch.is_digit(10)) {
			consumed = consumed + 1;
		}
		else {
			let t : Token = Token::Identifier(text[..consumed].to_string());
			return if consumed == 0 { None } else { Some((t, consumed)) };
		}
	}
	return None;
}

pub fn tokenize_boolean(text: &str) -> Option<(Token, usize)> {
	for elem in LITERALS {
		if !text.starts_with(elem.0) { continue; }	// MATCH TOKEN PATTERN

		match text[elem.0.len()..].chars().nth(0) {	// VALIDATE NEXT CHARACTER
			None => Some((elem.1.clone(), elem.0.len())),

			Some(ch) => 
			if !ch.is_alphabetic() && !ch.is_digit(10) {
				return Some((elem.1.clone(), elem.0.len()))
			}
			else {
				return None
			}
		};
	}
	return None;
}

pub fn tokenize_number(text: &str) -> Option<(Token, usize)> {
	let mut consumed : usize = 0;
	let mut base : u32 = 10;
	let mut base_len : usize = 0;
	let mut flt	: bool = false;
	let mut sep : bool = true;

	for &base_def in BASES {
		base_def.0.iter().for_each(|&elem| { if text.starts_with(elem) { base = base_def.1; base_len = elem.len(); } } );
	}
	
	for ch in text[base_len..].chars() {
		if ch.is_digit(base) {
			sep = false;
		}
		else if ch == DIGIT_SEPARATOR && !sep {
			sep = true;
		}
		else if base == 10 && ch == DECIMAL_SEPARATOR && flt == false && consumed > 0 {
			flt = true;
		}
		else {
			break;
		}
		consumed += 1;
	}
	if consumed > 0 && !sep {
		let ret =
			if flt 	{ Token::Float(text[..consumed].to_string()) }
			else 	{ Token::Integer(text[..consumed].to_string()) };
		return Some((ret, consumed));
	}

	return None;
}

pub fn tokenize_char(text: &str) -> Option<(Token, usize)> {
	if !text.starts_with(CHAR_QUOTE) { return None; }

	let inside: &str= &text[1..];

	// HEX CHARACRER
	if inside.starts_with(HEX_ESCAPE) { /* TODO */ }

	// ESCAPE SEQUENCES
	match parse_escape_sequence(inside) {
		None => {},
		Some(pair) => {
			if inside[pair.1..].starts_with(CHAR_QUOTE) {
				return Some((Token::Character(pair.0), CHAR_QUOTE.len() * 2 + pair.1));
			}
			else {
				return None;
			}
		},
	}

	// NORMAL CHARACTERS
	if !inside[1..].starts_with(CHAR_QUOTE) { return None; }
	match inside.chars().nth(0)
	{
		Some(ch) => return Some((Token::Character(ch), CHAR_QUOTE.len() * 2 + 1)),
		None => {}
	}

	return None;
}

pub fn tokenize_string(text: &str) -> Option<(Token, usize)> {
	if !text.starts_with(STRING_QUOTE) { return None; }

	let quote_len : usize		= STRING_QUOTE.len();
	let mut stream : String		= String::with_capacity(256);
	let mut consumed : usize	= quote_len;

	loop {
		let next = &text[consumed..];
		// STRING END
		if next.starts_with(STRING_QUOTE) {
			consumed += STRING_QUOTE.len();
			return Some((Token::String(stream.clone()), consumed));
		}

		// ESCAPE SEQUENCES
		match parse_escape_sequence(next)
		{
			None 		=> { },
			Some(pair) 	=> { stream.push(pair.0); consumed += pair.1; continue; }
		}

		// NORMAL CHARACTERS
		match next.chars().nth(0)
		{
			Some(ch) => {
				stream.push(ch); 
				consumed += 1;
				continue;
			},
			None => {}
		}
	}
}

fn parse_escape_sequence(text: &str) -> Option<(char, usize)> {
	for esc in ESCAPE_SEQ {
		if text.starts_with(esc.0) {
			return Some((esc.1, esc.0.len()));
		}
	}
	return None;
}

static SEPARATORS: &[(&str, Token)] = 
&[
	(".", Token::Period),
	(",", Token::Comma),
	(":", Token::Colon),
	(";", Token::Semicolon),
	("(", Token::ParenthesesOpen),
	(")", Token::ParenthesesClose),
	("[", Token::BracketsOpen),
	("]", Token::BracketsClose),
	("{", Token::BracesOpen),
	("}", Token::BracesClose),
];
static OPERATORS: &[(&str, Token)] = 
&[
	("==", 	Token::Equals),
	("<>", 	Token::NotEqual),
	(">=", 	Token::GreaterEqual),
	("<=", 	Token::LesserEqual),
	("++", 	Token::Increment),
	("--", 	Token::Decremnet),
	("!=", 	Token::ExclamationAssign),
	("&=", 	Token::AmpersandAssign),
	("|=", 	Token::VerticalPipeAssign),
	("^=", 	Token::CaretAssign),
	("-=", 	Token::MinusAssign),
	("*=", 	Token::StarAssign),
	("/=", 	Token::SlashAssign),
	("%=", 	Token::PercentAssign),
	("~=", 	Token::TildeAssign),
	("@=", 	Token::AtAssign),
	("#=", 	Token::PoundAssign),
	("$=", 	Token::DolarAssign),
	("=", 	Token::Assign),
	(">", 	Token::Greater),
	("<", 	Token::Lesser),
	("!", 	Token::Exclamation),
	("&", 	Token::Ampersand),
	("|", 	Token::VerticalPipe),
	("^", 	Token::Caret),
	("+", 	Token::Plus),
	("-", 	Token::Minus),
	("*", 	Token::Star),
	("/", 	Token::Slash),
	("%", 	Token::Percent),
	("~", 	Token::Tilde),
	("@", 	Token::At),
	("#", 	Token::NumberSign),
	("$", 	Token::Dolar),
];
static KEYWORDS: &[(&str, Token)] = 
&[
	("module",		Token::Module),
	("import",		Token::Import),
	("export",		Token::Export),
	("var",			Token::Variable),
	("fun",			Token::Function),
	("return",		Token::Ret),
	("as",			Token::As),
	("is",			Token::Is),
	("type",		Token::Type),
	("match",		Token::Match),
	("if",			Token::If),
	("else",		Token::Else),
	("_",			Token::Discard),
	("This",		Token::ThisType),
	("this",		Token::ThisObj),
];

static LITERALS: &[(&str, Token)] = 
&[
	("true",		Token::Boolean(true)),
	("false",		Token::Boolean(false)),
];

static ESCAPE_SEQ : &[(&str, char)]  =
&[
	("\\\'", '\''),
	("\\\"", '\"'),
	("\\\\", '\\'),
	("\\0",  '\0'),
	("\\t",  '\t'),
	("\\r",  '\r'),
	("\\n",  '\n'),
	// TODO add more escape sequences
];
static HEX_ESCAPE : &str = "\\x";
static CHAR_QUOTE : &str = "\'";
static STRING_QUOTE : &str = "\"";

static BASES: &[(&[&str], u32)] = &[
	(&["0b", "0B"], 2),
	(&["0o", "0O"], 8),
	(&["0x", "0X"], 16),
];
static DIGIT_SEPARATOR: char = '\'';
static DECIMAL_SEPARATOR: char = '.';

