This example demonstrates a variety of complex LaTeX features, including multi-line aligned equations, matrices, Greek symbols, and specialized mathematical notation, all rendered within Markdown.

#line(length: 100%)

=== 1. The General Relativity Field Equation

This equation uses subscripts, superscripts, and Greek letters to describe the fundamental interaction of gravitation.

$ R_(mu nu) - 1/2 R g_(mu nu) + Lambda g_(mu nu) = frac(8 pi G, c^4) T_(mu nu) $

#line(length: 100%)

=== 2. Multi-line Aligned Derivation

Using the `aligned` environment allows for beautiful step-by-step proofs where the equals signs are perfectly vertical.

$ nabla dot bold(E) &= frac(rho, epsilon.alt_0) \
  nabla dot bold(B) &= 0 \
  nabla times bold(E) &= -frac(partial bold(B), partial t) \
  nabla times bold(B) &= mu_0 ( bold(J) + epsilon.alt_0 frac(partial bold(E), partial t) ) $

#line(length: 100%)

=== 3. Complex Matrix Operations

This example shows a transition matrix for a Markov Chain, demonstrating the use of fractions within a matrix and specialized brackets.

$ P = mat(1 - q, q, 0; p, 1 - p - q, q; 0, p, 1 - p) times [ frac(sum_(i = 1)^n X_i, sqrt(op("Var") ( hat(beta) ))) ] $

#line(length: 100%)

=== 4. Calculus: Integration and Limits

A complex definite integral involving trigonometric functions and limits.

$ integral_0^infinity frac(sin ( x ), x) d x = lim_(t -> infinity) ( sum_(k = 1)^n frac(( - 1 )^(k - 1) t^(2 k - 1), ( 2 k - 1 ) ! ( 2 k - 1 )) ) = pi/2 $

#line(length: 100%)

=== 5. Case Functions and Piecewise Logic

The definition of the Dirac Delta function or a complex piecewise system:

$ delta ( x ) = cases(
  + infinity "if" x = 0,
  0 "if" x != 0
)   "subject to"   integral_(-infinity)^infinity delta ( x )   d x = 1 $

#line(length: 100%)

=== 6. The "Master" Equation (Combination)

Combining fractions, square roots, sums, and indices in a single line:

$ Psi ( bold(r) , t ) = underbrace(frac(1, sqrt(( 2 pi planck )^3)), "Normalization") integral_(RR^3) phi ( bold(p) ) exp [ i/planck ( bold(p) dot bold(r) - E t ) ] d^3 bold(p) $
