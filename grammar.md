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
    = primitive_type, [OPEN_LIST, CLOSE_LIST]
    | TYPE_STRING
    ;

primitive_type
    = TYPE_INT
    | TYPE_FLOAT
    | TYPE_BOOL
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
int_list_constant
    = OPEN_LIST, [CONST_INT, {SPLIT, CONST_INT}], CLOSE_LIST
    ;

float_list_constant
    = OPEN_LIST, [CONST_FLOAT, {SPLIT, CONST_FLOAT}], CLOSE_LIST
    ;

bool_list_constant
    = OPEN_LIST, [CONST_BOOL, {SPLIT, CONST_BOOL}], CLOSE_LIST
    ;

constant
    = CONST_INT
    | int_list_constant
    | CONST_FLOAT
    | float_list_constant
    | CONST_BOOL
    | bool_list_constant
    | CONST_STRING
    ;
```

### Constant or identifier
```ebnf
const_or_identifier_expression
    = constant | IDENTIFIER | grouped
    ;

grouped
    = OPEN_BRACKET, expression, CLOSE_BRACKET
    ;
```

### Function call, list access
```ebnf
function_call_or_list_access_expression
    = const_or_identifier_expression, (function_call | list_access)
    ;

function_call
    = OPEN_BRACKET, function_arguments, CLOSE_BRACKET
    ;

function_arguments
    = [expression, {SPLIT, expression}]
    ;

list_access
    = OPEN_LIST, index_or_range_access, CLOSE_LIST
    ;

index_or_range_access
    = expression, [RANGE, expression]
    ;
```

### Unary operators
```ebnf
unary_operator_expression
    = unary_operators, unary_operator_expression
    | function_call_or_list_access_expression
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
    = logical_alternative_expression, [ASSIGN, expression]
    ;
```

### Return or variable declaration
```ebnf
return_or_variable_declaration_expression
    = [KW_RETURN | variable_declaration], variable_assignment_expression
    ;

variable_declaration
    = KW_LET, IDENTIFIER, TYPE_SIGNATURE, type, ASSIGN
    ;
```

### Expression
```ebnf
expression
    = return_or_variable_declaration_expression
    | for_expression
    | while_expression
    | if_expression
    | code_block
    ;
```
