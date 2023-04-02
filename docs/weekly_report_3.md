# Weekly report, week 3

This week I made a ton of progress with the regex engine. In fact, it's practically complete now.
The program became way more complicated than I originally anticipated. Probably could've made it work 
with a simpler implementation, but I wanted to do it the way I did.

So, here's how it works. The parser asks tokens from the scanner and constructs an AST (abstract syntax tree). 
While constructing the AST, the nodes are already being processed and converted into NFA (nondeterministic finite automaton) 
fragments. The final NFA is then built using these fragments. After that, the NFA is converted into DFA (deterministic 
finite automaton). Finally, this DFA can be used to recognize input strings.

Regex --> Scanner --> <tokens> --> Parser --> AST --> NFA --> DFA

I still need to write some tests and do some final polishing. I might even expand the functionality. We'll see...

## Hours
| date | hours |
--- | --- |
|1.4.| 10h |
|2.4. | 10h |
