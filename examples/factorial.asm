; Factorial Calculator
; Calculates factorial of 5 (5! = 5 * 4 * 3 * 2 * 1 = 120)

LOADIMM R0, 5         ; Number to calculate factorial of
LOADIMM R1, 1         ; Result accumulator (starts at 1)
LOADIMM R2, 1         ; Decrement value

factorial_loop:
    ; Check if R0 <= 1 (base case)
    LOADIMM R3, 1     ; Load 1 for comparison
    CMP R0, R3        ; Compare R0 with 1
    JUMPIFZERO R0, done   ; If R0 == 0, we're done
    JUMPIFNEGATIVE done   ; If comparison was negative (R0 < 1), we're done
    
    ; Multiply result by current number
    MUL R1, R0, R1    ; R1 = R1 * R0
    
    ; Decrement R0
    SUB R0, R2, R0    ; R0 = R0 - 1
    
    ; Continue loop
    JUMP factorial_loop

done:
    PRINT R1          ; Print the factorial result (should be 120)
    HALT              ; End program