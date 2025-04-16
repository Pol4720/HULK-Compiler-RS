%option c++
%option noyywrap
%option nodefault
%option nounput
%option noinput

%{
#include "parser.tab.hh"
using namespace yy;
#define RETURN_TOKEN(tok) return parser::token::tok
%}

nonzero_digit     [1-9]
digit             [0-9]
letter            [a-zA-Z]
id                ({letter}|_)([a-zA-Z0-9_])*
num               0|{nonzero_digit}({digit})*|0\.{digit}+|{nonzero_digit}({digit})*\.[0-9]+
string            \"([a-zA-Z0-9@#=:,()'+\-*/^%$\"!<>\\;[\] ]*)\"
comment           "#\\*([^*]|\\*+[^#])*\\*#"
comment2          \/\/[^\n]*

%%

"function"         RETURN_TOKEN(FUNCTION);
"protocol"         RETURN_TOKEN(PROTOCOL);
"extends"          RETURN_TOKEN(EXTENDS);
"inherits"         RETURN_TOKEN(INHERITS);
"type"             RETURN_TOKEN(TYPE);
"return"           RETURN_TOKEN(RETURN);
"new"              RETURN_TOKEN(NEW);
"or"               RETURN_TOKEN(OR2);
"and"              RETURN_TOKEN(AND2);
"."                RETURN_TOKEN(DOT);
"sin"              RETURN_TOKEN(SIN);
"cos"              RETURN_TOKEN(COS);
"sqrt"             RETURN_TOKEN(SQRT);
"exp"              RETURN_TOKEN(EXP);
"log"              RETURN_TOKEN(LOG);
"rand"             RETURN_TOKEN(RAND);
"print"            RETURN_TOKEN(PRINT);
"is"               RETURN_TOKEN(IS);
"as"               RETURN_TOKEN(AS);
"PI"               RETURN_TOKEN(PI);
"E"                RETURN_TOKEN(E);
"let"              RETURN_TOKEN(LET);
"in"               RETURN_TOKEN(IN);
"true"             RETURN_TOKEN(TRUE);
"false"            RETURN_TOKEN(FALSE);
"if"               RETURN_TOKEN(IF);
"else"             RETURN_TOKEN(ELSE);
"elif"             RETURN_TOKEN(ELIF);
"while"            RETURN_TOKEN(WHILE);
"for"              RETURN_TOKEN(FOR);
"range"            RETURN_TOKEN(RANGE);

":="               RETURN_TOKEN(ASSIGN2);
"="                RETURN_TOKEN(ASSIGN1);
","                RETURN_TOKEN(COMMA);
"#("               RETURN_TOKEN(OPAR);
"#)"               RETURN_TOKEN(CPAR);
"+"                RETURN_TOKEN(PLUS);
"-"                RETURN_TOKEN(MINUS);
"#*#*"             RETURN_TOKEN(STAR2);
"#*"               RETURN_TOKEN(STAR);
"/"                RETURN_TOKEN(DIVIDE);
"^"                RETURN_TOKEN(POW);
"%"                RETURN_TOKEN(MOD);
"&"                RETURN_TOKEN(AND);
"#|#|"             RETURN_TOKEN(IMPLICIT);
"#|"               RETURN_TOKEN(OR);
"!"                RETURN_TOKEN(NOT);
"=="               RETURN_TOKEN(EQ);
"!="               RETURN_TOKEN(NE);
">="               RETURN_TOKEN(GE);
">"                RETURN_TOKEN(GT);
"<="               RETURN_TOKEN(LE);
"<"                RETURN_TOKEN(LT);
"@"                RETURN_TOKEN(CONCAT);
"@@"               RETURN_TOKEN(CONCAT_SPACE);
"{"                RETURN_TOKEN(LBRACE);
"}"                RETURN_TOKEN(RBRACE);
"["                RETURN_TOKEN(LBRAKE);
"]"                RETURN_TOKEN(RBRAKE);
";"                RETURN_TOKEN(SEMI);
":"                RETURN_TOKEN(COLON);
"=>"               RETURN_TOKEN(ARROW);

{comment}          RETURN_TOKEN(COMMENT);
{comment2}         RETURN_TOKEN(COMMENT2);

{id}               { yylval->emplace(yytext); return parser::token::ID; }
{num}              { yylval->emplace(yytext); return parser::token::NUM; }
{string}           { yylval->emplace(yytext); return parser::token::STRING; }

[ \t\r\n]+         ; // Ignorar espacios en blanco

.                  { std::cerr << "Carácter no reconocido: " << yytext << std::endl; }

%%
