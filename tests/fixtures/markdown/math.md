This example demonstrates a variety of complex LaTeX features, including multi-line aligned equations, matrices, Greek symbols, and specialized mathematical notation, all rendered within Markdown.

---

### 1. The General Relativity Field Equation
This equation uses subscripts, superscripts, and Greek letters to describe the fundamental interaction of gravitation.

$$R_{\mu \nu} - \frac{1}{2}R g_{\mu \nu} + \Lambda g_{\mu \nu} = \frac{8 \pi G}{c^4} T_{\mu \nu}$$

---

### 2. Multi-line Aligned Derivation
Using the `aligned` environment allows for beautiful step-by-step proofs where the equals signs are perfectly vertical.

$$
\begin{aligned}
\nabla \cdot \mathbf{E} &= \frac{\rho}{\varepsilon_0} \\
\nabla \cdot \mathbf{B} &= 0 \\
\nabla \times \mathbf{E} &= -\frac{\partial \mathbf{B}}{\partial t} \\
\nabla \times \mathbf{B} &= \mu_0 \left( \mathbf{J} + \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t} \right)
\end{aligned}
$$

---

### 3. Complex Matrix Operations
This example shows a transition matrix for a Markov Chain, demonstrating the use of fractions within a matrix and specialized brackets.

$$
P = \begin{pmatrix}
1 - q & q & 0 \\
p & 1 - p - q & q \\
0 & p & 1 - p
\end{pmatrix}
\times
\left[
\frac{\sum_{i=1}^{n} X_i}{\sqrt{\operatorname{Var}(\hat{\beta})}}
\right]
$$

---

### 4. Calculus: Integration and Limits
A complex definite integral involving trigonometric functions and limits.

$$
\int_{0}^{\infty} \frac{\sin(x)}{x} dx = \lim_{t \to \infty} \left( \sum_{k=1}^{n} \frac{(-1)^{k-1} t^{2k-1}}{(2k-1)!(2k-1)} \right) = \frac{\pi}{2}
$$

---

### 5. Case Functions and Piecewise Logic
The definition of the Dirac Delta function or a complex piecewise system:

$$
\delta(x) = \begin{cases}
+\infty, & x = 0 \\
0, & x \neq 0
\end{cases} \quad \text{subject to} \quad \int_{-\infty}^{\infty} \delta(x) \, dx = 1
$$

---

### 6. The "Master" Equation (Combination)
Combining fractions, square roots, sums, and indices in a single line:

$$
\Psi(\mathbf{r}, t) = \underbrace{\frac{1}{\sqrt{(2\pi\hbar)^3}}}_{\text{Normalization}} \int_{\mathbb{R}^3} \phi(\mathbf{p}) \exp \left[ \frac{i}{\hbar} (\mathbf{p} \cdot \mathbf{r} - Et) \right] d^3\mathbf{p}
$$
