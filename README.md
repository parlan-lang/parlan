# The Parlan Programming Language

Parlan is an open-source programming language designed to make programming simpler. 

##  Overview

Parlan was designed to be a simple, reliable and beginner-friendly programming language.  
Parlan archives this because it does not "reinvent the wheel" with the syntax or semantics, it simply polishes what already exists  

## Documentation

The language and compiler documentation can be founded at the [`docs/`](./docs/) directory

## Getting Started

To try out Parlan, follow these steps:

1. Clone the repository and navigate into it:

```Bash
git clone https://github.com/parlan-lang/parlan.git
cd parlan
```

1. Create a source file at the root named `hello.par` and paste this:

```Parlan
func main(): int {
        // printing
        return 0;
}
```

3. Run the compiler executing the following command in your terminal:

```Bash
cargo run -- hello.par --time --compile
```

*The --time flag will display the execution time and debug information to show you exactly how the compiler processes your code.*
*The --compile flag will compile the output file with Clang (if you use GCC use --gcc flag)*
