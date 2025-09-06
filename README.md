# pure_lisp_interpreter
Pure Lisp interpreter in Rust

## Todo (in any order)
[X] add doubles
[X] fix sexpr_to_string for proper lists
[X] add sexpr_to_string tests
[X] add tests for proper lists
[X] ' short hand for quote
[ ] redo . short hand for creating improper lists (requires cons)
[ ] figure out how to do lambdas (just store the list for the function?)

**_Functions_**  
[X] quote  
[X] car  
[X] cdr  
[X] setq  
[X] equal (structural equality)
[X] atom  
[X] listp  
[X] null  
[X] floor
[X] + (add) 
[X] - (subtract)
[X] * (mulitply)
[X] / (divide)
[X] mod  
[X] print  
[X] eval  
[ ] cons  
[ ] cond  
[ ] defun (what happens if a param is nil?) 
[ ] eq (address equality / literal equality) (requires string interning)
