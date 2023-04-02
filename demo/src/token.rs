#[derive(std::clone::Clone)]
#[derive(std::fmt::Debug)]
pub enum Token
{
	/* SEPARATORS */
	Comma,				// ","
	Semicolon,			// ";"
	ParenthesesOpen,	// "("
	ParenthesesClose,	// ")"
	BracketsOpen,		// "["
	BracketsClose,		// "]"
	BracesOpen,			// "{"
	BracesClose,		// "}"

	/* OPERATORS */
	Scope,				// "::"
	ReturnType,			// "->"
	Equals,				// "=="
	NotEqual,			// "<>"
	GreaterEqual,		// ">="
	LesserEqual,		// "<="
	Increment,			// "++"
	Decremnet,			// "--"
	ExclamationAssign,	// "!="
	AmpersandAssign,	// "&="
	VerticalPipeAssign,	// "|="
	CaretAssign,		// "^="
	MinusAssign,		// "-="
	StarAssign,			// "*="
	SlashAssign,		// "/="
	PercentAssign,		// "%="
	TildeAssign,		// "~="
	AtAssign,			// "@="
	PoundAssign,		// "#="
	DolarAssign,		// "$="
	Period,				// "."
	Colon,				// ":"
	Assign,				// "="
	Greater,			// ">"
	Lesser,				// "<"
	Exclamation,		// "!"
	Ampersand,			// "&"
	VerticalPipe,		// "|"
	Caret,				// "^"
	Plus,				// "+"
	Minus,				// "-"
	Star,				// "*"
	Slash,				// "/"
	Percent,			// "%"
	Tilde,				// "~"
	At,					// "@"
	NumberSign,			// "#"
	Dolar,				// "$"

	/* KEYWORDS */
	Module,				// "module"
	Import,				// "import"
	Export,				// "export"
	Variable,			// "var"
	Function,			// "fun"
	Ret,				// "return"
	As,					// "as"
	Is,					// "is"
	Type,				// "type"
	Match,				// "match"
	If,					// "if"
	Else,				// "else"
	Discard,			// "_"
	ThisType,			// "This"
	ThisObj,			// "this"

	/* LITERALS */
	Boolean(bool),		// "true"
	Integer(String),	// "0b11000011", "12'345.678'90f32", etc.
	Float(String),		// "0b11000011", "12'345.678'90f32", etc.
	Character(char),	// "'a'", "'\n'", "'\0xF00F'", etc.
	String(String),		// "\"Hello, world!\""

	/* IDENTIFIERS */
	Identifier(String),	// "x", "a", "_i", "name", "a1", etc. 
}
