(import (rnrs))

(define (transcribe-nucleotide nucleotide)
  (case nucleotide
    ((#\a) #\u)
    ((#\t) #\a)
    ((#\g) #\c)
    ((#\c) #\g)))

(define (dna->rna dna)
  (list->string (map transcribe-nucleotide (string->list dna))))
