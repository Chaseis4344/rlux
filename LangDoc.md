# Language Documentation

Note this may change or be innaccurate, depending on when I remeber to update this

## Features

### Variables
  Established with var keyword then replaces with a literal
### Comment
  C-style comments are supported using `//` and `/* */`
### Literal Types
  - Numbers: `f64` inside
  - Boolean: `bool` inside
  - String: `String` inside
  - Nil: Empty variant to represent nothing
### Expressions
  - Grouping: `(` Exppresion `)`; groups expressions
  - Math: Uses `+`, `-`, `*`, `/` to return a Number literal
  - String Adding: `"Hello" + "World"` turns into `"Hello World"`
  - Assignment: `x = literalType`; Prefix with var keyword to make a new variable
  - Comparison: Uses `>`, `>=`, `==`, `<`, `<=` to compare two numerical values
  - Ternary Operator: `x ? y : z`; if x then y else z
  - String concatontaions, math and comparison are performed left to right, if a specific operation needs to  be performed first, use a grouping to specify that
### Keywords
  - `if`; Conditionaly Executes next statment, whether single statement or block statemnet is up to user
  - `else`; Executes some code if the attatched if condition evaluates to false
  - `print`; Prints a varibale or a Literal
  - `var`; Initializes a variable
  - `true`; Literal, evaluates to bool::true
  - `false`; Literal, evaluates to bool::false
  - `nil`; Literal, prints to "NIL", evalutaes to a unit variant under the hood
  - `and`; will return the result of `x && y` (x and y)
  - `or`; will return the result of `x || y` (x or y)
  - `class`; TODO
  - `fun`; Defines a function for later use, functions are stored in the same space as Variables, and both will overwrite each other as mangling and shadowing are not implemented currently
  - `for`; While Loop syntactical sugar, instatiates first statement, iterates using second and check if it should continue iteration with third
  - `return`; TODO
  - `super`; TODO
  - `this`; TODO
  - `while`; C-Style while loop, while(bool){}
### Native Functions 
  - clock(), takes no arguments and returns current Unix time in seconds as a Number
  - print(), alias for Rust's println!() macro

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
