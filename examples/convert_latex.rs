use latex2typst::convert_latex;

fn main() {
    let latex = r#"\documentclass{article}
\title{My Document}
\author{Jane Smith}

\begin{document}

\section{Introduction}

This is a LaTeX document with \textbf{bold} and \emph{italic} text.

\subsection{Features}

Some inline math: $E = mc^2$

\subsection{Lists}

\begin{itemize}
\item First point
\item Second point
\end{itemize}

\section{Conclusion}

That's all!

\end{document}"#;

    println!("=== Input (LaTeX) ===");
    println!("{}", latex);
    println!("\n=== Output (Typst) ===");

    match convert_latex(latex) {
        Ok(typst) => println!("{}", typst),
        Err(e) => eprintln!("Error: {}", e),
    }
}
