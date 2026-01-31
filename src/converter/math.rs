//! Convert LaTeX math expressions to Typst math syntax

use crate::ast::math::MathExpr;
use crate::error::{Error, Result};

/// Convert a MathExpr to Typst math syntax
pub fn render(expr: &MathExpr) -> Result<String> {
    let mut renderer = MathRenderer::new();
    renderer.render_expr(expr)?;
    Ok(renderer.output)
}

struct MathRenderer {
    output: String,
}

impl MathRenderer {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn render_expr(&mut self, expr: &MathExpr) -> Result<()> {
        match expr {
            MathExpr::Symbol(s) => {
                self.output.push_str(s);
            }

            MathExpr::Command { name, args } => {
                // Handle known commands with special Typst syntax
                match name.as_str() {
                    "bold" => {
                        self.output.push_str("bold(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "hat" => {
                        self.output.push_str("hat(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "overline" => {
                        self.output.push_str("overline(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "arrow" => {
                        // Vector arrow
                        self.output.push_str("arrow(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "dot" => {
                        // Dot accent
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                            self.output.push_str("̇"); // Unicode combining dot above
                        }
                    }
                    "diaer" => {
                        // Double dot (diaeresis)
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                            self.output.push_str("̈"); // Unicode combining diaeresis
                        }
                    }
                    "tilde" => {
                        self.output.push_str("tilde(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "cal" => {
                        self.output.push_str("cal(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "bb" => {
                        self.output.push_str("bb(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "op" => {
                        self.output.push_str("op(");
                        if let Some(arg) = args.first() {
                            self.render_expr(arg)?;
                        }
                        self.output.push(')');
                    }
                    "underbrace" => {
                        self.output.push_str("underbrace(");
                        if let Some(body) = args.first() {
                            self.render_expr(body)?;
                        }
                        if args.len() > 1 {
                            self.output.push_str(", ");
                            self.render_expr(&args[1])?;
                        }
                        self.output.push(')');
                    }
                    "overbrace" => {
                        self.output.push_str("overbrace(");
                        if let Some(body) = args.first() {
                            self.render_expr(body)?;
                        }
                        if args.len() > 1 {
                            self.output.push_str(", ");
                            self.render_expr(&args[1])?;
                        }
                        self.output.push(')');
                    }
                    _ => {
                        // For unknown commands, output as function call
                        self.output.push_str(name);
                        if !args.is_empty() {
                            self.output.push('(');
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 {
                                    self.output.push_str(", ");
                                }
                                self.render_expr(arg)?;
                            }
                            self.output.push(')');
                        }
                    }
                }
            }

            MathExpr::Subscript { base, sub } => {
                self.render_expr(base)?;
                self.output.push('_');
                self.render_script_arg(sub)?;
            }

            MathExpr::Superscript { base, sup } => {
                self.render_expr(base)?;
                self.output.push('^');
                self.render_script_arg(sup)?;
            }

            MathExpr::SubSup { base, sub, sup } => {
                self.render_expr(base)?;
                self.output.push('_');
                self.render_script_arg(sub)?;
                self.output.push('^');
                self.render_script_arg(sup)?;
            }

            MathExpr::Fraction { num, den } => {
                // Use simple fraction notation (a/b) for simple cases
                // Use frac() function for complex cases
                if num.is_simple() && den.is_simple() {
                    self.render_expr(num)?;
                    self.output.push('/');
                    self.render_expr(den)?;
                } else {
                    self.output.push_str("frac(");
                    self.render_expr(num)?;
                    self.output.push_str(", ");
                    self.render_expr(den)?;
                    self.output.push(')');
                }
            }

            MathExpr::Sqrt { degree, radicand } => {
                if let Some(deg) = degree {
                    // nth root: root(n, x)
                    self.output.push_str("root(");
                    self.render_expr(deg)?;
                    self.output.push_str(", ");
                    self.render_expr(radicand)?;
                    self.output.push(')');
                } else {
                    // square root: sqrt(x)
                    self.output.push_str("sqrt(");
                    self.render_expr(radicand)?;
                    self.output.push(')');
                }
            }

            MathExpr::Group(exprs) => {
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        // Check if previous element was a unary minus - don't add space
                        let prev_is_unary_minus = match &exprs[i - 1] {
                            MathExpr::Symbol(s) => s == "-" && i == 1,
                            _ => false,
                        };
                        if !prev_is_unary_minus {
                            // Add space between elements for proper Typst parsing
                            // Typst interprets adjacent letters as a single variable name
                            self.output.push(' ');
                        }
                    }
                    self.render_expr(expr)?;
                }
            }

            MathExpr::Binary { op, left, right } => {
                self.render_expr(left)?;
                self.output.push(' ');
                self.output.push_str(op);
                self.output.push(' ');
                self.render_expr(right)?;
            }

            MathExpr::Operator { name, lower, upper } => {
                // Convert LaTeX operator names to Typst
                // Most are the same, but without the backslash
                self.output.push_str(name);

                if let Some(lower_expr) = lower {
                    self.output.push('_');
                    self.render_script_arg(lower_expr)?;
                }

                if let Some(upper_expr) = upper {
                    self.output.push('^');
                    self.render_script_arg(upper_expr)?;
                }
            }

            MathExpr::Delimited {
                left,
                content,
                right,
            } => {
                self.output.push_str(left);
                self.render_expr(content)?;
                self.output.push_str(right);
            }

            MathExpr::Environment { name, content } => {
                // Handle environments like matrices
                match name.as_str() {
                    "matrix" => {
                        self.output.push_str("mat(delim: #none, ");
                        self.render_matrix_content(content)?;
                        self.output.push(')');
                    }
                    "pmatrix" => {
                        self.output.push_str("mat(");
                        self.render_matrix_content(content)?;
                        self.output.push(')');
                    }
                    "bmatrix" => {
                        self.output.push_str("mat(delim: \"[\", ");
                        self.render_matrix_content(content)?;
                        self.output.push(')');
                    }
                    "vmatrix" => {
                        self.output.push_str("mat(delim: \"|\", ");
                        self.render_matrix_content(content)?;
                        self.output.push(')');
                    }
                    "Vmatrix" => {
                        self.output.push_str("mat(delim: \"||\", ");
                        self.render_matrix_content(content)?;
                        self.output.push(')');
                    }
                    "cases" => {
                        self.output.push_str("cases(\n");
                        for (i, row) in content.iter().enumerate() {
                            if i > 0 {
                                self.output.push_str(",\n");
                            }
                            self.output.push_str("  ");
                            // Cases: first cell is the value, rest is the condition
                            // Format: value "if" condition
                            if let Some(first) = row.first() {
                                self.render_expr(first)?;
                            }
                            if row.len() > 1 {
                                self.output.push_str(" \"if\" ");
                                for (j, cell) in row.iter().skip(1).enumerate() {
                                    if j > 0 {
                                        self.output.push_str(" ");
                                    }
                                    self.render_expr(cell)?;
                                }
                            }
                        }
                        self.output.push_str("\n)");
                    }
                    "aligned" | "align" | "split" => {
                        // Aligned environment: rows separated by \, columns aligned at &
                        for (i, row) in content.iter().enumerate() {
                            if i > 0 {
                                self.output.push_str(" \\\n  ");
                            }
                            for (j, cell) in row.iter().enumerate() {
                                if j > 0 {
                                    self.output.push_str(" &");
                                }
                                self.render_expr(cell)?;
                            }
                        }
                    }
                    "gather" => {
                        // Gather: centered equations separated by newlines
                        for (i, row) in content.iter().enumerate() {
                            if i > 0 {
                                self.output.push_str(" \\\n  ");
                            }
                            for cell in row.iter() {
                                self.render_expr(cell)?;
                            }
                        }
                    }
                    _ => {
                        return Err(Error::ConversionError(format!(
                            "Unsupported environment: {}",
                            name
                        )));
                    }
                }
            }

            MathExpr::Text(text) => {
                self.output.push('"');
                self.output.push_str(text);
                self.output.push('"');
            }

            MathExpr::Space => {
                self.output.push(' ');
            }
        }
        Ok(())
    }

    /// Render matrix content (rows and cells)
    fn render_matrix_content(&mut self, content: &[Vec<MathExpr>]) -> Result<()> {
        for (i, row) in content.iter().enumerate() {
            if i > 0 {
                self.output.push_str("; ");
            }
            for (j, cell) in row.iter().enumerate() {
                if j > 0 {
                    self.output.push_str(", ");
                }
                self.render_expr(cell)?;
            }
        }
        Ok(())
    }

    /// Render a subscript or superscript argument
    /// Wraps in parentheses if needed
    fn render_script_arg(&mut self, expr: &MathExpr) -> Result<()> {
        match expr {
            MathExpr::Symbol(_) => {
                self.render_expr(expr)?;
            }
            MathExpr::Group(exprs) if exprs.len() == 1 => {
                self.render_expr(&exprs[0])?;
            }
            _ => {
                self.output.push('(');
                self.render_expr(expr)?;
                self.output.push(')');
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_symbol() {
        let expr = MathExpr::Symbol("x".to_string());
        assert_eq!(render(&expr).unwrap(), "x");
    }

    #[test]
    fn test_render_greek() {
        let expr = MathExpr::Symbol("alpha".to_string());
        assert_eq!(render(&expr).unwrap(), "alpha");
    }

    #[test]
    fn test_render_superscript() {
        let expr = MathExpr::Superscript {
            base: Box::new(MathExpr::Symbol("x".to_string())),
            sup: Box::new(MathExpr::Symbol("2".to_string())),
        };
        assert_eq!(render(&expr).unwrap(), "x^2");
    }

    #[test]
    fn test_render_subscript() {
        let expr = MathExpr::Subscript {
            base: Box::new(MathExpr::Symbol("x".to_string())),
            sub: Box::new(MathExpr::Symbol("i".to_string())),
        };
        assert_eq!(render(&expr).unwrap(), "x_i");
    }

    #[test]
    fn test_render_simple_fraction() {
        let expr = MathExpr::Fraction {
            num: Box::new(MathExpr::Symbol("a".to_string())),
            den: Box::new(MathExpr::Symbol("b".to_string())),
        };
        assert_eq!(render(&expr).unwrap(), "a/b");
    }

    #[test]
    fn test_render_complex_fraction() {
        let expr = MathExpr::Fraction {
            num: Box::new(MathExpr::Binary {
                op: "+".to_string(),
                left: Box::new(MathExpr::Symbol("a".to_string())),
                right: Box::new(MathExpr::Symbol("b".to_string())),
            }),
            den: Box::new(MathExpr::Symbol("c".to_string())),
        };
        assert_eq!(render(&expr).unwrap(), "frac(a + b, c)");
    }

    #[test]
    fn test_render_sqrt() {
        let expr = MathExpr::Sqrt {
            degree: None,
            radicand: Box::new(MathExpr::Symbol("x".to_string())),
        };
        assert_eq!(render(&expr).unwrap(), "sqrt(x)");
    }

    #[test]
    fn test_render_sum() {
        let expr = MathExpr::Operator {
            name: "sum".to_string(),
            lower: Some(Box::new(MathExpr::Symbol("i".to_string()))),
            upper: Some(Box::new(MathExpr::Symbol("n".to_string()))),
        };
        assert_eq!(render(&expr).unwrap(), "sum_i^n");
    }

    #[test]
    fn test_render_binary_expr() {
        let expr = MathExpr::Binary {
            op: "+".to_string(),
            left: Box::new(MathExpr::Superscript {
                base: Box::new(MathExpr::Symbol("x".to_string())),
                sup: Box::new(MathExpr::Symbol("2".to_string())),
            }),
            right: Box::new(MathExpr::Superscript {
                base: Box::new(MathExpr::Symbol("y".to_string())),
                sup: Box::new(MathExpr::Symbol("2".to_string())),
            }),
        };
        assert_eq!(render(&expr).unwrap(), "x^2 + y^2");
    }
}
