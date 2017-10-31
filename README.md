# toylang

## About

toylang is an interpreted programming language created for the purpose of learning about grammar parsing,
abstract syntax trees, and other things related to the implementation of programming languages. It does not
aim to be actually useful.

## Examples

Basic assignment and printing:
````
let some_string = "Hello, world";
println some_string;
````

Fizzbuzz:
````
func print_fizzbuzz(n: num) {
  if n % 15 == 0 {
    println "FizzBuzz";
  } elif n % 3 == 0 {
    println "Fizz";
  } elif n % 5 == 0 {
    println "Buzz";
  } else {
    println n;
  }
}

let i = 1;
while i <= 100 {
  print_fizzbuzz(i);
  i += 1;
}

````

Functions can be [higher-order](https://en.wikipedia.org/wiki/Higher-order_function), so you can do this:
````
func inc(x: num) {
  return x + 1;
}

func twice(f: func(num), x: num) {
  return f(f(x));
}

println twice(inc, 40); // Prints 42
````

## Features

* Primitives (number, string, boolean, array)
* Variable assignment
* Variable printing
* Boolean logic (comparison of numbers/strings)
* `if` / `elif` / `else` statements
* `while` loops
* Interactive REPL

## To-Do
* Read line of input (e.g. Bash's `read` builtin)
* `for` loops
* Pick a better name
