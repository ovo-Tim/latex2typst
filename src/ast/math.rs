//! Math expression AST nodes

/// Mathematical expression
#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    /// Symbol or variable (e.g., "x", "alpha", "1")
    Symbol(String),

    /// LaTeX command with arguments (e.g., \frac{a}{b})
    Command { name: String, args: Vec<MathExpr> },

    /// Subscript (base_sub)
    Subscript {
        base: Box<MathExpr>,
        sub: Box<MathExpr>,
    },

    /// Superscript (base^sup)
    Superscript {
        base: Box<MathExpr>,
        sup: Box<MathExpr>,
    },

    /// Combined subscript and superscript (base_sub^sup)
    SubSup {
        base: Box<MathExpr>,
        sub: Box<MathExpr>,
        sup: Box<MathExpr>,
    },

    /// Fraction (numerator/denominator)
    Fraction {
        num: Box<MathExpr>,
        den: Box<MathExpr>,
    },

    /// Square root or nth root
    Sqrt {
        degree: Option<Box<MathExpr>>,
        radicand: Box<MathExpr>,
    },

    /// Group of expressions ({...})
    Group(Vec<MathExpr>),

    /// Binary operation (a + b, a * b, etc.)
    Binary {
        op: String,
        left: Box<MathExpr>,
        right: Box<MathExpr>,
    },

    /// Math operator with optional limits (sum, int, etc.)
    Operator {
        name: String,
        lower: Option<Box<MathExpr>>,
        upper: Option<Box<MathExpr>>,
    },

    /// Delimited expression (e.g., \left( ... \right))
    Delimited {
        left: String,
        content: Box<MathExpr>,
        right: String,
    },

    /// Environment (matrix, aligned, cases, etc.)
    Environment {
        name: String,
        content: Vec<Vec<MathExpr>>,
    },

    /// Text in math mode
    Text(String),

    /// Space
    Space,
}

impl MathExpr {
    /// Create a simple symbol
    pub fn symbol(s: impl Into<String>) -> Self {
        MathExpr::Symbol(s.into())
    }

    /// Create a command with arguments
    pub fn command(name: impl Into<String>, args: Vec<MathExpr>) -> Self {
        MathExpr::Command {
            name: name.into(),
            args,
        }
    }

    /// Create a fraction
    pub fn fraction(num: MathExpr, den: MathExpr) -> Self {
        MathExpr::Fraction {
            num: Box::new(num),
            den: Box::new(den),
        }
    }

    /// Create a square root
    pub fn sqrt(radicand: MathExpr) -> Self {
        MathExpr::Sqrt {
            degree: None,
            radicand: Box::new(radicand),
        }
    }

    /// Create an nth root
    pub fn nthroot(degree: MathExpr, radicand: MathExpr) -> Self {
        MathExpr::Sqrt {
            degree: Some(Box::new(degree)),
            radicand: Box::new(radicand),
        }
    }

    /// Create a subscript
    pub fn subscript(base: MathExpr, sub: MathExpr) -> Self {
        MathExpr::Subscript {
            base: Box::new(base),
            sub: Box::new(sub),
        }
    }

    /// Create a superscript
    pub fn superscript(base: MathExpr, sup: MathExpr) -> Self {
        MathExpr::Superscript {
            base: Box::new(base),
            sup: Box::new(sup),
        }
    }

    /// Create a group
    pub fn group(exprs: Vec<MathExpr>) -> Self {
        MathExpr::Group(exprs)
    }

    /// Check if this is a simple expression (single symbol or number)
    pub fn is_simple(&self) -> bool {
        matches!(self, MathExpr::Symbol(_) | MathExpr::Text(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_creation() {
        let sym = MathExpr::symbol("x");
        assert_eq!(sym, MathExpr::Symbol("x".to_string()));
    }

    #[test]
    fn test_fraction_creation() {
        let frac = MathExpr::fraction(MathExpr::symbol("a"), MathExpr::symbol("b"));

        match frac {
            MathExpr::Fraction { num, den } => {
                assert_eq!(*num, MathExpr::Symbol("a".to_string()));
                assert_eq!(*den, MathExpr::Symbol("b".to_string()));
            }
            _ => panic!("Expected fraction"),
        }
    }

    #[test]
    fn test_superscript() {
        let expr = MathExpr::superscript(MathExpr::symbol("x"), MathExpr::symbol("2"));

        match expr {
            MathExpr::Superscript { base, sup } => {
                assert_eq!(*base, MathExpr::Symbol("x".to_string()));
                assert_eq!(*sup, MathExpr::Symbol("2".to_string()));
            }
            _ => panic!("Expected superscript"),
        }
    }

    #[test]
    fn test_sqrt() {
        let expr = MathExpr::sqrt(MathExpr::symbol("x"));

        match expr {
            MathExpr::Sqrt { degree, radicand } => {
                assert!(degree.is_none());
                assert_eq!(*radicand, MathExpr::Symbol("x".to_string()));
            }
            _ => panic!("Expected sqrt"),
        }
    }

    #[test]
    fn test_is_simple() {
        assert!(MathExpr::symbol("x").is_simple());
        assert!(MathExpr::Text("hello".to_string()).is_simple());
        assert!(!MathExpr::fraction(MathExpr::symbol("a"), MathExpr::symbol("b")).is_simple());
    }
}
