## Testing

I generated the test code coverage reports using a tool called 
[grcov](https://github.com/mozilla/grcov). 

The tool has some strange behavior when it comes to funtion coverage. The percentages, in many cases, are showing 
quite low even though all of the funtions are covered in the tests. This seems to caused by macros and
inside function calls that could potentially fail. As a result, grcov interpretes them as partial hits. 
[Here](https://github.com/mozilla/grcov/issues/476) 
is some discussion about the issue. I've tried a bunch of workarounds, but haven't been able to solve the issue 
yet. The problem is that, in a lot of cases, I'm not able to write a test that would cover the case where the funtion call 
inside the function would fail, while still being able to compile the code. Therefore, from the practical point 
of view, it would be completely unnecessary to do so. Also, correctness is already being tested in multiple 
layers, which in many cases ensures that the inputs for the function calls are as expected.

TLDR: Poor funcion coverage in the coverage report should be not taken too seriously.

The coverage report can be viewed 
[here](https://htmlpreview.github.io/?https://github.com/thiom/tiralab/blob/main/rs-regex/coverage/index.html).
