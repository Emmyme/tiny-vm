; Basic Math Operations Demo
; This program demonstrates basic arithmetic operations

; Add two numbers
LOADIMM R0, 15        ; Load 15 into R0
LOADIMM R1, 27        ; Load 27 into R1
ADD R0, R1, R2        ; R2 = 15 + 27 = 42
PRINT R2              ; Print 42

; Subtract numbers
SUB R0, R1, R3        ; R3 = 15 - 27 = -12
PRINT R3              ; Print -12

; Multiply numbers
LOADIMM R0, 6         ; Load 6 into R0
LOADIMM R1, 7         ; Load 7 into R1
MUL R0, R1, R2        ; R2 = 6 * 7 = 42
PRINT R2              ; Print 42

; Divide numbers
LOADIMM R0, 84        ; Load 84 into R0
LOADIMM R1, 2         ; Load 2 into R1
DIV R0, R1, R3        ; R3 = 84 / 2 = 42
PRINT R3              ; Print 42

HALT                  ; End program