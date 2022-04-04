# Tokens
## Dynamic (in regex)
```
IDENTIFIER      = [\pL\pM_][\pL\pM\pN_]*
COMMENT         = (\/\/.*)|(\/\*[\s\S]*?\*\/)
CONST_STRING    = "((\\")|(\\\\)|([^"]))*?"
CONST_FLOAT     = ([1-9][0-9]*|0)[.][0-9]+
CONST_INTEGER   = [1-9][0-9]*|0
CONST_BOOL      = true|false
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

RANGE            = '..'
ASSIGN           = '='
RETURN_SIGNATURE = '->'
EXPRESSION_END   = ';'
TYPE_SIGNATURE   = ':'
SPLIT            = ','

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
    = KW_FN, OPEN_BRACKET, parameters, CLOSE_BRACKET, [RETURN_SIGNATURE, type], code_block
    ;

parameters
    = [parameter, {SPLIT, parameter}]
    ;

parameter
    = identifier, TYPE_SIGNATURE, type
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

## Expressions
### Code block
```ebnf
code_block
    = OPEN_CODEBLOCK, statements, [expression], CLOSE_CODEBLOCK
    ;

statements
    = {statement}
    ;

statement
    = expression, END_EXPRESSION
    ;
```

### If / if else
```ebnf
if_expression
    = KW_IF, expression, code_block, [KW_ELSE, code_block]
    ;
```

### Loops
```ebnf
while_expression
    = KW_WHILE, expression, code_block
    ;

for_expression
    = KW_FOR, identifier, KW_IN, expression, code_block
    ;
```

### Variable declaration
variable_declaration
    = KW_LET, IDENTIFIER, ASSIGN, 
    ;

### Return
```ebnf
return_expression
    = KW_RETURN, expression
    ;
```

### Function call
```ebnf
function_call
    = IDENTIFIER, OPEN_BRACKET, function_arguments, CLOSE_BRACKET
    ;

function_arguments
    = [expression, {SPLIT, expression}]
    ;
```

### Constants
```ebnf
primitive_constant
    = CONST_FLOAT
    | CONST_INTEGER
    | CONST_BOOL
    ;

list_constant
    = OPEN_LIST, [primitive_constant, {SPLIT, primitive_constant}], CLOSE_LIST
    ;

constant
    = primitive_constant
    | list_constant
    | CONST_STRING
    ;
```
