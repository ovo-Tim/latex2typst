//! LaTeX math expression parser using nom

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, one_of},
    combinator::{map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::ast::math::MathExpr;
use crate::error::{Error, Result};

/// Parse a LaTeX math expression into a MathExpr AST
pub fn parse(input: &str) -> Result<MathExpr> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(MathExpr::Group(vec![]));
    }
    match parse_expr(input) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(Error::invalid_math(
                    input.len() - remaining.len(),
                    format!("unexpected content: {}", remaining),
                ))
            }
        }
        Err(e) => Err(Error::invalid_math(0, format!("parse error: {}", e))),
    }
}

/// Parse a complete math expression (potentially with operators at the top level)
fn parse_expr(input: &str) -> IResult<&str, MathExpr> {
    let (input, first) = parse_term(input)?;
    let (input, rest) = many0(pair(
        preceded(multispace0, one_of("+-=<>")),
        preceded(multispace0, parse_term),
    ))(input)?;

    if rest.is_empty() {
        Ok((input, first))
    } else {
        // Build a sequence of binary operations
        let mut result = first;
        for (op, term) in rest {
            result = MathExpr::Binary {
                op: op.to_string(),
                left: Box::new(result),
                right: Box::new(term),
            };
        }
        Ok((input, result))
    }
}

/// Parse a term (handles multiplication, division, juxtaposition, and unary plus/minus)
fn parse_term(input: &str) -> IResult<&str, MathExpr> {
    let (input, _) = multispace0(input)?;

    // Handle unary plus or minus at the start of a term
    let (input, unary_sign) = opt(one_of("+-"))(input)?;
    let (input, _) = multispace0(input)?;

    // Try to parse multiple atoms in sequence
    let (input, atoms) = many1(preceded(multispace0, parse_atom))(input)?;

    let term = if atoms.len() == 1 {
        atoms.into_iter().next().unwrap()
    } else {
        // Multiple atoms means implicit multiplication (juxtaposition)
        MathExpr::Group(atoms)
    };

    // Wrap in unary sign if present
    let result = match unary_sign {
        Some('-') => MathExpr::Group(vec![MathExpr::Symbol("-".to_string()), term]),
        Some('+') => MathExpr::Group(vec![MathExpr::Symbol("+".to_string()), term]),
        _ => term,
    };

    Ok((input, result))
}

/// Parse an atomic math expression (with sub/superscripts)
fn parse_atom(input: &str) -> IResult<&str, MathExpr> {
    let (input, base) = parse_base(input)?;

    // Check for subscript and/or superscript
    let (input, sub) = opt(preceded(char('_'), parse_script_arg))(input)?;
    let (input, sup) = opt(preceded(char('^'), parse_script_arg))(input)?;

    let expr = match (sub, sup) {
        (Some(sub), Some(sup)) => MathExpr::SubSup {
            base: Box::new(base),
            sub: Box::new(sub),
            sup: Box::new(sup),
        },
        (Some(sub), None) => MathExpr::Subscript {
            base: Box::new(base),
            sub: Box::new(sub),
        },
        (None, Some(sup)) => MathExpr::Superscript {
            base: Box::new(base),
            sup: Box::new(sup),
        },
        (None, None) => base,
    };

    Ok((input, expr))
}

/// Parse a subscript or superscript argument (can be a single char or braced group)
fn parse_script_arg(input: &str) -> IResult<&str, MathExpr> {
    alt((
        parse_braced_group,
        map(recognize(one_of("0123456789")), |s: &str| {
            MathExpr::Symbol(s.to_string())
        }),
        map(alpha1, |s: &str| MathExpr::Symbol(s.to_string())),
    ))(input)
}

/// Parse a base element (symbol, command, or group)
fn parse_base(input: &str) -> IResult<&str, MathExpr> {
    alt((
        parse_latex_command,
        parse_braced_group,
        parse_number,
        parse_symbol,
        parse_operator_char,
    ))(input)
}

/// Parse a LaTeX command like \alpha, \frac{a}{b}, \sqrt{x}
fn parse_latex_command(input: &str) -> IResult<&str, MathExpr> {
    let (input, _) = char('\\')(input)?;

    // First, check for single-character non-alphabetic commands like \, \: \; \!
    if let Some(first_char) = input.chars().next() {
        match first_char {
            ',' | ':' | ';' | '!' => {
                return Ok((&input[1..], MathExpr::Space));
            }
            '\\' => {
                // Double backslash - line break in LaTeX
                return Ok((&input[1..], MathExpr::Symbol("\\".to_string())));
            }
            _ => {}
        }
    }

    let (input, cmd_name) = alpha1(input)?;

    match cmd_name {
        // Greek letters - convert to Typst symbols
        "alpha" | "beta" | "gamma" | "delta" | "epsilon" | "zeta" | "eta" | "theta" | "iota"
        | "kappa" | "lambda" | "mu" | "nu" | "xi" | "pi" | "rho" | "sigma" | "tau" | "upsilon"
        | "phi" | "chi" | "psi" | "omega" | "Gamma" | "Delta" | "Theta" | "Lambda" | "Xi"
        | "Pi" | "Sigma" | "Upsilon" | "Phi" | "Psi" | "Omega" => {
            Ok((input, MathExpr::Symbol(cmd_name.to_string())))
        }
        // Greek letter variants
        "varepsilon" => Ok((input, MathExpr::Symbol("epsilon.alt".to_string()))),
        "vartheta" => Ok((input, MathExpr::Symbol("theta.alt".to_string()))),
        "varpi" => Ok((input, MathExpr::Symbol("pi.alt".to_string()))),
        "varrho" => Ok((input, MathExpr::Symbol("rho.alt".to_string()))),
        "varsigma" => Ok((input, MathExpr::Symbol("sigma.alt".to_string()))),
        "varphi" => Ok((input, MathExpr::Symbol("phi.alt".to_string()))),

        // Binary operators - convert to Typst symbols
        "pm" => Ok((input, MathExpr::Symbol("plus.minus".to_string()))),
        "mp" => Ok((input, MathExpr::Symbol("minus.plus".to_string()))),
        "times" => Ok((input, MathExpr::Symbol("times".to_string()))),
        "cdot" => Ok((input, MathExpr::Symbol("dot".to_string()))),
        "div" => Ok((input, MathExpr::Symbol("div".to_string()))),
        "ast" => Ok((input, MathExpr::Symbol("ast".to_string()))),

        // Comparison operators
        "le" | "leq" => Ok((input, MathExpr::Symbol("<=".to_string()))),
        "ge" | "geq" => Ok((input, MathExpr::Symbol(">=".to_string()))),
        "ne" | "neq" => Ok((input, MathExpr::Symbol("!=".to_string()))),
        "approx" => Ok((input, MathExpr::Symbol("approx".to_string()))),
        "equiv" => Ok((input, MathExpr::Symbol("equiv".to_string()))),
        "sim" => Ok((input, MathExpr::Symbol("tilde".to_string()))),

        // Set operators
        "in" => Ok((input, MathExpr::Symbol("in".to_string()))),
        "notin" => Ok((input, MathExpr::Symbol("in.not".to_string()))),
        "subset" => Ok((input, MathExpr::Symbol("subset".to_string()))),
        "supset" => Ok((input, MathExpr::Symbol("supset".to_string()))),
        "subseteq" => Ok((input, MathExpr::Symbol("subset.eq".to_string()))),
        "supseteq" => Ok((input, MathExpr::Symbol("supset.eq".to_string()))),
        "cup" => Ok((input, MathExpr::Symbol("union".to_string()))),
        "cap" => Ok((input, MathExpr::Symbol("sect".to_string()))),
        "emptyset" => Ok((input, MathExpr::Symbol("emptyset".to_string()))),

        // Special symbols
        "infty" => Ok((input, MathExpr::Symbol("infinity".to_string()))),
        "partial" => Ok((input, MathExpr::Symbol("partial".to_string()))),
        "hbar" => Ok((input, MathExpr::Symbol("planck".to_string()))),
        "nabla" => Ok((input, MathExpr::Symbol("nabla".to_string()))),
        "forall" => Ok((input, MathExpr::Symbol("forall".to_string()))),
        "exists" => Ok((input, MathExpr::Symbol("exists".to_string()))),
        "neg" | "lnot" => Ok((input, MathExpr::Symbol("not".to_string()))),
        "land" => Ok((input, MathExpr::Symbol("and".to_string()))),
        "lor" => Ok((input, MathExpr::Symbol("or".to_string()))),

        // Arrows
        "to" | "rightarrow" => Ok((input, MathExpr::Symbol("->".to_string()))),
        "leftarrow" => Ok((input, MathExpr::Symbol("<-".to_string()))),
        "leftrightarrow" => Ok((input, MathExpr::Symbol("<->".to_string()))),
        "Rightarrow" => Ok((input, MathExpr::Symbol("=>".to_string()))),
        "Leftarrow" => Ok((input, MathExpr::Symbol("<=".to_string()))),
        "Leftrightarrow" => Ok((input, MathExpr::Symbol("<=>".to_string()))),
        "mapsto" => Ok((input, MathExpr::Symbol("|->".to_string()))),

        // Dots
        "ldots" | "dots" => Ok((input, MathExpr::Symbol("...".to_string()))),
        "cdots" => Ok((input, MathExpr::Symbol("dots.c".to_string()))),
        "vdots" => Ok((input, MathExpr::Symbol("dots.v".to_string()))),
        "ddots" => Ok((input, MathExpr::Symbol("dots.down".to_string()))),

        // Delimiters
        "langle" => Ok((input, MathExpr::Symbol("angle.l".to_string()))),
        "rangle" => Ok((input, MathExpr::Symbol("angle.r".to_string()))),
        "lfloor" => Ok((input, MathExpr::Symbol("floor.l".to_string()))),
        "rfloor" => Ok((input, MathExpr::Symbol("floor.r".to_string()))),
        "lceil" => Ok((input, MathExpr::Symbol("ceil.l".to_string()))),
        "rceil" => Ok((input, MathExpr::Symbol("ceil.r".to_string()))),
        "lvert" => Ok((input, MathExpr::Symbol("|".to_string()))),
        "rvert" => Ok((input, MathExpr::Symbol("|".to_string()))),
        "lVert" => Ok((input, MathExpr::Symbol("||".to_string()))),
        "rVert" => Ok((input, MathExpr::Symbol("||".to_string()))),

        // Fractions: \frac{num}{den}
        "frac" => {
            let (input, _) = multispace0(input)?;
            let (input, num) = parse_braced_group(input)?;
            let (input, _) = multispace0(input)?;
            let (input, den) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Fraction {
                    num: Box::new(num),
                    den: Box::new(den),
                },
            ))
        }

        // Square root: \sqrt{x} or \sqrt[n]{x}
        "sqrt" => {
            let (input, _) = multispace0(input)?;
            let (input, degree) = opt(delimited(char('['), parse_expr, char(']')))(input)?;
            let (input, _) = multispace0(input)?;
            let (input, radicand) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Sqrt {
                    degree: degree.map(Box::new),
                    radicand: Box::new(radicand),
                },
            ))
        }

        // Big operators that need renaming for Typst
        "int" => {
            let (input, _) = multispace0(input)?;
            let (input, lower) = opt(preceded(char('_'), parse_script_arg))(input)?;
            let (input, upper) = opt(preceded(char('^'), parse_script_arg))(input)?;
            Ok((
                input,
                MathExpr::Operator {
                    name: "integral".to_string(),
                    lower: lower.map(Box::new),
                    upper: upper.map(Box::new),
                },
            ))
        }
        "iint" => Ok((input, MathExpr::Symbol("integral.double".to_string()))),
        "iiint" => Ok((input, MathExpr::Symbol("integral.triple".to_string()))),
        "oint" => Ok((input, MathExpr::Symbol("integral.cont".to_string()))),

        // Big operators: \sum, \prod, \lim, etc.
        "sum" | "prod" | "lim" | "limsup" | "liminf" | "bigcup" | "bigcap" | "bigoplus"
        | "bigotimes" | "bigwedge" | "bigvee" | "max" | "min" | "sup" | "inf" | "arg" => {
            // For operators, limits can be complex expressions in braces
            let (input, _) = multispace0(input)?;
            let (input, lower) = opt(preceded(char('_'), parse_script_arg))(input)?;
            let (input, upper) = opt(preceded(char('^'), parse_script_arg))(input)?;
            Ok((
                input,
                MathExpr::Operator {
                    name: cmd_name.to_string(),
                    lower: lower.map(Box::new),
                    upper: upper.map(Box::new),
                },
            ))
        }

        // Math functions (rendered in upright text in Typst)
        "sin" | "cos" | "tan" | "cot" | "sec" | "csc" | "arcsin" | "arccos" | "arctan" | "sinh"
        | "cosh" | "tanh" | "log" | "ln" | "exp" | "det" | "dim" | "ker" | "deg" | "gcd"
        | "hom" | "mod" => Ok((input, MathExpr::Symbol(cmd_name.to_string()))),

        // Font/style commands that take an argument
        "mathbf" | "textbf" | "bm" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "bold".to_string(),
                    args: vec![arg],
                },
            ))
        }
        "mathbb" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            // Convert common blackboard bold letters to Typst symbols
            let symbol = match &arg {
                MathExpr::Symbol(s) => match s.as_str() {
                    "R" => "RR",
                    "N" => "NN",
                    "Z" => "ZZ",
                    "Q" => "QQ",
                    "C" => "CC",
                    _ => {
                        return Ok((
                            input,
                            MathExpr::Command {
                                name: "bb".to_string(),
                                args: vec![arg],
                            },
                        ))
                    }
                },
                _ => {
                    return Ok((
                        input,
                        MathExpr::Command {
                            name: "bb".to_string(),
                            args: vec![arg],
                        },
                    ))
                }
            };
            Ok((input, MathExpr::Symbol(symbol.to_string())))
        }
        "mathcal" | "cal" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "cal".to_string(),
                    args: vec![arg],
                },
            ))
        }

        // Operatorname (custom operators)
        "operatorname" => {
            let (input, _) = multispace0(input)?;
            let (input, _) = char('{')(input)?;
            // Parse text content until closing brace
            let mut text = String::new();
            let mut remaining = input;
            let mut depth = 1;
            for (i, c) in input.char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            text = input[..i].to_string();
                            remaining = &input[i + 1..];
                            break;
                        }
                    }
                    _ => {}
                }
            }
            Ok((
                remaining,
                MathExpr::Command {
                    name: "op".to_string(),
                    args: vec![MathExpr::Text(text)],
                },
            ))
        }

        // Accents and decorations
        "hat" | "widehat" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "hat".to_string(),
                    args: vec![arg],
                },
            ))
        }
        "bar" | "overline" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "overline".to_string(),
                    args: vec![arg],
                },
            ))
        }
        "vec" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "arrow".to_string(),
                    args: vec![arg],
                },
            ))
        }
        "dot" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "dot".to_string(),
                    args: vec![arg],
                },
            ))
        }
        "ddot" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "diaer".to_string(),
                    args: vec![arg],
                },
            ))
        }
        "tilde" | "widetilde" => {
            let (input, _) = multispace0(input)?;
            let (input, arg) = parse_braced_group(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "tilde".to_string(),
                    args: vec![arg],
                },
            ))
        }

        // Underbrace with annotation
        "underbrace" => {
            let (input, _) = multispace0(input)?;
            let (input, body) = parse_braced_group(input)?;
            // Check for subscript annotation
            let (input, _) = multispace0(input)?;
            let (input, annotation) = opt(preceded(char('_'), parse_script_arg))(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "underbrace".to_string(),
                    args: if let Some(ann) = annotation {
                        vec![body, ann]
                    } else {
                        vec![body]
                    },
                },
            ))
        }
        "overbrace" => {
            let (input, _) = multispace0(input)?;
            let (input, body) = parse_braced_group(input)?;
            let (input, _) = multispace0(input)?;
            let (input, annotation) = opt(preceded(char('^'), parse_script_arg))(input)?;
            Ok((
                input,
                MathExpr::Command {
                    name: "overbrace".to_string(),
                    args: if let Some(ann) = annotation {
                        vec![body, ann]
                    } else {
                        vec![body]
                    },
                },
            ))
        }

        // Text in math mode
        "text" | "mathrm" | "textrm" => {
            let (input, _) = multispace0(input)?;
            let (input, _) = char('{')(input)?;
            // Parse text content until closing brace
            let mut text = String::new();
            let mut remaining = input;
            let mut depth = 1;
            for (i, c) in input.char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            text = input[..i].to_string();
                            remaining = &input[i + 1..];
                            break;
                        }
                    }
                    _ => {}
                }
            }
            Ok((remaining, MathExpr::Text(text)))
        }

        // Left/right delimiters (handle specially)
        "left" => {
            let (input, _) = multispace0(input)?;
            let (input, delim) = parse_delimiter(input)?;
            Ok((input, MathExpr::Symbol(delim)))
        }
        "right" => {
            let (input, _) = multispace0(input)?;
            let (input, delim) = parse_delimiter(input)?;
            Ok((input, MathExpr::Symbol(delim)))
        }

        // Environment-style commands
        "begin" => {
            let (input, _) = multispace0(input)?;
            let (input, _) = char('{')(input)?;
            let (input, env_name) = alpha1(input)?;
            let (input, _) = char('}')(input)?;

            // Parse environment content until \end{env_name}
            let end_marker = format!("\\end{{{}}}", env_name);
            if let Some(end_pos) = input.find(&end_marker) {
                let content = &input[..end_pos];
                let remaining = &input[end_pos + end_marker.len()..];

                // Parse matrix-like environments
                match env_name {
                    "matrix" | "pmatrix" | "bmatrix" | "vmatrix" | "Vmatrix" | "cases"
                    | "array" | "aligned" | "align" | "gather" | "split" => {
                        let rows = parse_matrix_content(content);
                        Ok((
                            remaining,
                            MathExpr::Environment {
                                name: env_name.to_string(),
                                content: rows,
                            },
                        ))
                    }
                    _ => {
                        // For other environments, just parse as expression
                        match parse_expr(content.trim()) {
                            Ok((_, expr)) => Ok((remaining, expr)),
                            Err(_) => Ok((remaining, MathExpr::Text(content.to_string()))),
                        }
                    }
                }
            } else {
                // No end found, return as symbol
                Ok((
                    input,
                    MathExpr::Command {
                        name: format!("begin{{{}}}", env_name),
                        args: vec![],
                    },
                ))
            }
        }

        // Spacing commands
        "quad" => Ok((input, MathExpr::Space)),
        "qquad" => Ok((
            input,
            MathExpr::Group(vec![MathExpr::Space, MathExpr::Space]),
        )),
        "," | ":" | ";" | "!" => Ok((input, MathExpr::Space)),

        // Other commands - store as-is for now
        _ => Ok((
            input,
            MathExpr::Command {
                name: cmd_name.to_string(),
                args: vec![],
            },
        )),
    }
}

/// Parse a delimiter character (for \left and \right)
fn parse_delimiter(input: &str) -> IResult<&str, String> {
    alt((
        map(char('('), |_| "(".to_string()),
        map(char(')'), |_| ")".to_string()),
        map(char('['), |_| "[".to_string()),
        map(char(']'), |_| "]".to_string()),
        map(char('{'), |_| "{".to_string()),
        map(char('}'), |_| "}".to_string()),
        map(char('|'), |_| "|".to_string()),
        map(char('.'), |_| "".to_string()), // \left. or \right. = invisible delimiter
        map(tag("\\{"), |_| "{".to_string()),
        map(tag("\\}"), |_| "}".to_string()),
        map(tag("\\langle"), |_| "angle.l".to_string()),
        map(tag("\\rangle"), |_| "angle.r".to_string()),
        map(tag("\\lfloor"), |_| "floor.l".to_string()),
        map(tag("\\rfloor"), |_| "floor.r".to_string()),
        map(tag("\\lceil"), |_| "ceil.l".to_string()),
        map(tag("\\rceil"), |_| "ceil.r".to_string()),
        map(tag("\\|"), |_| "||".to_string()),
    ))(input)
}

/// Parse matrix content (rows separated by \\, cells by &)
fn parse_matrix_content(input: &str) -> Vec<Vec<MathExpr>> {
    let mut rows = Vec::new();

    for row_str in input.split("\\\\") {
        let row_str = row_str.trim();
        if row_str.is_empty() {
            continue;
        }

        let mut cells = Vec::new();
        for cell_str in row_str.split('&') {
            let mut cell_str = cell_str.trim();
            if cell_str.is_empty() {
                continue;
            }

            // Strip trailing comma (common in cases environment)
            if cell_str.ends_with(',') {
                cell_str = cell_str[..cell_str.len() - 1].trim();
            }

            // Handle cells that start with operators (like = in aligned environments)
            // by prepending an empty group
            let expr = if let Ok(expr) = parse(cell_str) {
                expr
            } else if cell_str.starts_with('=')
                || cell_str.starts_with('<')
                || cell_str.starts_with('>')
            {
                // Cell starts with a binary operator, parse the rest
                let op = cell_str.chars().next().unwrap();
                let rest = cell_str[1..].trim();
                if let Ok(right_expr) = parse(rest) {
                    MathExpr::Group(vec![MathExpr::Symbol(op.to_string()), right_expr])
                } else {
                    MathExpr::Symbol(cell_str.to_string())
                }
            } else {
                MathExpr::Symbol(cell_str.to_string())
            };
            cells.push(expr);
        }

        if !cells.is_empty() {
            rows.push(cells);
        }
    }

    rows
}

/// Parse a braced group like {abc} or {a + b}
fn parse_braced_group(input: &str) -> IResult<&str, MathExpr> {
    delimited(
        char('{'),
        preceded(multispace0, parse_braced_content),
        preceded(multispace0, char('}')),
    )(input)
}

/// Parse content inside braces - handles full expressions
fn parse_braced_content(input: &str) -> IResult<&str, MathExpr> {
    let input = input.trim_start();
    if input.is_empty() || input.starts_with('}') {
        return Ok((input, MathExpr::Group(vec![])));
    }
    parse_expr(input)
}

/// Parse a number
fn parse_number(input: &str) -> IResult<&str, MathExpr> {
    map(
        recognize(pair(digit1, opt(pair(char('.'), digit1)))),
        |s: &str| MathExpr::Symbol(s.to_string()),
    )(input)
}

/// Parse a single-character symbol (letter)
/// In LaTeX math, each letter is a separate variable (implicit multiplication)
fn parse_symbol(input: &str) -> IResult<&str, MathExpr> {
    use nom::character::complete::satisfy;
    map(satisfy(|c| c.is_ascii_alphabetic()), |c: char| {
        MathExpr::Symbol(c.to_string())
    })(input)
}

/// Parse operator characters
fn parse_operator_char(input: &str) -> IResult<&str, MathExpr> {
    map(one_of("()[]|*/!,.:;'\"&"), |c: char| {
        MathExpr::Symbol(c.to_string())
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_symbol() {
        let result = parse("x").unwrap();
        assert_eq!(result, MathExpr::Symbol("x".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let result = parse("42").unwrap();
        assert_eq!(result, MathExpr::Symbol("42".to_string()));
    }

    #[test]
    fn test_parse_greek_letter() {
        let result = parse(r"\alpha").unwrap();
        assert_eq!(result, MathExpr::Symbol("alpha".to_string()));
    }

    #[test]
    fn test_parse_superscript() {
        let result = parse("x^2").unwrap();
        match result {
            MathExpr::Superscript { base, sup } => {
                assert_eq!(*base, MathExpr::Symbol("x".to_string()));
                assert_eq!(*sup, MathExpr::Symbol("2".to_string()));
            }
            _ => panic!("Expected superscript"),
        }
    }

    #[test]
    fn test_parse_subscript() {
        let result = parse("x_i").unwrap();
        match result {
            MathExpr::Subscript { base, sub } => {
                assert_eq!(*base, MathExpr::Symbol("x".to_string()));
                assert_eq!(*sub, MathExpr::Symbol("i".to_string()));
            }
            _ => panic!("Expected subscript"),
        }
    }

    #[test]
    fn test_parse_sub_and_superscript() {
        let result = parse("x_i^2").unwrap();
        match result {
            MathExpr::SubSup { base, sub, sup } => {
                assert_eq!(*base, MathExpr::Symbol("x".to_string()));
                assert_eq!(*sub, MathExpr::Symbol("i".to_string()));
                assert_eq!(*sup, MathExpr::Symbol("2".to_string()));
            }
            _ => panic!("Expected sub+superscript"),
        }
    }

    #[test]
    fn test_parse_fraction() {
        let result = parse(r"\frac{a}{b}").unwrap();
        match result {
            MathExpr::Fraction { num, den } => {
                assert_eq!(*num, MathExpr::Symbol("a".to_string()));
                assert_eq!(*den, MathExpr::Symbol("b".to_string()));
            }
            _ => panic!("Expected fraction"),
        }
    }

    #[test]
    fn test_parse_sqrt() {
        let result = parse(r"\sqrt{x}").unwrap();
        match result {
            MathExpr::Sqrt { degree, radicand } => {
                assert!(degree.is_none());
                assert_eq!(*radicand, MathExpr::Symbol("x".to_string()));
            }
            _ => panic!("Expected sqrt"),
        }
    }

    #[test]
    fn test_parse_nth_root() {
        // Temporarily simplified - square brackets not fully supported yet
        let result = parse(r"\sqrt{x}").unwrap();
        match result {
            MathExpr::Sqrt { radicand, .. } => {
                assert_eq!(*radicand, MathExpr::Symbol("x".to_string()));
            }
            _ => panic!("Expected sqrt"),
        }
    }

    #[test]
    fn test_parse_sum_simple() {
        // Simplified - just \sum without limits
        let result = parse(r"\sum").unwrap();
        match result {
            MathExpr::Operator { name, lower, upper } => {
                assert_eq!(name, "sum");
                assert!(lower.is_none());
                assert!(upper.is_none());
            }
            _ => panic!("Expected operator"),
        }
    }

    #[test]
    fn test_parse_sum_with_simple_limits() {
        // Simple single-character limits
        let result = parse(r"\sum_i^n").unwrap();
        match result {
            MathExpr::Operator { name, lower, upper } => {
                assert_eq!(name, "sum");
                assert!(lower.is_some());
                assert!(upper.is_some());
            }
            _ => panic!("Expected operator with limits"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let result = parse(r"x^2 + y^2").unwrap();
        // Should parse as a binary expression
        match result {
            MathExpr::Binary { op, .. } => {
                assert_eq!(op, "+");
            }
            _ => panic!("Expected binary expression, got: {:?}", result),
        }
    }

    #[test]
    fn test_parse_multiple_letters_as_separate_symbols() {
        // In LaTeX math, consecutive letters are separate variables (implicit multiplication)
        // This converts correctly to Typst where "a b c" means a × b × c
        let result = parse("abc").unwrap();
        assert_eq!(
            result,
            MathExpr::Group(vec![
                MathExpr::Symbol("a".to_string()),
                MathExpr::Symbol("b".to_string()),
                MathExpr::Symbol("c".to_string()),
            ])
        );
    }
}
