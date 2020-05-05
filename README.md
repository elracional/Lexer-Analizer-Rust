# Lexer Analizer Rust

##Installation Terminal:
 * curl https://sh.rustup.rs -sSf | sh

# Check the path variable
https://www.rust-lang.org/tools/install

Shows the way of operation of a lexical analyzer, which through regular expressions and state handlers verifies a source code of the BASIC language in Rust.
#### Keywords: Automaton, tokens, Rust, Regex, lexer.

## Programming
### I. Introduction
Lexical analysis or scanning is the process where the stream of characters making up the source program is read from left-to-right and grouped into tokens.
### II. The Basics
Tokens are sequences of characters with a collective meaning. There are usually only a small number of tokens for a programming language: constants (integer, double, char, string, etc.), operators (arithmetic, relational, logical), punctuation, and reserved words.
The lexical analyzer takes a source program as input, and produces a stream of tokens as output. The lexical analyzer might recognize particular instances of tokens such as:
1) 3 or 255 for an integer constant token.
2) "Fred" or "Wilma" for a string constant token
3) numTickets or queue for a variable token.
Such specific instances are called lexemes.
A lexeme is the actual character sequence forming a token, the token is the general class that a lexeme belongs to. Some tokens have exactly one lexeme (e.g., the > character); for others, there are many lexemes (e.g., integer constants).
Illustration 1 Example of Basic language
III. Scanner Implementation 2: Regular Expressions and Finite Automata
A quick detour for some background review and then let’s see how we can generate a scanner building on techniques from automata theory.
### OVERVIEW
We assume that you are well acquainted with regular expressions and all this is old news to you.
* Symbol
an abstract entity that we shall not define formally (such as “point” in geometry). Letters, digits and punctuation are examples of symbols.
* Alphabet
a finite set of symbols out of which we build larger structures. An alphabet is typically
denoted using the Greek sigma Σ, e.g., Σ = {0,1}.
* String
a finite sequence of symbols from a particular alphabet juxtaposed. For example: a, b, c, are symbols and abcb is a string.
* Empty string
denoted ε (or sometimes ∂) is the string consisting of zero symbols.
* Formal language Σ*
the set of all possible strings that can be generated from a given alphabet.
* Regular expressions
rules that define exactly the set of words that are valid tokens in a formal language.
programming languages that are written in the form of sentences. The lexical analyzer divides these syntaxes into a series of tokens, eliminating any blank space in addition to the comments in the source code. In case the lexical analyzer finds a token, which is not considered valid, it generates an error. The lexical analyzer works in close collaboration with the syntax parser because it forms the first part of the compiler and the lexical analyzer is at a later stage. The lexical analyzer will read character streams from the source code, check for legal tokens, and pass the data to the syntax parser when required.
Illustration 2lexical analyzer diagram
### Implementation
The central routine of the scanner is yylex, an integer function that returns a token number, indicating the type (identifier, integer constant, semicolon, etc.), of the next token in the input stream. In addition to the token type, yylex must set the global variables yyline and yycolumn to the line and column number at which that token appears, and – in the case of identifiers and constants –, put the extra information needed, as described above, into the global integer variable yyval.semantic value. Lex will write yylex for you, using the patterns and rules defined in your lex input file (which should be called lexer.l). Your rules must include the code to maintain yyline, yycolumn and yyval.
Error Handling
Your lexical analyzer should recover from all malformed lexemes, as well as such things as string constants that extended across a line boundary or comments that are never terminated. Due date and materials to be handed in The assignment is due Tuesday, Jan 29. You will do a demo at the due day. Test cases named hello.pasc, sieve.pasc, hist.pasc, error.pasc and lexerror.pasc can be found the class web page. A driver file, driver.c, that calls yylex and prints each token with its value as the input is scanned, can also be the class web page.
The rows of the transition table are labeled with the states in which the FSM syntactic analyzer is located and the columns with all the characters or elements that can be used to move to other states. Each cell of the table contains the state in which it is positioned in the FSM, if the character in the corresponding column is read while the FSM is in the state in the corresponding row. A cell that contains an E means that there is no transition in the FSM to any state. The transition table provides a visual aid to make a transition function.
### IV.Conclusion
A lexical analyzer is important for any compiler, regardless of the programming language, since it is the main process through which a source code that you want to compile passes.
### Glossary
1.Rust Rust is a multi-paradigm system programming language focused on safety, especially safe concurrency. Rust is syntactically similar to C++, but is designed to provide better memory safety while maintaining high performance.
2. Automaton: mechanical device that performs some function determined by a predefined set of instructions.
3. Programming language: vocabulary and a set of grammar rules for the instruction of a computer or computing device.
#### BIBLIOGRAPHY
A. Aho, R. Sethi, J. Ullman, Compilers: Principles, Techniques, and Tools. Reading, MA: Addison-Wesley, 1986.
J. Hopcroft, J. Ullman, Introduction to Automata Theory, Languages, and Computation, Reading MA: Addison-Wesley, 1979.
S. Kleene, "Representation of Events in Nerve Nets and Finite Automata," in C. Shannon and J. McCarthy (eds), Automata Studies, Princeton, NJ: Princeton University Press, 1956.
R.L.Wexelblat, History of Programming Languages. London: Academic Press, 1981Flores, J. (2015 de Julio de 23). michelletorres. Obtenido de michelletorres
