# Grammar

This is the formal grammar for Ted in EBNF notation.

## Lexical Grammar

```ebnf
(* Whitespace and comments *)
whitespace = " " | "\t" | "\n" | "\r" ;
line_comment = "//" , { any_char - "\n" } , "\n" ;
block_comment = "/*" , { any_char } , "*/" ;

(* Identifiers *)
ident = ( letter | "_" ) , { letter | digit | "_" } ;
letter = "a" | ... | "z" | "A" | ... | "Z" ;
digit = "0" | ... | "9" ;

(* Literals *)
int_literal = decimal | hex | binary | octal ;
decimal = digit , { digit | "_" } ;
hex = "0x" , hex_digit , { hex_digit | "_" } ;
binary = "0b" , ( "0" | "1" ) , { "0" | "1" | "_" } ;
octal = "0o" , octal_digit , { octal_digit | "_" } ;

time_literal = int_literal , time_unit ;
time_unit = "ns" | "us" | "ms" | "s" ;

(* Keywords *)
keyword = "mod" | "in" | "out" | "inout" | "let" | "const"
        | "on" | "rising" | "falling" | "change"
        | "if" | "else" | "loop" | "break" | "return"
        | "fn" | "struct" | "true" | "false" ;
```

## Syntax Grammar

```ebnf
(* Program structure *)
program = { module } ;

module = "mod" , ident , [ generic_params ] , "{" , module_body , "}" ;
module_body = { port_decl | signal_decl | event_handler | module_inst } ;

(* Ports and signals *)
port_decl = port_dir , ident , ":" , type , "," ;
port_dir = "in" | "out" | "inout" ;
signal_decl = "let" , ident , [ ":" , type ] , [ "=" , expr ] , ";" ;
const_decl = "const" , ident , ":" , type , "=" , expr , ";" ;

(* Types *)
type = primitive_type | array_type | custom_type ;
primitive_type = "bit" | "u8" | "u16" | "u32" | "u64"
               | "i8" | "i16" | "i32" | "i64"
               | "uint" , "<" , int_literal , ">" ;
array_type = "[" , type , ";" , int_literal , "]" ;
custom_type = ident , [ generic_args ] ;

(* Events *)
event_handler = "on" , event_spec , [ "if" , expr ] , block ;
event_spec = event_type , "(" , ident_list , ")" ;
event_type = "rising" | "falling" | "change" ;
ident_list = ident , { "," , ident } ;

(* Expressions *)
expr = unary_expr | binary_expr | primary_expr | time_expr ;
unary_expr = unary_op , expr ;
unary_op = "!" | "~" | "-" ;
binary_expr = expr , binary_op , expr ;
binary_op = "+" | "-" | "*" | "/" | "%"
          | "&" | "|" | "^" | "<<" | ">>"
          | "==" | "!=" | "<" | "<=" | ">" | ">="
          | "&&" | "||" ;
time_expr = expr , "@" , time_offset ;
time_offset = ( "+" | "-" ) , ( int_literal | time_literal ) ;
primary_expr = ident | literal | "(" , expr , ")" | block ;

(* Statements *)
stmt = assign_stmt | if_stmt | loop_stmt | break_stmt | expr_stmt ;
assign_stmt = ident , "=" , expr , [ "@" , time_offset ] , ";" ;
if_stmt = "if" , expr , block , [ "else" , ( block | if_stmt ) ] ;
loop_stmt = "loop" , block ;
break_stmt = "break" , ";" ;
expr_stmt = expr , ";" ;

block = "{" , { stmt } , [ expr ] , "}" ;

(* Module instantiation *)
module_inst = ident , [ generic_args ] , ident , "{" , connections , "}" , ";" ;
connections = { ident , ":" , ( expr | "_" ) , "," } ;

(* Generics *)
generic_params = "<" , generic_param , { "," , generic_param } , ">" ;
generic_param = ident , ":" , type , [ "=" , expr ] ;
generic_args = "<" , expr , { "," , expr } , ">" ;
```

## Operator Precedence

From highest to lowest:

| Precedence | Operators | Associativity |
|------------|-----------|---------------|
| 1 | `@` (time) | Left |
| 2 | `!` `~` `-` (unary) | Right |
| 3 | `*` `/` `%` | Left |
| 4 | `+` `-` | Left |
| 5 | `<<` `>>` | Left |
| 6 | `<` `<=` `>` `>=` | Left |
| 7 | `==` `!=` | Left |
| 8 | `&` | Left |
| 9 | `^` | Left |
| 10 | `|` | Left |
| 11 | `&&` | Left |
| 12 | `||` | Left |
| 13 | `=` (assignment) | Right |
