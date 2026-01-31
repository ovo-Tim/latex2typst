# LaTeX to Typst Conversion Test

This file contains various examples to test the LaTeX to Typst converter.

## Basic Markdown

This is **bold text** and this is *italic text*.

Here's a [link](https://typst.app) to Typst.

### Lists

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
- Item 3

1. First
2. Second
3. Third

## Inline Math

The famous equation $E = mc^2$ relates energy and mass.

Variables: $x$, $y$, $z$

Subscripts: $x_1$, $x_2$, $x_n$

Superscripts: $x^2$, $y^3$, $e^{-x}$

## Display Math

The quadratic formula:

$$
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$

Summation:

$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$

Integration:

$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

## Greek Letters

Alpha: $\alpha$, Beta: $\beta$, Gamma: $\gamma$, Delta: $\delta$

Uppercase: $\Gamma$, $\Delta$, $\Theta$, $\Lambda$, $\Sigma$, $\Omega$

## Complex Expressions

Matrix notation:

$$
A = \begin{pmatrix}
a_{11} & a_{12} \\
a_{21} & a_{22}
\end{pmatrix}
$$

Fractions and binomials:

$$
\binom{n}{k} = \frac{n!}{k!(n-k)!}
$$

## Special Symbols

Infinity: $\infty$

Set notation: $x \in \mathbb{R}$, $A \subset B$, $\emptyset$

Logic: $\forall x$, $\exists y$, $\land$, $\lor$, $\neg$

Relations: $\leq$, $\geq$, $\neq$, $\approx$

## Limits and Derivatives

Limit:

$$
\lim_{x \to \infty} \frac{1}{x} = 0
$$

Derivative:

$$
\frac{d}{dx} x^n = nx^{n-1}
$$

## Trigonometry

$$
\sin^2(\theta) + \cos^2(\theta) = 1
$$

$$
e^{i\pi} + 1 = 0
$$

## Test Instructions

To test this file:

1. Open this file in VS Code
2. Press `Cmd+Shift+P` (or `Ctrl+Shift+P` on Windows/Linux)
3. Run "LaTeX to Typst: Show Preview" to see the converted output
4. Or run "LaTeX to Typst: Convert and Save as Typst" to save as a `.typ` file

Try selecting individual sections and using "LaTeX to Typst: Convert Selection" to convert just that part.
