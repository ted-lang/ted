%{
    #include "node.hpp"

    extern int yylex();
    void yyerror(const char *s) { printf("ERROR: %sn", s); }
%}

/* still working */
