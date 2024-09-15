# Language Documentation

Note this may change or be innaccurate, depending on when I remeber to update this

## Features

- Variables
  Established with var keyword then replaces with a literal
- Comment
  C-style comments are supported using `//` and `/* */`
- Literal Types
  - Numbers: `f64` inside
  - Boolean: `bool` inside
  - String: Read the tin
  - Nil: Empty variant to represent nothing
- Expressions
  - Grouping: `(` Exppresion `)`; groups expressions
  - Math: Uses `+`, `-`, `*`, `/` to return a Number literal
  - String Adding: `"Hello" + "World"` turns into `"Hello World"`
  - Assignment: `x = literalType`; Prefix with var keyword to make a new variable
  - Comparison: Uses `>`, `>=`, `==`, `<`, `<=` to compare two numerical values
  - Ternary Operator: `x ? y : z`; if x then y else z
- Keywords
  - `if`; Conditionaly Executes Code, tab aligned
  - `else`; Executes some code if the attatched if condition evaluates to false
  - `print`; Prints a varibale or a Literal
  - `var`; Initializes a variable
  - `true`; Literal, evaluates to bool::true
  - `false`; Literal, evaluates to bool::false
  - `nil`; Literal, prints to "NIL", evalutaes to a unit variant under the hood
  - `and`; will return the result of `x && y` (x and y)
  - `or`; will return the result of `x || y` (x or y)
  - `class`; TODO
  - `fun`; TODO
  - `for`; TODO
  - `return`; TODO
  - `super`; TODO
  - `this`; TODO
  - `while`; TODO

## Example

```rlux //Assign a bunch of variables
var _arg = 4/2;
var cat = 0;
var dog = 12*4;
var turt = (13/2);
var tester="strung";
var goolean45 = false;
var vardoolean = true;
var threshold =5;
var adder = 1+0-5;
var tern = true ? 5 : 10;

/* Then print them */
print tern;         //5
print _arg;         //2
print cat;          //0
print dog;          //48
print turt;         //6.5
print tester;       //strung
print goolean45;    //false
print vardoolean;   //true
print threshold;    //5
print adder;        //-4

if (_arg> threshold)
    print "Greater";
else
    print "Lesser";

//This will throw a runtime error then put NIL in the output
print not_found_test;

```
