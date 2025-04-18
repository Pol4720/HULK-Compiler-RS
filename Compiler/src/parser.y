%{
#include <iostream>
#include <cstdio>
#include <cstdlib>
using namespace std;
int yylex(void);
void yyerror(const char *s);
extern FILE *yyin;
%}

%union {
    double dval;
    char* symp;
}

%token <dval> NUMBER
%token <symp> WORD
%token PLUS MINUS
%token PRODUCT DIV

%left PLUS MINUS
%left PRODUCT DIV

%type <dval> EXP TERM FACTOR

%%

input:
    EXP { printf("Resultado: %g\n", $1); }
    ;

EXP: EXP PLUS TERM   { $$ = $1 + $3; }
    | EXP MINUS TERM  { $$ = $1 - $3; }
    | TERM           { $$ = $1; }
    ;

TERM: TERM PRODUCT FACTOR   { $$ = $1 * $3; }
    | TERM DIV FACTOR   {
                            if ($3 == 0){
                                yyerror("Error: División por cero.");
                                $$ = 0;
                            } else {
                                $$ = $1 / $3;
                            }
                        }
    | FACTOR { $$ = $1; }
    ;

FACTOR: NUMBER  { $$ = $1; }
    | '(' EXP ')'   { $$ = $2; }
    ;

%%

void yyerror(const char* s) {
    fprintf(stderr, "Error: %s\n", s);
}

