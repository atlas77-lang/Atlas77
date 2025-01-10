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
            '@' => CompTime,
        },
        Either {
            '=' => '=' => OpEq, OpAssign,
            '!' => '=' => OpNEq, Bang,
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
        "impure"    => KwImpure,
        "extern"    => KwExtern,
        "then"      => KwThen,
        "if"        => KwIf,
        "else"      => KwElse,
        "struct"    => KwStruct,
        "true"      => KwTrue,
        "false"     => KwFalse,
        "let"       => KwLet,
        "include"   => KwInclude,
        "return"    => KwReturn, //will probably be removed at one point
        "enum"      => KwEnum,
        "end"       => KwEnd,
        "do"        => KwDo,
        "i64"       => I64Ty,
        "f64"       => F64Ty,
        "u64"       => U64Ty,
        "char"      => CharTy,
        "bool"      => BoolTy,
        "str"       => StrTy,
        "List"      => ListTy,
    },
    Number {
        trailing {
            "_i64"  => i64  => I64,
            "_u64"  => u64  => U64,
            "_f64"  => f64  => F64
        },
        float: true,
        u_int: true,
        int: true
    },
}
