lexer grammar JasmLexer;

// Whitespace
WHITESPACE         : [\t\r\n ]+ -> skip ;

// Keywords
LET                : 'let'      ;
IF                 : 'if'       ;
ELSE               : 'else'     ;
TRUE               : 'true'     ;
FALSE              : 'false'    ;
WHILE              : 'while'    ;
NULL               : 'null'     ;
FUNCTION           : 'function' ;
STRUCT             : 'struct'   ;
AS                 : 'as'       ;
RETURN             : 'return'   ;

// Operators
SHIFTLEFT          : '<<' ;
SHIFTRIGHT         : '>>' ;
AMPERSAND          : '&'  ;
DOT                : '.'  ;
EXCLAMATION        : '!'  ;
PLUS               : '+'  ;
MINUS              : '-'  ;
ASTERISK           : '*'  ;
DIVISION           : '/'  ;
PERCENT            : '%'  ;
ASSIGN             : '='  ;
EQUAL              : '==' ;
NOTEQUAL           : '!=' ;
GREATER            : '>'  ;
GREATEREQUAL       : '>=' ;
LESS               : '<'  ;
LESSEQUAL          : '<=' ;
AND                : 'and';
OR                 : 'or' ; 
XOR                : 'xor';
LEFTPAREN          : '('  ;
RIGHTPAREN         : ')'  ;
LEFTBRACE          : '{'  ;
RIGHTBRACE         : '}'  ;
LEFTSQUARE         : '['  ;
RIGHTSQUARE        : ']'  ;
SEMICOLON          : ';'  ;
COMMA              : ','  ;
COLON              : ':'  ;
QUESTIONMARK       : '?'  ;
DOUBLEQUOTE        : '"' -> pushMode(STRING);

// Literals
INTEGER            : '0'|[1-9][0-9]* ;
DECIMAL            : ('0'|[1-9][0-9]*) '.' [0-9]+ ;

// Identifiers
IDENTIFIER         : [A-Za-z_$][A-Za-z_$0-9]* ;


mode STRING; // see here: https://stackoverflow.com/questions/53504903/parse-string-antlr

TEXT: ~["]+ ;
DOUBLEQUOTE_STRING: '"' -> type(DOUBLEQUOTE), popMode;