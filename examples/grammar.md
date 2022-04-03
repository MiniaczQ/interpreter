# Tokens
## Literals
```
lt_comment
lt_string
lt_integer
lt_float
lt_bolean
```
## Keywords
```
kw_function_decl
kw_variable_decl
```
## Other
```
tk_primitive_type
```
# EBNF
```ebnf
function_definitions
    : function_definition
    | function_definition, function_definitions
    ;

function_definition
    : kw_function_decl, '(', parameters, ')', ['->', type], code_block
    ;

parameters
    : [parameter, {',', parameter}]
    ;

parameter
    : identifier, ':', type
    ;

type
    : tk_primitive_type, ['[', ']']
    ;

code_block
    : '{', statements, '}'
    ;

statements
    : {statement}
    ;

statement
    : expression, ';'
    | if_statement
    | loop_statement,
    | return_statement,
    ;

if_statement
    : 'if', expression, code_block, ['else', code_block]
    ;

loop_statement
    : while_statement
    | for_statement
    ;

while_statement
    : 'while', expression, code_block
    ;

for_statement
    : 'for', identifier, 'in', expression, code_block
    ;


```