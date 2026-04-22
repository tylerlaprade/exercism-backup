(import (rnrs))

(define (leap-year? year)
  (define (divisible_by divisor) (= (modulo year divisor) 0))
  (and
   (divisible_by 4)
   (or
    (not (divisible_by 25)) 
    (divisible_by 16))))
