use atlas_core::prelude::*;
//tbf, atlas-core should be rebuilt from the ground up
lexer_builder! {
    DefaultSystem {
        number: true,
        symbol: true,
        keyword: true,
        string: true,
        comment: true,
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
        //Items
        "class"     => KwClass,
        "func"      => KwFunc,
        "extern"    => KwExtern,
        "struct"    => KwStruct,
        "trait"     => KwTrait,
        "enum"      => KwEnum,
        "union"     => KwUnion,
        "import"    => KwImport,
        //Visibility
        "public"    => KwPublic,
        "private"   => KwPrivate,
        //Control Flow
        "if"        => KwIf,
        "else"      => KwElse,
        "match"     => KwMatch,
        //Loops
        "while"     => KwWhile,
        "break"     => KwBreak,
        "continue"  => KwContinue,
        //Return
        "return"    => KwReturn,
        //Variables
        "let"       => KwLet,
        "const"     => KwConst,
        //Misc
        "comptime"  => KwComptime,
        "as"        => KwAs,
        //Boolean
        "true"      => KwTrue, //should be fixed
        "false"     => KwFalse,
        //Primitive Types
        "i64"       => I64Ty,
        "f64"       => F64Ty,
        "u64"       => U64Ty,
        "unit"      => UnitTy,
        "char"      => CharTy,
        "bool"      => BoolTy,
        //Complex Types
        "str"       => StrTy,
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            TokenKind::Literal(l) => {
                write!(f, "{:?}", l)
            }
            TokenKind::KwElse => {
                write!(f, "else")
            }
            TokenKind::KwEnum => {
                write!(f, "enum")
            }
            TokenKind::KwExtern => {
                write!(f, "extern")
            }
            _ => {
                write!(f, "{:?}", self.kind())
            }
        }
    }
}

#[derive(Debug)]
pub struct TokenVec(pub Vec<TokenKind>);

impl std::fmt::Display for TokenVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(f, "{:?} ", token)?;
        }
        Ok(())
    }
}
