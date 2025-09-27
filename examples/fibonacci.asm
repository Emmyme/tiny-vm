; Fibonacci Sequence Generator
; Generates the first 10 Fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34

; Initialize first two Fibonacci numbers
LOADIMM R0, 0         ; F(0) = 0
LOADIMM R1, 1         ; F(1) = 1
LOADIMM R2, 10        ; Counter: how many numbers to generate
LOADIMM R3, 0         ; Loop counter

; Print the first two numbers
PRINT R0              ; Print 0
PRINT R1              ; Print 1

; Main fibonacci loop
fib_loop:
    ADD R0, R1, R3    ; R3 = R0 + R1 (next Fibonacci number)
    PRINT R3          ; Print the next Fibonacci number
    
    ; Shift values: R0 = R1, R1 = R3
    MOVE R1, R0       ; R0 = previous R1
    MOVE R3, R1       ; R1 = newly calculated Fibonacci number
    
    ; Decrement counter
    LOADIMM R3, 1     ; Load 1 for subtraction
    SUB R2, R3, R2    ; R2 = R2 - 1
    
    ; Check if we should continue (R2 > 2 because we already printed 2 numbers)
    LOADIMM R3, 2     ; Load 2 for comparison
    CMP R2, R3        ; Compare R2 with 2
    JUMPIFPOSITIVE fib_loop  ; If R2 > 2, continue loop

HALT                  ; End program