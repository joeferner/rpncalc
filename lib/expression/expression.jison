
%lex
%%

\s+                   /* skip whitespace */
[0-9]+("."[0-9]+)?\b  return 'NUMBER'
[a-zA-Z]+[a-zA-Z0-9]* return 'IDENTIFIER'
"*"                   return '*'
"/"                   return '/'
"+"                   return '+'
"-"                   return '-'
"^"                   return '^'
"%"                   return '%'
"("                   return '('
")"                   return ')'
";"                   return ';'
<<EOF>>               return 'EOF'
.                     return 'INVALID'

/lex

%left '+' '-'
%left '*' '/'
%left '^'
%right '%'
%left UMINUS


%start start
%%

start
  : expressions EOF
    { return $1; }
  ;

expressions
  : e
    { $$ = [ $1 ]; }
  | expressions ';' e
    { $$ = $1.concat($3); }
  ;

e
  : IDENTIFIER '(' parameterList? ')'
    { $$ = yy.functionCall($1, $3); }
  | IDENTIFIER
    { $$ = yy.value($1); }
  | e '+' e
    { $$ = yy.binary($1, $3, $2); }
  | e '-' e
    { $$ = yy.binary($1, $3, $2); }
  | e '*' e
    { $$ = yy.binary($1, $3, $2); }
  | e '/' e
    { $$ = yy.binary($1, $3, $2); }
  | '-' e %prec UMINUS
    { $$ = yy.unary($2, 'neg'); }
  | '(' e ')'
    { $$ = $2; }
  | NUMBER
    { $$ = yy.value($1); }
  ;

parameterList
  : e
    { $$ = [ $1 ]; }
  | parameterList ',' e
    { $$ = $1.concat($3); }
  ;
