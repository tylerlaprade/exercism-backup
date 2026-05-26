(import (rnrs))

(define (divisible-by? year divisor) (zero? (modulo year divisor)))

(define (leap-year? year)
  (and (divisible-by? year 4)
       (or (not (divisible-by? year 100)) 
           (divisible-by? year 400))))
