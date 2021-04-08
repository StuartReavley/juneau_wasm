grammar Jasm;

options {
    tokenVocab = JasmLexer;
}


statement  : expr=expression SEMICOLON                                                           #       expressionStatement
           | iff=if_statement                                                                    #               ifStatement 
           | WHILE condition=expression body=block                                          #            whileStatement
           | func                                                                                #         functionStatement
           | STRUCT   name=IDENTIFIER LEFTBRACE (parameter    (COMMA parameter   )*)? RIGHTBRACE # structDefinitionStatement
           | LET name=IDENTIFIER COLON t=typ SEMICOLON                                           #      declarationStatement
           | left=expression ASSIGN right=expression SEMICOLON                                   #           assignStatement
           | RETURN expr=expression? SEMICOLON                                                   #           returnStatement
           ; 

if_statement: IF condition=expression then=block (ELSE (els_if=if_statement | (els=block)))? ;


block : LEFTBRACE statement* RIGHTBRACE;

func       : FUNCTION name=IDENTIFIER LEFTPAREN (parameter    (COMMA parameter   )*)? RIGHTPAREN COLON t=typ implementation=block;

expression : v=value                                                                                                       #         constantExpression
           | LEFTPAREN expr=expression RIGHTPAREN                                                                          #            parenExpression
           | object=expression DOT name=IDENTIFIER                                                                         #     structAccessExpression
           | AMPERSAND expr=expression                                                                                     #        referenceExpression
           | ASTERISK  expr=expression                                                                                     #      dereferenceExpression
           | function=IDENTIFIER LEFTPAREN (expression (COMMA expression)*)? RIGHTPAREN                                    #       invocationExpression
           | left=expression LEFTSQUARE right=expression RIGHTSQUARE                                                       #      arrayAccessExpression
           | operator=(MINUS|EXCLAMATION) expr=expression                                                                  #            unaryExpression
           | expr=expression AS t=typ                                                                                      #             castExpression
           | left=expression operator=(DIVISION|ASTERISK|PERCENT)                          right=expression                #           binaryExpression
           | left=expression operator=(PLUS|MINUS)                                         right=expression                #           binaryExpression
           | left=expression operator=(EQUAL|NOTEQUAL|GREATER|GREATEREQUAL|LESS|LESSEQUAL) right=expression                #           binaryExpression
           | left=expression operator=(AND|OR|XOR|SHIFTLEFT|SHIFTRIGHT)                    right=expression                #           binaryExpression
           | name=IDENTIFIER                                                                                               #         variableExpression
           ;

value      : sign=MINUS? v=INTEGER                                                                                         #          integerValue
           | sign=MINUS? v=DECIMAL                                                                                         #            floatValue
           | v=TRUE                                                                                                        #             trueValue
           | v=FALSE                                                                                                       #            falseValue
           | v=NULL                                                                                                        #             nullValue
           | DOUBLEQUOTE v=TEXT DOUBLEQUOTE                                                                                #           stringValue
           ;

parameter  : name=IDENTIFIER COLON t=typ ;


typ        : name=IDENTIFIER                                                                                #   valueType
           | ASTERISK t=typ                                                                                 # pointerType
           ;

memberAssign: name=IDENTIFIER COLON expr=expression ;