Solve
=====

A simple command line utility to parse mathematical expressions that correctly handles operator precedence, parenthesis and functions.

Usage
------------

To evaluate an expression, simply pass it as arguments:

```bash
  so <expression> [<expression>...]
```

For example, the invoking `so 1+1` will output `2`. Arguments are concatenated when evaluating the expression, thus ignoring spaces in between them. As a result, one could also write `so 1 + 1`. Note that all operations are done on double precission floats (`f64`).

`Solve` supports all basic math **operators**: addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`) and exponentiation (`^`). Operator precedence is maintained while parsing an expression. For example, `so 1+2*3` will evaluate `2*3` before `1` adding it to it.

One can also use **parenthesis** in an expression to change the normal operator precedence. Evaluation of tokens inside parenthesis will take place before all other tokens on the same level are evaluated. For example, `so (1+2)*3` will evaluate the sum before the product.

`Solve` currently recognizes both `pi` and `e` as math constants, and will parse them correctly.

License
-------

Copyright © 2022 Antonio de Haro

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
