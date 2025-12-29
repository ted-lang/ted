# Grammar

This is the formal grammar for Ted in EBNF notation.

> TODO: This grammar is evolving. It reflects the documented syntax but does not yet capture every planned feature (for example, `fn` and advanced type forms).

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
bool_literal = "true" | "false" ;
string_literal = "\"" , { ? any character except \" and newline ? | escape_sequence } , "\"" ;
escape_sequence = "\\\\" , ( "\"" | "\\" | "n" | "r" | "t" ) ;

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
module_body = { port_decl | const_decl | signal_decl | struct_decl | module_stmt | event_handler | timed_loop | module_inst } ;

(* Ports and signals *)
port_decl = port_dir , ident , ":" , type , "," ;
port_dir = "in" | "out" | "inout" ;
signal_decl = "let" , ident , [ ":" , type ] , [ "=" , expr ] , ";" ;
const_decl = "const" , ident , ":" , type , "=" , expr , ";" ;
struct_decl = "struct" , ident , "{" , { struct_field } , "}" ;
struct_field = ident , ":" , type , "," ;

(* Types *)
type = primitive_type | array_type | custom_type ;
primitive_type = "bit" | "u8" | "u16" | "u32" | "u64"
               | "i8" | "i16" | "i32" | "i64"
               | "uint" , "<" , expr , ">" ;
array_type = "[" , type , ";" , expr , "]" ;
custom_type = ident , [ generic_args ] ;

(* Events *)
event_handler = "on" , event_spec , [ "if" , expr ] , block ;
event_spec = event_type , "(" , ident_list , ")" ;
event_type = "rising" | "falling" | "change" ;
ident_list = ident , { "," , ident } ;

(* Timed tasks *)
timed_loop = "loop" , block ;

(* Expressions *)
expr = time_expr | binary_expr | unary_expr | postfix_expr ;
unary_expr = unary_op , expr ;
unary_op = "!" | "~" | "-" ;
binary_expr = expr , binary_op , expr ;
binary_op = "+" | "-" | "*" | "/" | "%"
          | "&" | "|" | "^" | "<<" | ">>"
          | "==" | "!=" | "<" | "<=" | ">" | ">="
          | "&&" | "||" ;
time_expr = postfix_expr , "@" , time_offset ;
time_offset = ( "+" | "-" ) , expr ;
postfix_expr = primary_expr , { field_access | index_access | slice_access | call_suffix } ;
field_access = "." , ident ;
index_access = "[" , expr , "]" ;
slice_access = "[" , expr , ":" , expr , "]" ;
call_suffix = "(" , [ expr_list ] , ")" ;
expr_list = expr , { "," , expr } ;
primary_expr = ident | literal | "(" , expr , ")" | block | concat_expr ;
concat_expr = "{" , expr , { "," , expr } , "}" ;
literal = int_literal | bool_literal | time_literal | string_literal ;

(* Statements *)
stmt = let_stmt | assign_stmt | if_stmt | loop_stmt | break_stmt | expr_stmt ;
(* Module-level statements (combinational logic) *)
module_stmt = assign_stmt | expr_stmt ;
let_stmt = "let" , ident , [ ":" , type ] , [ "=" , expr ] , ";" ;
assign_stmt = lvalue , "=" , expr , ";" ;
lvalue = ident , { field_access | index_access | slice_access } ;
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
