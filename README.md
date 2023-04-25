## Tiralabra 2023
## rs-regex

An iterpreter for 
[regular expressions](https://en.wikipedia.org/wiki/Regular_expression) 
in their formal definition in 
[formal language theory](https://en.wikipedia.org/wiki/Formal_language)


### How to run

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

Empty string (epsilon) is represented by the combination ```()*```

Here are some examples

```"(a|b)(a|b)*"``` accepts any string that only contains characters 'a' and 'b' and does not accept en epmty string.  
e.g. "b", "a", "abba" and "babbaabbabababa" are accepted  
```"(Hello)( World|()*)!"``` only accepts the strings "Hello!" and "Hello World!"  
```"(0|(-|()*)(1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*)"``` recognizes all valid integers  
e.g. "100", "-9999991" and "0" are accepted. "-0" "09" and "000001" are rejected  


### Docs

[Project specification](https://github.com/thiom/tiralab/blob/main/docs/specification_doc.md)

[Testing](https://github.com/thiom/tiralab/blob/main/docs/testing_doc.md)

[Implementation](https://github.com/thiom/tiralab/blob/main/docs/implementation_doc.pdf) 
(contains usage instructions)


### Weekly reports

[Week 1](https://github.com/thiom/tiralab/blob/main/docs/weekly_report_1.md)

[Week 2](https://github.com/thiom/tiralab/blob/main/docs/weekly_report_2.md)

[Week 3](https://github.com/thiom/tiralab/blob/main/docs/weekly_report_3.md)

[Week 4](https://github.com/thiom/tiralab/blob/main/docs/weekly_report_4.md)

[Week 5](https://github.com/thiom/tiralab/blob/main/docs/weekly_report_5.md)
