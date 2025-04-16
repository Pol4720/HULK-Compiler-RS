#include "parser.tab.hh"

int main() {
    yy::parser parser;
    return parser.parse();
}
