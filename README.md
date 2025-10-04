# pure_lisp_interpreter
Pure Lisp interpreter in Rust

## Todo (in any order)
[X] add doubles
[X] fix sexpr_to_string for proper lists
[X] add sexpr_to_string tests
[X] add tests for proper lists
[X] ' short hand for quote

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
[X] cons  
basically the second value gets concatenated to the first
- (cons 1 1) => (1 . 1)
- (cons '(1 1) '(1 1)) => ((1 1) 1 1)
- (cons 1 '(1 1 1)) => (1 1 1 1)
- (cons '(1 1 1) 1) => ((1 1 1) . 1)
[X] cond  
(cond 
    ((conditional) (return value))
    (t (return value))
)
[X] defun (what happens if a param is nil?) 
- just push it to the oblist 
[ ] eq (address equality / literal equality) (requires string interning)
- how do i do this without making unsafe code in rust? is it possible? something to come back to
