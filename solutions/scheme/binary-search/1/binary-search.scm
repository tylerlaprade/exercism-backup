(import (rnrs))

(define (bs-recursive lower-bound upper-bound array target)
  (define middle-index (quotient (+ lower-bound upper-bound) 2))
  (define middle-value (array-ref array middle-index))
  (cond [(= target middle-value) middle-index]
        [(< target middle-value) (if (= lower-bound middle-index) 'not-found (bs-recursive lower-bound middle-index array target))]
        [(= (+ middle-index 1) upper-bound) 'not-found]
        [else (bs-recursive (+ middle-index 1) upper-bound array target)]
  )
)
(define (binary-search array target)
  (define length (array-length array))
  (if (= 0 length) 'not-found (bs-recursive 0 length array target))
)
