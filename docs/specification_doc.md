# Regular Expressions Interpreter

## Overview
My plan is to write an interpreter for regular expressions. This will be a command-line tool and take a regex and
 a string to be matched as inputs.

Regexes will support the following operators:

?:       a?      matches 0 or 1 "a"
Or:      0|1     matches "0" or "1" 
Star:    a*      matches 0 or more "a"
Plus:    a+      matches 1 or more "a"
Group:   (ab)+   matches 1 or more "ab"
Dot:     .       matches any supported char


## Implementation
The program will be follow a common convention for writing interpeters. It will have a scanner (or lexer) that will read input
as characters one by one and split it into tokens. These tokens will then be passed to the parser which will try to understand some
syntactical structures from the input and construct a graph (usually AST, but here DFA or NFA). As for data structures, 
at least graphs will be used, maybe stacks as well. The graph will be explored with some searching algorithms, possibly DFS.
I'm not quite sure yet about how exactly I'm going to implement this so it's difficult to give more details at this point. This first
week was very busy for me and I didn't have too much time to look into this project yet.
I would expect the runtime to be around O(n), n being the length of the string to be matched.

## Languages
The programming language for the project will be Rust. I can do peer review in many languages, including Java, Python, JS, C and C++.
The documentation and code will all be written in English, but I understand Finnish as well (for peer review).


I am doing bachelor's in CS.
