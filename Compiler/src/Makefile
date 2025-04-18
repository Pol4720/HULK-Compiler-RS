# Archivos fuente
LEX_SRC=lexer.l
YACC_SRC=parser.y
MAIN_SRC=main.cpp

# Archivos generados
YACC_C=parser.tab.c
YACC_H=parser.tab.h
LEX_C=lex.yy.c
EXEC=parser

.PHONY: all compile run clean

all: compile

compile:
	bison -d $(YACC_SRC)
	flex $(LEX_SRC)
	g++ $(MAIN_SRC) $(YACC_C) $(LEX_C) -lfl -o $(EXEC)

run: compile
	./$(EXEC)

clean:
	rm -f $(YACC_C) $(YACC_H) $(LEX_C) $(EXEC)
