; Simple Number Game
; The computer "thinks" of a number (42) and you try to guess it
; This demo simulates different guesses to show the logic

; The secret number
LOADIMM R0, 42        ; Secret number is 42

; Simulate guess 1: 50
LOADIMM R1, 50        ; Player guesses 50
SUB R1, R0, R2        ; R2 = guess - secret  
JUMPIFZERO R2, correct    ; If difference is 0, they're equal!
CMP R1, R0            ; Compare guess with secret  
JUMPIFNEGATIVE too_low ; If guess < secret, too low
; If we get here, guess > secret (too high)

; Print "Too high!" (ASCII: T=84, o=111, o=111, space=32, h=104, i=105, g=103, h=104, !=33)
LOADIMM R2, 84; T
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
PRINTCHAR R2  ; o again
LOADIMM R2, 32; space
PRINTCHAR R2
LOADIMM R2, 104; h
PRINTCHAR R2
LOADIMM R2, 105; i
PRINTCHAR R2
LOADIMM R2, 103; g
PRINTCHAR R2
LOADIMM R2, 104; h
PRINTCHAR R2
LOADIMM R2, 33; !
PRINTCHAR R2
LOADIMM R2, 10; newline
PRINTCHAR R2

JUMP guess2

too_low:
; Print "Too low!" (ASCII: T=84, o=111, o=111, space=32, l=108, o=111, w=119, !=33)
LOADIMM R2, 84; T
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
PRINTCHAR R2  ; o again
LOADIMM R2, 32; space
PRINTCHAR R2
LOADIMM R2, 108; l
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
LOADIMM R2, 119; w
PRINTCHAR R2
LOADIMM R2, 33; !
PRINTCHAR R2
LOADIMM R2, 10; newline
PRINTCHAR R2

JUMP guess2

guess2:
; Simulate guess 2: 30
LOADIMM R1, 30        ; Player guesses 30
SUB R1, R0, R2        ; R2 = guess - secret
JUMPIFZERO R2, correct    ; If difference is 0, they're equal!
CMP R1, R0            ; Compare guess with secret
JUMPIFNEGATIVE too_low2 ; If guess < secret, too low

; Too high case for guess 2
LOADIMM R2, 84; T
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
PRINTCHAR R2  ; o again
LOADIMM R2, 32; space
PRINTCHAR R2
LOADIMM R2, 104; h
PRINTCHAR R2
LOADIMM R2, 105; i
PRINTCHAR R2
LOADIMM R2, 103; g
PRINTCHAR R2
LOADIMM R2, 104; h
PRINTCHAR R2
LOADIMM R2, 33; !
PRINTCHAR R2
LOADIMM R2, 10; newline
PRINTCHAR R2
JUMP guess3

too_low2:
LOADIMM R2, 84; T
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
PRINTCHAR R2  ; o again
LOADIMM R2, 32; space
PRINTCHAR R2
LOADIMM R2, 108; l
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
LOADIMM R2, 119; w
PRINTCHAR R2
LOADIMM R2, 33; !
PRINTCHAR R2
LOADIMM R2, 10; newline
PRINTCHAR R2

guess3:
; Simulate guess 3: 42 (correct!)
LOADIMM R1, 42        ; Player guesses 42
SUB R1, R0, R2        ; R2 = guess - secret  
JUMPIFZERO R2, correct    ; If difference is 0, they're equal!

correct:
; Print "Correct!"
LOADIMM R2, 67; C
PRINTCHAR R2
LOADIMM R2, 111; o
PRINTCHAR R2
LOADIMM R2, 114; r
PRINTCHAR R2
LOADIMM R2, 114; r
PRINTCHAR R2
LOADIMM R2, 101; e
PRINTCHAR R2
LOADIMM R2, 99; c
PRINTCHAR R2
LOADIMM R2, 116; t
PRINTCHAR R2
LOADIMM R2, 33; !
PRINTCHAR R2
LOADIMM R2, 10; newline
PRINTCHAR R2

; Print the secret number to confirm
PRINT R0

HALT