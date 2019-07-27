# lisp-xp

This is me following along with a neat little Lisp creation described by Will Speak. https://willspeak.me/2019/07/10/lisp-in-two-days-with-rust.html.

Of course I am starting in a half-assed backward manner with the evaluation module because that is the way my brain works.  The hashmapped global environment, mapping functions to the eval fn call via the Value Callable variant based upon the expression input which provides the vector args is really neat stuff.  Ideally,  the evaluator just converts an expression into a value using a hashmap scratchpad for simple symbol bindings to numbers and mapping operators and commands along with requisite args to Callable  closure implementations.