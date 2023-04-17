# Regular Expressions Interpreter

## Overview

My plan is to write an interpreter for regular expressions (regex). This will be a command-line app and it will take a regex 
as an initial input. After that, it will read given input strings one by one and tell if the language (defined by the regex) 
recognizes them or not. The syntax of the regex is based on the formal definition of 
[regular expressions](https://en.wikipedia.org/wiki/Regular_expression) 
in their formal definition in 
[formal language theory](https://en.wikipedia.org/wiki/Formal_language).

Regexes will support the following operators:

|Operator  |Syntax  | Matches|
--- | --- | --- |
|Or| 0\|1 | "0" or "1"|  
|Star |a* | 0 or more "a"|
|Concatenation | ab | "a" followed by "b"|
|Group | (a\|b)* | 0 or more "a" or "b"|


## Implementation

I've never looked into implementation of regex engines before, but I have written an interpreter for a programming language recently, 
so I'm very familiar with regular expressions in the context of theory of computation and compiler/interpreter design (used in the scanning phase). 
My initial instict would be to take similar approach to this one and follow a common convention for writing interpeters. 
That is, there will be a scanner (or lexer) that will read input as characters one by one and split it into tokens. These tokens will 
then be passed to the parser which will try to understand some syntactical structures from the input and construct an AST. 
After that, the AST will be coverted to NFA and the NFA will be converted into DFA. These two conversion are based on the techniques 
described in the book called "Introduction to the theory of computation, third edition" by Michael Sipser (pages 54, 66). 
Finally, the graph will be used for trying to see if the language (described by the regex) will recognize the input strings. 

As for data structures, at least graphs, queues, hash tables, hash sets and some basic data structures will be used. 
I would expect the runtime to be around O(n), n being the length of the string to be matched. The more detailed implementation 
of the program will be described in the implementation document.

## Languages

The programming language for the project will be Rust. I can do peer review in many languages, including Java, Python, JS, C and C++.
The documentation and code will all be written in English, but I understand Finnish as well (for peer review).


I am doing bachelor's in CS.
