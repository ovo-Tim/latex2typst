//! Test complex math conversion

use latex2typst::convert_markdown;

fn main() {
    let input = r#"# Mathematical Document

## Inline Math

The Pythagorean theorem states that $a^2 + b^2 = c^2$ for right triangles.

Greek letters: $\alpha$, $\beta$, $\gamma$.

## Display Math

The quadratic formula:

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

Summation notation:

$$\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$$

## More Examples

Square root: $\sqrt{x}$

Subscripts and superscripts: $x_i^2$
"#;

    println!("=== Input (Markdown with LaTeX Math) ===");
    println!("{}", input);

    match convert_markdown(input) {
        Ok(output) => {
            println!("=== Output (Typst) ===");
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
