# Tokens
## Dynamic (in regex)
```
IDENTIFIER  = [\pL\pM_][\pL\pM\pN_]*
COMMENT     = (\/\/.*)|(\/\*[\s\S]*?\*\/)
STRING      = "((\\")|(\\\\)|([^"]))*?"
FLOAT       = ([1-9][0-9]*|0)[.][0-9]+
INTEGER     = [1-9][0-9]*|0
BOOL        = true|false
```
## Static (in quoted strings)
```cpp
OP_PLUS             = '+'
OP_MINUS            = '-'
OP_MULTIPLICATION   = '*'
OP_DIVISION         = '/'
OP_REMAINDER        = '%'

OP_NEGATE           = '!'
OP_AND              = '&'
OP_OR               = '|'

OP_EQUAL            = '=='
OP_UNEQUAL          = '!='
OP_LESSER           = '<'
OP_LESSER_EQUAL     = '<='
OP_GREATER          = '>'
OP_GREATER_EQUAL    = '>='

OP_ASSIGN           = '='
OP_RETURN           = '->'
OP_END              = ';'
OP_TYPE             = ':'
OP_SPLIT            = ','

TYPE_INT            = 'int'
TYPE_FLOAT          = 'float'
TYPE_STRING         = 'string'
TYPE_BOOL           = 'bool'

KW_LET              = 'let'
KW_FN               = 'fn'
KW_RETURN           = 'return'
KW_WHILE            = 'while'
KW_FOR              = 'for'
KW_IN               = 'in'
KW_IF               = 'if'
KW_ELSE             = 'else'

OPEN_CODEBLOCK      = '{'
CLOSE_CODEBLOCK     = '}'

OPEN_BRACKET        = '('
CLOSE_BRACKET       = ')'

OPEN_LIST           = '['
CLOSE_LIST          = ']'
```
# Grammar (in EBNF)
## Function definitions
```ebnf
function_definitions
    = function_definition
    | function_definition, function_definitions
    ;

function_definition
    = KW_FN, OPEN_BRACKET, parameters, CLOSE_BRACKET, [OP_RETURN, type], code_block
    ;

parameters
    = [parameter, {OP_SPLIT, parameter}]
    ;

parameter
    = identifier, OP_TYPE, type
    ;

type
    = primitive_type, [OPEN_LIST, CLOSE_LIST]
    ;

primitive_type
    = TYPE_INT
    | TYPE_FLOAT
    | TYPE_STRING
    | TYPE_BOOL
    ;
```
## Flow control and expressions
```ebnf
code_block
    = OPEN_CODEBLOCK, statements, [expression], CLOSE_CODEBLOCK
    ;

statements
    = {statement}
    ;

statement
    = expression, OP_END
    | if_statement
    | loop_statement,
    | return_statement,
    ;

if_statement
    = KW_IF, expression, code_block, [KW_ELSE, code_block]
    ;

loop_statement
    = while_statement
    | for_statement
    ;

while_statement
    = KW_WHILE, expression, code_block
    ;

for_statement
    = KW_FOR, identifier, KW_IN, expression, code_block
    ;

return_statement
    = KW_RETURN, expression
    ;

constant
    = STRING
    | FLOAT
    | INTEGER
    | BOOL
    ;

```
## Function calls
```ebnf

```
## Variable definition and assignment
```ebnf

```
## Arithmetical and logical operators
```ebnf

```