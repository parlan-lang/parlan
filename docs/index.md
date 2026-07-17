# The Parlan Programming Language Documentation

This is the official documentation for the Parlan programming language.

## Table Of Contents

- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Hello, World!](#hello-world)
- [Basic Concepts](#basic-concepts)
  - [Variables](#variables)
  - [Data Types](#data-types)
  - [Operators](#operators)
  - [Control Flow](#control-flow)
  - [Functions](#functions)
- [What's New in Parlan](#whats-new-in-parlan)

## Getting Started

### Installation

Follow these steps to clone the repository and build the Parlan compiler from source.

#### 1. Clone the Repository

Open your terminal and run the following commands to clone the repository and navigate into the project directory:

```
> git clone https://github.com/parlan-lang/parlan.git
> cd parlan
```

#### 2. Build the Compiler

To build the compiler, you must have the Rust compiler (`rustc`) installed on your system.
Once `rustc` is installed, run the following command to build the project:

```
> rustc src/main.rs --crate-name parlan -O
```

**Note:** you can safely ignore any formatting warnings thrown by `rustc` during the build

#### 3. Verify the Installation

After a successful build, a `Parlan` executable will be generated in the root directory.

Run the following command to verify that everything works and to check your Parlan version:

```
> ./parlan --version
parlan v0.3
```

### Hello, World!

Let's write your first `hello, world!` program in Parlan!

Create a new file with the name: `hello.par` and write the following code:

```parlan
extern func printf(fmt: str, ...): int

func main(): int {
    printf("hello, world!\n")
    return 0
}
```

#### Compile the program:

Save the file, open your terminal, and run the following command to compile your program:

```
> ./parlan hello.par -o hello
```

**Note for Windows:** if you are on Windows, make sure to add the `.exe` extension to your output file: `./parlan hello.par -o hello.exe`

#### Run the Program:

If the compilation succeeds, a `hello` executable will be generated in the current directory. Run it with the following command:

```
> ./hello
hello, world!
```

Congratulations! You wrote your first program in Parlan!

> [!NOTE]
> If Parlan throws an error regarding a missing C compiler, it means that `clang` (the default C compiler) cannot be found on your system.
> If you have a different C compiler installed (such as `gcc`), you can explicitly provide the path or command to it using the `-cc` flag:
>
> ```bash
> ./parlan hello.par -o hello -cc gcc
> ```

## Basic Concepts

### Variables

In Parlan, variables are declared using the `var` keyword, and the type of the variable can be inferred.
To declare a variable, you need to provide a name, a type, and a initial value. For example:
```
var pi: float = 3.14;
```

Variables in Parlan are mutable by default. This means you can change their value at any point after they are declared.

Here is an example:
```
func main(): int {
    // this is a comment!
    // here we define a variable:
    var a = 9;

    // and here we change its value:
    a = 8;

    return 0;
}
```

### Data Types

Parlan provides several primitive data types.
The table below lists the available types, their descriptions, and their equivalents in the generated C code:

| Type | Description | Equivalent in C |
| :-- | :-- | :-- |
| `int` | Integer number | `int` |
| `float` | Floating-point number | `double` |
| `bool` | Boolean (`true` or `false`) | `unsigned char` (`1` or `0`) |
| `str` | Text string | `const char*` |
| `void` | Absence of a value (used for functions that return nothing) | `void` |

### Operators

Parlan supports all standard arithmetic, logical (boolean), and comparison operators.

#### Arithmetic Operators
Used to perform standard mathematical calculations.

| Operator | Description | Example |
| :---: | :--- | :--- |
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |

#### Logical (Boolean) Operators
Used to combine multiple conditions or invert Booleans.

| Operator | Description | Example |
| :---: | :--- | :--- |
| `and` | Logical AND (true if both are true) | `condition1 and condition2` |
| `or` | Logical OR (true if at least one is true) | `condition1 or condition2` |

#### Comparison Operators
Used to compare two values. These operations always return a `bool` (`true` or `false`).

| Operator | Description | Example |
| :---: | :--- | :--- |
| `==` | Equal to | `a == b` |
| `!=` | Not equal to | `a != b` |
| `>` | Greater than | `a > b` |
| `<` | Less than | `a < b` |
| `>=` | Greater than or equal to | `a >= b` |
| `<=` | Less than or equal to | `a <= b` |

### Control Flow

Control flow structures allow you to control the order in which your code executes based on conditions and loops.

#### Conditionals

Parlan uses `if` statements to execute code only when a specific condition is met.
You can use `else if` to check additional conditions,
and `else` to execute code if none of the conditions are true.

Here is a little example:
```
var x = 6;
if x < 10 {
    // something
} else if x == 6 {
    // another thing
} else {
    // yet another thing
}
```

#### Loops

Loops are used to repeat a block of code multiple times.

Currently, Parlan supports `while` loops:

```
while 1 == 1 {
    // do something
}
```

### Functions

In Parlan, functions are defined using the `func` keyword, followed by the function's name, a list of parameters with their types, then the return type after a colon (`:`), and finally the function body.

Here is an example of a simple function:

```
func square(n: int): int {
    return n * n
}
```

#### External Functions

You can declare a function as external by prefixing it with the `extern` keyword. This tells the compiler that the function's implementation is defined elsewhere (outside the current file).
This is especially useful for calling the standard C library (`libc`) functions (e.g., `printf`) directly in Parlan.

Example:

```
extern func printf(fmt: str, ...): int;
```

#### The `main` function

Because Parlan transpiles to C, every executable program must include a `main` function to act as the program's entry point.

If your program needs to accept command-line arguments, you can pass the standard `argc` and `argv` parameters to the `main` function.

Example:

```
func main(argc: int, argv: str): int {
    return argc
}
```

## What's New in Parlan

Here are the primary highlights and changes introduced in the latest release.

### Main Changes in v0.3

- **New Type Checker and Semantic Analyzer**

This new version features a type checker and a semantic analyzer, making Parlan safer.

In previous versions, you could assign a variable of type `int` a value of type `float`, but in this version the type checker ensures that this never happens.

The semantic analyzer, on the other hand, ensures that all symbols are correctly defined before they are used.

- **Syntax Change**

    In this new version, every statement *must* end with a semicolon, like C or Rust.

    This change makes Parlan less ambiguous.
