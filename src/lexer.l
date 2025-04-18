%{
#include "parser.tab.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
%}

%option noyywrap

%%

[0-9]+  { 
            yylval.dval = atof(yytext);  
            return NUMBER;
        }
"+"     { return PLUS; }
"-"     { return MINUS; }
"*"     { return PRODUCT; }
"/"     { return DIV; }
[A-Za-z]+   {
                yylval.symp = strdup(yytext);
                return WORD;
            }

[ \t\n]+          { /* ignorar espacios en blanco */ }

.                 { printf("Carácter inesperado: %s\n", yytext); }

%%
