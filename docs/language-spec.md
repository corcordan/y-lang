# Y Language Specification

### Simple Hello World

```
"Hello World!" |> p
```

In this first example, a classic Hello World program is created. This program takes a string and prints it to the console. There are a few important pieces of syntax in this program to look at. First, the most noticable element is the Pipe ```|>``` element. The Pipe will feed whatever argument on the left into whatever identifier (in this case a function) on the right. On the left is a String which is enclosed by two Double-Quotes ```"```. On the right is ```p``` which is a built-in function called Print. This function will print the argument passed in and output it to the console.

### Print

```
"Y is so cool!" |> p
5 |> p
```

The print function: ```p``` is a built-in function that takes in one parameter. The parameter can really be any expression, from Strings and Numbers. To send the arguments to the print function, to the left, you must put the argument, then the Pipe ```|>``` like in the Hello World example above. More about the Pipe later.

[Future]
Arrays and Functions will be able to be passed in to the print function.

### Strings

```
"Y is so cool!"
"y"
""
```
Strings allow basic text to be seen by the user. A String literal is created with a Double-quote ```"``` at the beginning, text in the middle, and another Double-quote to complete the String. Strings are an Array of Chars (more information below). They can be passed to the print ```p``` function. Strings can be empty, one character, or more to form whole sentences.

[Future]
String Concatenation will be possible with a Plus ```+``` and a String on the left and right.

String Interpolation will be possible by using Backticks ```\```` to surround the string. Curly braces within the Backticks will allow variables or other values within to be added to the String in that spot.

### Chars

```
'y'
''
```
Chars are Unicode values that make up Strings or can be on their own. The Single-Quote ```'``` is reserved specifically for this use. To create a Char literal, a Single-Quote marks the start of a char, the text in the middle, and another Single-Quote marks the end of the Char. Chars can be empty or one character, nothing more.

### Numbers

```
5
3.14
-42
```
Numbers in Y are floating-point values (64-bit) that support basic arithmetic operations. Number literals can be integers or decimals, positive or negative. They are created by writing digits with an optional decimal point. Numbers can be passed to the print function and used in all mathematical operations.

### Booleans

```
1
0
```
Y uses numeric booleans where 1 represents true and 0 represents false. This approach simplifies the language while maintaining compatibility with arithmetic operations. Comparison operators return 1 for true and 0 for false.

[Future]
Dedicated boolean literals ```true``` and ```false``` may be added for clarity.

### Comments

```
# This is a comment
5 + 3 |> p  # This adds two numbers and prints the result
```
Comments in Y start with a hash symbol ```#``` and continue until the end of the line. They are completely ignored by the interpreter and are useful for documenting code or temporarily disabling parts of programs.

### Addition

```
5 + 3 |> p
```
Addition combines two numbers using the Plus ```+``` operator. It takes the number on the left and adds it to the number on the right, producing their sum. Addition follows standard mathematical rules and works with both positive and negative numbers.

### Subtraction

```
10 - 4 |> p
```
Subtraction removes the right number from the left number using the Minus ```-``` operator. It calculates the difference between the two operands, supporting negative results when appropriate.

### Multiplication

```
6 * 7 |> p
```
Multiplication multiplies two numbers together using the Star ```*``` operator. It produces the product of the left and right operands, following standard mathematical multiplication rules.

### Division

```
15 / 3 |> p
```
Division divides the left number by the right number using the Slash ```/``` operator. It produces the quotient of the division. Division by zero will cause a runtime error.

### Modulo

```
5 % 2 |> p
```
Modulo returns the remainder of dividing the left number by the right number using the Percent ```%``` operator. For example, 5 divided by 2 equals 2 with a remainder of 1. Modulo by zero will cause a runtime error.

### Power

```
2 ** 3 |> p
```
Power raises the left number to the power of the right number using the Double-Star ```**``` operator. It calculates exponents, so 2 to the power of 3 equals 8.

### Arithmetic (Summarization)

```
(2 + 3) * 4 - 1 |> p
5 % 2 |> p
2 ** 3 + 1 |> p
```
Y supports all standard arithmetic operations: addition (+), subtraction (-), multiplication (*), division (/), modulo (%), and exponentiation (**). Operations follow standard mathematical precedence (parentheses first, then exponents, then multiplication/division/modulo, then addition/subtraction). All operations work with floating-point numbers and will panic on division or modulo by zero. These operations form the foundation of mathematical computations in Y programs.

### Greater Than

```
5 > 3 |> p
```
Greater Than compares if the left number is larger than the right number using the ```>``` operator. It returns 1 if true, 0 if false.

### Greater Than Or Equal To

```
5 >= 5 |> p
```
Greater Than Or Equal To compares if the left number is larger than or equal to the right number using the ```>=``` operator. It returns 1 if true, 0 if false.

### Less Than

```
3 < 5 |> p
```
Less Than compares if the left number is smaller than the right number using the ```<``` operator. It returns 1 if true, 0 if false.

### Less Than Or Equal To

```
5 <= 5 |> p
```
Less Than Or Equal To compares if the left number is smaller than or equal to the right number using the ```<=``` operator. It returns 1 if true, 0 if false.

### Equal To

```
5 == 5 |> p
```
Equal To checks if the left value is equal to the right value using the Double-Equal ```==``` operator. It returns 1 if they are equal, 0 if they are not.

### Not Equal To

```
5 != 3 |> p
```
Not Equal To checks if the left value is not equal to the right value using the Bang-Equal ```!=``` operator. It returns 1 if they are different, 0 if they are the same.

### Equality & Comparison (Summarization)

```
(5 > 3) == 1 |> p
10 >= 5 |> p
2 < 4 |> p
7 <= 7 |> p
5 == 5 |> p
3 != 4 |> p
```
Y provides six comparison operators that work with numbers: greater than (>), greater than or equal (>=), less than (<), less than or equal (<=), equal to (==), and not equal to (!=). All comparisons return 1 for true and 0 for false, allowing them to be used in arithmetic expressions. Comparisons have lower precedence than arithmetic operations but higher than logical operations. These operators are essential for control flow and conditional logic in Y programs.

### Increment

```
++5 |> p
5++ |> p
```
Increment increases a number by 1 using the Double-Plus ```++``` operator as a prefix or postfix. It adds 1 to the operand and returns the result.

### Decrement

```
--10 |> p
10-- |> p
```
Decrement decreases a number by 1 using the Double-Minus ```--``` operator as a prefix or postfix. It subtracts 1 from the operand and returns the result.

### Negate

```
-5 |> p
```
Negate changes the sign of a number using the Minus ```-``` operator as a prefix. It makes positive numbers negative and negative numbers positive.

### Not

```
!1 |> p
```
Not inverts a boolean value using the Bang ```!``` operator as a prefix. It turns 1 (true) into 0 (false) and 0 (false) into 1 (true).

### Unaries (Summarization)

```
++5 |> p
--10 |> p
-3 |> p
!0 |> p
```
Y supports four unary operators that work on single operands: increment (++), decrement (--), negate (-), and logical not (!). All unary operators are prefix (come before the operand), with increment and decrement also allowing postfix, and have higher precedence than binary operations. Increment and decrement modify numbers by 1, negate changes the sign, and not inverts boolean values. These operators are useful for quick modifications and logical operations in expressions.