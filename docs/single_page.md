# Regular Expressions Interpreter

## Overview

This is a single page doc for the purpose of demoing the app... I'm still working on it.

Regexes will support the following operators:

|Operator  |Syntax  | Matches|
--- | --- | --- |
|Union | 0\|1 | "0" or "1"|  
|Star |a* | 0 or more "a"|
|Concatenation | ab | "a" followed by "b"|
|Group | (a\|b)* | 0 or more "a" or "b"|


![](https://github.com/thiom/tiralab/blob/main/docs/img/rs-regex-overview.drawio.png)

### Parser

The parser (src/parser.rs) request tokens from the scanner one by one during the parsing process. 
The job is to take the incoming tokens and put them in some kind of meaningful context. Often times, 
there are a specific set of tokens that the parser expects to encounter. If the incoming token does not 
match a single member of this sets, a parsing error has occured and the parsing process will be 
terminated. A succesfull series of matches will eventually result into the parser recogizing some notable 
syntactical structure. These structures can be described as productions in context-free grammar (CFG). 
For this program, the CFG looks like this:

TODO

Above, tokens are represented in bolded text. The specific techique implemented for parsing is LL(1). The 
first 'L' refers to "Left-to-right", meaning that the input (RE) is read from left-to-right. The second 'L' 
refers to "Leftmost derivarion", meaning that from the right-hand-side of the productions, we are going to 
expand the leftmost derivation first. This results to so called top-down (or recursive descent) parsing, 
which can be visualized in the parse tree example shown below. The number one in LL(1) means that we-re making 
parsing decisions based on only one look-ahead symbol.

![](https://github.com/thiom/tiralab/blob/main/docs/img/parse_tree.drawio.svg)

After a succesfull production, a node (src/ast.rs) will be created. The information that is relevent for any further 
stages of the program is stored in the node and it will be inserted as a part of an abstract syntax tree (AST). The 
AST is a striped-down version of the parse tree. It will only contain the information that is relevant for execution 
in a form that guarantees syntactical correctness and enables easy translation into a corresponding automaton.


### NFA

Nondeterministic finite automaton (srsc/nfa.rs) is finite-state machine that can be used to recognize regular 
languages. This representaion will be constructed from the AST by converting the nodes of the AST into building 
blocks called NFA fragments (src/nfa_fragment.rs) and connecting them to each other in recursive routine 
(src/ast.rs : fn to_fragment). This will result into a single NFA fragment that will then be converted into NFA  
(src/nfa_fragment.rs : fn to_nfa) by reprocessing the state transitions. The process of converting the regular 
expression into an equivalent NFA is based on techniques discribed in the book "Introduction to the Theory of 
Computation, Third Edition, Michael Sipser" on pages 66-69. Here are a few examples of the illustrated conversions:

![](https://github.com/thiom/tiralab/blob/main/docs/img/regex_to_nfa_v5.drawio.svg)


### Sources

- https://en.wikipedia.org/wiki/Regular_expression 
- https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton 
- https://en.wikipedia.org/wiki/Deterministic_finite_automaton 
- https://en.wikipedia.org/wiki/Context-free_grammar 
- Introduction to the Theory of Computation, Third Edition, Michael Sipser 
- [Crafting Interpreters, Robert Nystrom](https://craftinginterpreters.com/) 

