# Testing

Runtime performance is not really relevant for this program, so there are no tests for that. 
The main focus is on correctness. That is, the program will need to interpret the given 
regex correctly. After that, it has to recognize input strings correctly. Therefore, 
these aspects are the main focus for the tests. Also, there are a lot of conversion 
phases involved when the regex is converted into DFA, so ideally the correctness of 
those conversions would be tested thoroughly as well. I certainly will be be writing 
some tests for these, but I would argue that the end-to-end-style of testing that is 
done in the Regex module (regex.rs) will test the correctness for some of the conversions 
quite sufficiently. This is because incorrect conversions will eventually result to the 
runtime giving false positives or false negatives for the matched strings.

### Coverage

I generated the test code coverage reports using a tool called 
[grcov](https://github.com/mozilla/grcov). 

The tool has some strange behavior when it comes to function coverage. The percentages, in many cases, are showing 
quite low even though all of the funtions are covered in the tests. This seems to caused by macros and
inside function calls that could potentially fail. As a result, grcov interpretes them as partial hits. 
[Here](https://github.com/mozilla/grcov/issues/476) 
is some discussion about the issue. I've tried a bunch of workarounds, but haven't been able to solve the issue 
yet. The problem is that, in a lot of cases, I'm not able to write a test that would cover the case where the funtion call 
inside the function would fail, while still being able to compile the code. Therefore, from the practical point 
of view, it would be completely unnecessary to do so. Also, correctness is already being tested in multiple 
layers, which in many cases ensures that the inputs for the function calls are as expected.

TLDR: Poor function coverage in the coverage report should not be taken too seriously.

The coverage report can be viewed 
[here](https://htmlpreview.github.io/?https://github.com/thiom/tiralab/blob/main/rs-regex/coverage/index.html).


### Organization
The unit tests can be found directly in the source code files of the code that they are testing. The command-line 
interface is not currently being tested. All of the core functionality is run through the Regex module (regex.rs) 
so the end-to-end-style of testing is done there.
