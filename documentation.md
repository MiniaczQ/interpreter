# Language lexer, parser and interpreter
General topic: Lists  
Author: Jakub Motyka

# Functionality
Language focuses on list operations.  
It's is strongly and dynamically typed.  
(While most types can be determined during parsing, lists would require an additional layer of type resolving, which is outside of this project's scope.)

## Data types
The language provides few basic data types:
- `int`
- `float`
- `bool`
- `string`
- `list` - `[]`

as well as a hidden `none` type (aka `void`).

Numeric types allow for negation, addition, subtraction, multiplication and division, where the `int` type also allows for modulo. They also allow for all 6 comparison operations (==, !=, <, <=, >, >=).

Bool type can be negated and compared for equality or inequality.

String type can be added (as in concatenated) as well as compared for equality or inequality.

List can contain any type.

Both strings and lists allow for index `[i]` or range `[a::b]` based lookup. While indexing a list yields a specific element, indexing a string will always yield a string. Range access will always return a list or string. Only integer indices are accepted.

## Language
This language is expresison driven, meaning almost anything is an expression and returns a value of some sort.

## Constructs
Besides the typical expressions, such as mathematical operations, function calls, etc., there are:
- code-block - returns value of the last expression in their block that doesn't end with a semicolon, otherwise `none`
- if-else - much like code-block, but for each branch
- for-loop - allows iteration over lists, the loop returns a list of code block values
- while-loop - same as for-loop, but based on a predicate
- assignment - returns the assigned value
- declaration - same as assignment
- return - technically returns a `none`

## Standard library
Provides few basic functions:
- print - accepts any amount of any type, prints it to the console
- cast_int, cast_float, cast_bool, cast_string - accepts a single argument of acceptable type (per cast basis), success of casting from a string also depends on it's content
- type - accepts a single argument of any type, returns a string with the type name
- length - accepts a singular list or string, returns a length integer
- append - accepts a list and additional value of any type, returns the list with element added to the end

# Realization
The entire project was done in Rust language.

## Availability
While I only provide binaries for Windows and Linux, it'll likely compile for Mac and possibly more systems.

## User interface
The app provides a simple CLI.

Providing no arguments will display the manual.

The -i/--interactive flag will interpret code from standard input. This works well for piping files, but not so well as a shell.

The -f/--file [FILE] flag will interpret the provided file.

## Libraries
The following creates were used:
utf8-chars - simple wrapper for reading individual chars from streams
serde & ron - serialization library which helps in displaying the execution tree neatly

## Tests
Use the built-in testing architecture.

Scopes of tests are:
- individual lexems
- lexer
- individual language constructs
- parser
- individual expressions
- app (laxer + parser + interpreter)

## Error handling
Each layer of the app has it's own error handling system.

They can be accessed after the interpretation has ended or after a critical exception.

### Lexer
The lexer creates only warnings which are stored in a buffer. If a warning was critical it will easily be detected during parsing as a different error.


### Parser
The parser has both warnings and errors. Warnings are associated with language constructs which have a known structure, hence a missing token can be ignored. Too many warnings cause an error. Errors are created when missing tokens cannot be ignored. Warnings are stored in a buffer, while an error will be returned from failed parsing.

### Interpreter
The interpreter returns only critical errors, this includes attempts of division by 0, trying to use a non-existing variable or assigning `none` to a variable. Errors terminate the execution of program. They come with a context stack trace, although lack the exact position of the problem.

All warnings and errors are printed to the standard error output stream before app termination.

## Structure
As mentioned, the project is split into 3 modules: lexer, parser and interpreter.

While the lexer-parser link is highly modular and interchangeable, the parser and interpreter modules are tightly connected.

### Lexer
Lexer layer is responsible for reading a text input (a file or standard input) and yielding lexems. This is done lazily as to not waste memory or time. To implement that, a character scanner is used. It buffers singular characters inside of itself which provides a neat interface for detecting lexems. Scanner in this case is also responsible for unifying newline sequences. When the end of input is reached, it yields nothing.

### Parser
The parser accepts tokens (mapped from lexems) as an input. Lack of lexem is mapped to a special `EndOfTokens` token. The algorithm used is recursive descent LL(1) without backtracking. The parser also uses non-recursive alternatives for certain grammar rules.

Parser results can be split into 2 types: function definitions and function bodies (statements/expressions).
The surface layer of the program is a bunch of function definitions. At minimum you need a `main` function, which is the program's entry point.
Statements come in 2 types: expressions or expression terminations (semicolons). They belong inside the function definition's bodies.

As mentioned earlier, the interpreter is tightly interconnected with parser. The reason for that is lack of abstract syntax tree. The parser directly generates structures, which are responsible for interpreting the code.

### Interpreter
Is an abstract concept. It adds a small layer to the execution tree created by previous step. The main contribution are contexts, which give the program a stack-like architecture.

The first context to exist is the standard library context, it cannot store variables, but holds definitions for built-in functions.

The second context is the program context. It stores all the user-defined functions and no variables (if it did, they would be global variables).

Next are the function contexts, they provide a lookup to functions defined in higher contexts.
They allow for variable storage, which is initialized with function arguments, but stop the lookup to higher contexts. This means that functions can only access their arguments or local variables.

The last type of context is a block context. It's created by code-block, if-else, while-loop and for-loop expressions. It allows for both function and variable lookup, but also provide their local variable store. This means that variables created in this scope will only exist during this scope. Higher-scoped variables can also be accessed.
