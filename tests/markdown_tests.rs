use latex2typst::convert_markdown;

#[test]
fn test_basic_markdown() {
    let input = include_str!("fixtures/markdown/basic.md");
    let expected = include_str!("fixtures/markdown/basic.typ");

    let result = convert_markdown(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_math_markdown() {
    let input = include_str!("fixtures/markdown/math.md");
    let expected = include_str!("fixtures/markdown/math.typ");

    let result = convert_markdown(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_simple_heading() {
    let input = "# Hello World";
    let result = convert_markdown(input).unwrap();
    assert_eq!(result, "= Hello World\n");
}

#[test]
fn test_multiple_headings() {
    let input = "# Level 1\n\n## Level 2\n\n### Level 3";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("= Level 1"));
    assert!(result.contains("== Level 2"));
    assert!(result.contains("=== Level 3"));
}

#[test]
fn test_paragraph_with_formatting() {
    let input = "This is **bold** and this is _italic_.";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("*bold*"));
    assert!(result.contains("_italic_"));
}

#[test]
fn test_unordered_list() {
    let input = "- Item 1\n- Item 2\n- Item 3";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("- Item 1"));
    assert!(result.contains("- Item 2"));
    assert!(result.contains("- Item 3"));
}

#[test]
fn test_ordered_list() {
    let input = "1. First\n2. Second\n3. Third";
    let result = convert_markdown(input).unwrap();
    // Ordered lists in Typst use +
    assert!(result.contains("+ First"));
    assert!(result.contains("+ Second"));
    assert!(result.contains("+ Third"));
}

#[test]
fn test_code_block() {
    let input = "```rust\nfn main() {}\n```";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("```rust"));
    assert!(result.contains("fn main() {}"));
    assert!(result.contains("```"));
}

#[test]
fn test_inline_code() {
    let input = "Some `code` here";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("`code`"));
}

#[test]
fn test_link() {
    let input = "[Click here](https://example.com)";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("#link(\"https://example.com\")[Click here]"));
}

#[test]
fn test_special_character_escaping() {
    let input = "Text with # and * and _ symbols";
    let result = convert_markdown(input).unwrap();
    // These should be escaped in the output
    assert!(result.contains("\\#"));
    assert!(result.contains("\\*"));
    assert!(result.contains("\\_"));
}

#[test]
fn test_empty_document() {
    let input = "";
    let result = convert_markdown(input).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_mixed_content() {
    let input = r#"# Title

A paragraph with **bold**, _italic_, and `code`.

- List item 1
- List item 2

```python
print("hello")
```

Another paragraph with [a link](https://example.com).
"#;

    let result = convert_markdown(input).unwrap();

    // Check all elements are present
    assert!(result.contains("= Title"));
    assert!(result.contains("*bold*"));
    assert!(result.contains("_italic_"));
    assert!(result.contains("`code`"));
    assert!(result.contains("- List item 1"));
    assert!(result.contains("```python"));
    assert!(result.contains("print(\"hello\")"));
    assert!(result.contains("#link(\"https://example.com\")"));
}

// ============ Math Tests ============

#[test]
fn test_inline_math_simple() {
    let input = "The equation $x^2$ is quadratic.";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("$x^2$"));
}

#[test]
fn test_inline_math_greek_letters() {
    let input = r"Greek: $\alpha$, $\beta$, $\gamma$, $\pi$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("$alpha$"));
    assert!(result.contains("$beta$"));
    assert!(result.contains("$gamma$"));
    assert!(result.contains("$pi$"));
}

#[test]
fn test_inline_math_binary_operators() {
    let input = r"$a + b = c$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("$a + b = c$"));
}

#[test]
fn test_inline_math_subscript_superscript() {
    let input = r"$x_i^2$ and $a_{n+1}$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("$x_i^2$"));
    // Complex subscript should use parentheses
    assert!(result.contains("a_(n + 1)"));
}

#[test]
fn test_display_math_fraction() {
    let input = r"$$\frac{a}{b}$$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("$ a/b $") || result.contains("$ frac(a, b) $"));
}

#[test]
fn test_display_math_sqrt() {
    let input = r"$$\sqrt{x}$$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("$ sqrt(x) $"));
}

#[test]
fn test_display_math_sum() {
    let input = r"$$\sum_{i=1}^{n} i$$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("sum_"));
    assert!(result.contains("^n"));
}

#[test]
fn test_display_math_complex_fraction() {
    let input = r"$$\frac{-b}{2a}$$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("frac(-b, 2 a)"));
}

#[test]
fn test_math_pm_operator() {
    let input = r"$a \pm b$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("plus.minus"));
}

#[test]
fn test_math_comparison_operators() {
    let input = r"$a \le b$ and $c \ge d$ and $e \neq f$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("<="));
    assert!(result.contains(">="));
    assert!(result.contains("!="));
}

#[test]
fn test_math_infinity() {
    let input = r"$\lim_{x \to \infty} f(x)$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("lim_"));
    assert!(result.contains("infinity"));
}

#[test]
fn test_math_trig_functions() {
    let input = r"$\sin(x) + \cos(x) = \tan(x)$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("sin"));
    assert!(result.contains("cos"));
    assert!(result.contains("tan"));
}

#[test]
fn test_math_text_in_math() {
    let input = r"$x \text{ if } x > 0$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("\" if \"") || result.contains("\"if\""));
}

#[test]
fn test_quadratic_formula() {
    let input = r"$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$";
    let result = convert_markdown(input).unwrap();
    // Should contain key components
    assert!(result.contains("frac("));
    assert!(result.contains("-b"));
    assert!(result.contains("plus.minus"));
    assert!(result.contains("sqrt("));
    assert!(result.contains("2 a"));
}

#[test]
fn test_math_dots() {
    let input = r"$a_1, a_2, \ldots, a_n$";
    let result = convert_markdown(input).unwrap();
    assert!(result.contains("..."));
}
