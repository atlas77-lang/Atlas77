use atlas_core::prelude::*;

lexer_builder! {
    DefaultSystem {
        number: true,
        symbol: true,
        keyword: true,
        string: true,
        whitespace: {
            allow_them: false,
            use_system: true,
        },
    },
    Symbols {
        Single {
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            ',' => Comma,
            '+' => OpAdd,
            '/' => OpDiv,
            '*' => OpMul,
            '^' => OpPow,
            '%' => OpMod,
            '\\' => BackSlash,
            '_' => Underscore,
            ';' => Semicolon,
            '\'' => Quote,
            '?' => Interrogation,
        },
        Either {
            '=' => '=' => OpEq, OpAssign,
            '!' => '=' =>  OpNEq, Bang,
            '.' => '.' => DoubleDot, Dot,
            ':' => ':' => DoubleColon, Colon,
            '-' => '>' => RArrow, OpSub,
            '<' => '=' => OpLessThanEq, OpLessThan,
            '>' => '=' => OpGreaterThanEq, OpGreaterThan,
            '&' => '&' => OpAnd, Ampersand,
            '|' => '|' => OpOr, Pipe,
            '~' => '>' => FatArrow, Tilde,
        }
    },
    Keyword {
        // Keywords
        "then", "if", "else", "struct", "true", "false", "let", "import", "return", "enum", "List", "end", "do", "match", "new", "extern", "as", 
        // Types
        "int", "uint", "float", "string", "List", "char",
    },
    Number {
        trailing {
            "_int"   => i64  => INT,
            "_uint"  => u64 => UINT,
            "_float" => f64  => FLOAT
        },
        float: true,
        u_int: true,
        int: true
    },
}
