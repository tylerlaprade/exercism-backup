#!r6rs

(import (rnrs))

(define (two-fer . maybe-name)
  (display "One for ")
  (display (if (null? maybe-name) 'you (car maybe-name)))
  (display ", one for me.")
)