## How to run

- [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
- inside **rs-regex** directory, run  
```cargo run "<YOUR REGEX>"```  
- or print instructions by running  
```cargo run -- --help```

NOTE: The regex must be given inside quotes

The following operators are supported

|Operator  |Syntax  | Matches|
--- | --- | --- |
|Union | 0\|1 | "0" or "1"|  
|Star |a* | 0 or more "a"|
|Concatenation | ab | "a" followed by "b"|
|Group | (a\|b)* | 0 or more "a" or "b"|

Empty string (epsilon) is represented by the combination ```()*```. Also, if you want to add a space at the front 
or at the end of the regex, you need to wrap it inside parenthases. So ```"(  )a"``` accepts " a" and ```"b(  )*"``` 
accepts "b", "b " and "b &nbsp; &nbsp; &nbsp;". Spaces between characters are interpreted just as you'd expect, so ```"x y"``` 
accepts "x y", but not "xy".

Here are some more examples:

```"(a|b)(a|b)*"``` accepts any string that only contains characters 'a' and 'b' and does not accept en empty string.  
e.g. "b", "a", "abba" and "babbaabbabababa" are accepted  

```"(Hello)( world|()*)!"``` only accepts the strings "Hello!" and "Hello world!".  

```"(0|(-|()*)(1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*)"``` recognizes all valid integers.  
e.g. "100", "-9999991" and "0" are accepted, whereas "-0", "09", "000001" and "0.1" are rejected  

There is also an option to print the tokens for a given regular expression. This will just print the tokens and 
exit the program

```
cargo run "a(b|c)*" -t

Token(Char, a)
Token(LeftParen, ()
Token(Char, b)
Token(Union, |)
Token(Char, c)
Token(RightParen, ))
Token(Star, *)
```