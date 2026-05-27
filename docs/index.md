# The Parlan Programming Language Documentation

This is the official documentation of the parlan programming language.

## Table Of Contents

- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Hello, World!](#hello-world)
- [Basic Concepts](#basic-concepts)
  - [Variables](#variables)
  - [Data Types](#data-types)
  - [Operators](#operators)
  - [Control Flow](#control-flow)
  - [Functions]()
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

**Note:** you can safely ignore any formatting warnigns thrown by `rustc` during the build

#### 3. Verify the Installation

After a successful build, a `parlan` executable will be generated in the root directory.

Run the following command to verify everything works and to check your parlan version:

```
> ./parlan --version
parlan v0.2
```

### Hello, World!

Let's write your first `hello, world!` program in parlan!  

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

**Note for Windows:** if you are on windows, make sure to add the `.exe` extension to your output file: `./parlan hello.par -o hello.exe`

#### Run the Program:

If the compilation succeeds, a `hello` executable will be generated in the current directory. Run it with the following command:

```
> ./hello
hello, world!
```

Congratulations! you made your first program in parlan!

> [!NOTE]  
> if parlan throws an error regarding a missing C compiler, it means `clang` (the default C compiler) cannot be found in your system.  
> in case you have a diffrent C compiler installed (such as `gcc`), you can explicitly provide the path or commant to it using the `-cc` flag: 
> ```./parlan hello.par -o hello -cc gcc```

## Basic Concepts

### Variables

In parlan, variables are declared using the `var` keyword, and every variable must have an explicity defined type.  
To declare a variable, you need to provide a name, a type, and a initial value. for example: 
```
var pi: float = 3.14
``` 

Variables in parlan are mutable by default. this means you can change their value at any point after they are declared.  
here is an example: 
```
func main(): int {
    // this is a comment!
    // here we define a variable:
    var a: int = 9

    // and here we change its value:
    a = 8

    return 0
}
```

> [!WARNING]
> the parlan compiler does not currently feature type checking, this means parlan itself will not throw an error if you assing a value of a diffrent type to an existing variable.  
> however, because parlan transpiles to C, the underlying C compiler will probably catch these type mismatches and will fail to compile.  
> So always ensure your new values match the original type

### Data Types

Parlan provides several primitive data types.  
The table below lists the available types, their descriptions, and their equivalents in the generated C code:

| Type | Description | equivalent in C |
| :-- | :-- | :-- |
| `int` | integer number | `int` |
| `floar` | floating-point number | `double` |
| `bool` | boolean (`true` or `false`) | `unsigned char` (`1` or `0`) |
| `str` | text string | `const char*` |
| `void` | absence of a value (used for functions that return nothing) | `void` |

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
Used to combine multiple conditions or invert booleans.

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

Control flow strcutures allow you to control the order in which your code executes based on conditions and loops

#### Conditionals

Parlan uses `if` statements to execute code only when a specific condition is met.  
You can use `else if` to check additional conditions,  
and `else` to execute code if none of the conditions were true.

Here is a little example:
```
var x: int = 6
if x < 10 {
    // something
} else if x == 6 {
    // another thing
} else {
    // yet another thing
}
```

#### Loops 

Loops are used to repeat a block of code multiple times

Currently, parlan supports `while` loops:

```
while 1 == 1 {
    // do something
}
```

### Functions

In parlan, functions are defined using the `func` keyword, followed by the name, a list of parameters with their types, the return type after a colon (`:`), and finally the function body.

Here is an example of a simple function:

```
func square(n: int): int {
    return n * n
}
```

#### External Functions

You can declare a function as external by prefixing it with the `extern` keyword. This tells the compiler that the function's implementation is defined elsewhere (outside the currrent file).  
This is especially useful for calling standard C library (`libc`) functions (e.g., `printf`) directly in parlan

Example: 

```
extern func printf(fmt: str, ...): int
```

#### The `main` function

Because parlan transpiles to C, every executable program must include a `main` function to act as the program's entry point.

If your program needs to accept command-line arguments, you can pass standard `argc` and `argv` parameters to the `main` function

Example:

```
func main(argc: int, argv: str): int {
    return argc
}
```

## What's New in Parlan

Here are the primary highlights and changes introduced in the latest release.

### Main Changes in v0.2

- **New compiler flags**  
  
    This new version of the compiler comes with new flags, here is a simple list of the added compiler flags:
    - **`-cc`:** manually specify your preferred C compiler
    - **`-time-report`:** make a simple resume of the time the compiler took to compile (inspired by clang `-ftime-report`)
    - **`--version`:** check your current compiler version
    - **`--help`:** shows a simple help message 
   
    *to check all the avaiable flags, run the `--help` command*

- **Full Rewrite**

    The main change of this new version is that the compiler was 100% rewrited from scratch, and the performance have increase (*formal benchmarks have not yet been conducted*)

    The main pourpuse of this rewrite was to make the compiler easier to expand and maintain

- **`extern` keyword**

    This new version arrives with the `extern` keyword, that makes posible to call foreign functions with 100% parlan syntax

- **Vector Intrinsics and `c_code` blocks removed**

    This version also totally removes vector intrinsics and `c_code` blocks.  

    * **Vector intrinsics:** Previouly used to store a collection of elements of the same type, these have been removed to reduce unnecessary compiler complexity

    * **`c_code` blocks:** Allowed direct C code injection into the final output. Was powerful, but it introduced safety risks. These blocks can be now replaced by `extern` functions
