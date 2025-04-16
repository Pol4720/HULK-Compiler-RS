%language "c++"
%define api.value.type {std::string}
%define parse.error verbose
%define api.token.constructor
%define api.location.type {location}
%locations

%code requires {
  #include <string>
}

%code {
  #include <iostream>
  void yyerror(const std::string &msg);
  int yylex(yy::parser::semantic_type *yylval, yy::parser::location_type *yylloc);
}

%token FUNCTION PROTOCOL EXTENDS INHERITS TYPE RETURN NEW
%token OR2 AND2 DOT SIN COS SQRT EXP LOG RAND PRINT IS AS
%token PI E LET IN TRUE FALSE IF ELSE ELIF WHILE FOR RANGE
%token ASSIGN1 ASSIGN2 COMMA OPAR CPAR PLUS MINUS STAR STAR2
%token DIVIDE POW MOD AND OR IMPLICIT NOT EQ NE GT GE LT LE
%token CONCAT CONCAT_SPACE LBRACE RBRACE LBRAKE RBRAKE SEMI COLON ARROW
%token COMMENT COMMENT2
%token <std::string> ID NUM STRING

%%

program:
    | program statement
    ;

statement:
      expr SEMI     { std::cout << "Statement válido\n"; }
    ;

expr:
      NUM           { std::cout << "Número: " << $1 << std::endl; }
    | ID            { std::cout << "Identificador: " << $1 << std::endl; }
    | STRING        { std::cout << "Cadena: " << $1 << std::endl; }
    ;

%%

void yyerror(const std::string &msg) {
    std::cerr << "Error de sintaxis: " << msg << std::endl;
}
