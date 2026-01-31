use latex2typst::convert_markdown;

fn main() {
    let markdown = r#"# Math Examples

## Inline Math

The famous equation: $E = mc^2$

Pythagorean theorem: $a^2 + b^2 = c^2$

Greek letters: $\alpha$, $\beta$, $\gamma$

## Display Math

Quadratic formula:

$$x = \frac{-b}{2a}$$

Summation:

$$\sum_{i=1}^{n} i$$

Square root: $\sqrt{x}$

Subscripts: $x_i$ and superscripts: $x^2$
"#;

    println!("=== Input (Markdown with LaTeX Math) ===");
    println!("{}", markdown);
    println!("\n=== Output (Typst) ===");

    match convert_markdown(markdown) {
        Ok(typst) => println!("{}", typst),
        Err(e) => eprintln!("Error: {}", e),
    }
}
