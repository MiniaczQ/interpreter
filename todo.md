## Expressions
### Function call, list access
```ebnf
function_call_or_list_access_expression
    = const_or_identifier_expression, (function_call | list_access)
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
