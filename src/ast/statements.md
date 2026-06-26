statement      → exprStmt
               | ifStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;