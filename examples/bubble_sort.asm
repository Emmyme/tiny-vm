; Simple Array Demo
; Store 3 numbers in memory, then print them

; Store numbers in memory
LOADIMM R0, 42        ; Load 42
STORE R0, 0           ; Store at memory[0]

LOADIMM R0, 17        ; Load 17  
STORE R0, 4           ; Store at memory[4]

LOADIMM R0, 99        ; Load 99
STORE R0, 8           ; Store at memory[8]

; Print the numbers back
LOAD R1, 0            ; Load from memory[0]
PRINT R1              ; Print 42

LOAD R1, 4            ; Load from memory[4]
PRINT R1              ; Print 17

LOAD R1, 8            ; Load from memory[8]  
PRINT R1              ; Print 99

; Now let's sort them manually (simple comparison)
; Load all three numbers
LOAD R0, 0            ; R0 = 42
LOAD R1, 4            ; R1 = 17  
LOAD R2, 8            ; R2 = 99

; Compare R0 and R1, swap if R0 > R1
CMP R0, R1            ; Compare 42 vs 17
JUMPIFNEGATIVE no_swap1  ; If 42 <= 17, don't swap

; Swap R0 and R1
STORE R1, 0           ; Store 17 at memory[0]
STORE R0, 4           ; Store 42 at memory[4]
MOVE R1, R0           ; Update R0 = 17
MOVE R0, R1           ; Update R1 = 42

no_swap1:
; Now R0 should be <= R1

; Compare R1 and R2, swap if R1 > R2  
CMP R1, R2            ; Compare middle vs last
JUMPIFNEGATIVE no_swap2  ; If R1 <= R2, don't swap

; Swap R1 and R2
STORE R2, 4           ; Store R2 at memory[4]  
STORE R1, 8           ; Store R1 at memory[8]

no_swap2:
; Print sorted results
LOAD R0, 0            ; Load first number
PRINT R0

LOAD R0, 4            ; Load second number  
PRINT R0

LOAD R0, 8            ; Load third number
PRINT R0

HALT