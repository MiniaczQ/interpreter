# Tokens
## Dynamic (in regex)
```
IDENTIFIER      = [\pL\pM_][\pL\pM\pN_]*
COMMENT         = (\/\/.*)|(\/\*[\s\S]*?\*\/)
CONST_FLOAT     = ([1-9][0-9]*|0)[.][0-9]+
CONST_INT       = [1-9][0-9]*|0
CONST_BOOL      = true|false
CONST_STRING    = "((\\")|(\\\\)|([^"]))*?"
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

RANGE            = '::'
ASSIGN           = '='
RETURN_SIGNATURE = '->'
EXPRESSION_END   = ';'
TYPE_SIGNATURE   = ':'
SPLIT            = ','

TYPE_INT            = 'int'
TYPE_FLOAT          = 'float'
TYPE_BOOL           = 'bool'
TYPE_STRING         = 'string'

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

ETX                 = '\x03'
```
# Grammar (in EBNF)
## Function definitions
```ebnf
function_definitions
    = {function_definition}
    ;

function_definition
    = KW_FN, IDENTIFIER, OPEN_BRACKET, parameters, CLOSE_BRACKET, [RETURN_SIGNATURE, type], code_block
    ;

parameters
    = [parameter, {SPLIT, parameter}]
    ;

parameter
    = IDENTIFIER, TYPE_SIGNATURE, type
    ;

type
    = TYPE_INT
    | TYPE_FLOAT
    | TYPE_BOOL
    | TYPE_STRING
    | OPEN_LIST, CLOSE_LIST
    ;
```

## Expressions
### Code block
```ebnf
code_block
    = OPEN_CODEBLOCK, statements, CLOSE_CODEBLOCK
    ;

statements
    = {statement}
    ;

statement
    = expression
    | EXPRESSION_END
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
    = KW_FOR, IDENTIFIER, KW_IN, expression, code_block
    ;
```

### Constants
```ebnf
list_expression
    = OPEN_LIST, [expression, {SPLIT, expression}], CLOSE_LIST
    ;

constant
    = CONST_INT
    | CONST_FLOAT
    | CONST_BOOL
    | CONST_STRING
    ;
```

### Identifier or function call
```ebnf
identifier_or_function_call
    = IDENTIFIER, [function_call]
    ;

function_call
    = OPEN_BRACKET, function_arguments, CLOSE_BRACKET
    ;

function_arguments
    = [expression, {SPLIT, expression}]
    ;
```

### Grouped
```ebnf
grouped
    = OPEN_BRACKET, expression, CLOSE_BRACKET
    ;
```

### Constant or identifier
```ebnf
const_or_identifier_or_function_call_expression
    = constant | list_expression | identifier_or_function_call | grouped
    ;
```

### List access
```ebnf
list_access
    = OPEN_LIST, index_or_range_access, CLOSE_LIST
    ;

index_or_range_access
    = expression, [RANGE, expression]
    ;

list_access_expression
    = const_or_identifier_or_function_call_expression, [list_access]
    ;
```

### Unary operators
```ebnf
unary_operator_expression
    = {unary_operators}, list_access_expression
    ;

unary_operators
    = OP_NEGATE | OP_MINUS
    ;
```

### Multiplication, division
```ebnf
mul_div_expression
    = unary_operator_expression, {mul_div_operators, unary_operator_expression}
    ;

mul_div_operators
    = OP_MULTIPLICATION | OP_DIVISION | OP_REMAINDER
    ;
```

### Addition, subtraction
```ebnf
add_sub_expression
    = mul_div_expression, {add_sub_operators, mul_div_expression}
    ;

add_sub_operators
    = OP_PLUS | OP_MINUS
    ;
```

### Comparison
```ebnf
comparison_expression
    = add_sub_expression, {comparison_operators, add_sub_expression}
    ;

comparison_operators
    = OP_EQUAL | OP_UNEQUAL | OP_LESSER | OP_LESSER_EQUAL | OP_GREATER | OP_GREATER_EQUAL
    ;
```

### Logical conjunction
```ebnf
logical_conjunction_expression
    = comparison_expression, {OP_AND, comparison_expression}
    ;
```

### Logical alternative
```ebnf
logical_alternative_expression
    = logical_conjunction_expression, {OP_OR, logical_conjunction_expression}
    ;
```

### Variable assignment
```ebnf
variable_assignment_expression
    = logical_alternative_expression, {ASSIGN, expression}
    ;
```

### For, while, if or codeblock
```ebnf
control_flow_expression
    = variable_assignment_expression
    | for_expression
    | while_expression
    | if_expression
    | code_block
    ;
```

### Return
```ebnf
return_expression
    = KW_RETURN, [control_flow_expression]
    ;
```

### Variable declaration
```ebnf
variable_declaration_expression
    = KW_LET, IDENTIFIER, COLON, TYPE_SIGNATURE, type, ASSIGN, control_flow_expression
    ;
```

### Expression
```ebnf
expression
    = return_expression
    | variable_declaration
    | control_flow_expression
    ;
```
