#include <cstdio>
#include <cstdlib>

// Declarar el puntero de archivo que utiliza el lexer
extern FILE *yyin;

// Declaración de la función de análisis sintáctico generada por Bison
extern int yyparse();

int main()
{
    // Abrir el archivo program.txt en modo lectura
    yyin = fopen("programa.txt", "r");
    if (!yyin)
    {
        fprintf(stderr, "Error: no se pudo abrir el archivo programa.txt\n");
        return EXIT_FAILURE;
    }

    // Ejecutar el parser; se analizará lo que esté en program.txt
    int parseResult = yyparse();

    // Cerrar el archivo de entrada
    fclose(yyin);

    // Retornar el resultado del análisis (0: sin errores)
    return parseResult;
}
