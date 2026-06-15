Arranged based on HIGHER -> LOWER order of precedence
expression -> equality ;

equality   -> ("==" | "!=")

comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )

term       -> factor ( ( "-" | "+" ) factor )

factor     -> unary ( ( "/" | "*" ) unary )

unary      -> ( "!" | "-" ) unary
            | primary ;

primary    -> NUMBER | STRING | "true" | "false" | "nil"
            | "(" expression ")" ;