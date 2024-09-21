%ASSIGN UP_ARROW #01011110
%ASSIGN DOWN_ARROW #00100111
%ASSIGN LEFT_ARROW #00111100
%ASSIGN RIGHT_ARROW #00111110
%ASSIGN Q_CHARACTER #01010001
%ASSIGN X_VARIABLE #11111110
%ASSIGN Y_VARIABLE #11111101

; store x and y variables
mov r0 12                   ; middle of screen                         | 2
str r0 X_VARIABLE           ; x                                        | 4
str r0 Y_VARIABLE           ; y                                        | 6

; loop until input is 'q'
rprt r3 7                   ; read port 7                              | 7
mov r0 Q_CHARACTER          ; \                                        | 9
sub r0 r3                   ;  continue loop if port 7 is not 'q'      | 10
jmp_zro 75                  ; /                                        | 12

ldr r1 X_VARIABLE           ; load x variable                          | 14
ldr r2 Y_VARIABLE           ; load y variable                          | 16

; check if input is up arrow
mov r0 UP_ARROW             ; \                                        | 18
sub r0 r3                   ;  check if port 7 is equal to up arrow    | 19
jmp_zro 23                  ; /                                        | 21
jmp 28                      ; jump past if not equal                   | 23 

; move object up
mov r0 1                    ; \                                        | 25
add r2 r0                   ;  load y, add 1 to y, store y             | 26
str r2 Y_VARIABLE           ; /                                        | 28

; check if input is down arrow
mov r0 DOWN_ARROW           ; \                                        | 30
sub r0 r3                   ;  check if port 7 is equal to up arrow    | 31
jmp_zro 35                  ; /                                        | 33
jmp 40                      ; jump past if not equal                   | 35 

; move object down
mov r0 1                    ; \                                        | 37
sub r2 r0                   ;  load y, sub 1 from y, store y           | 38
str r2 Y_VARIABLE           ; /                                        | 40

; check if input is right arrow
mov r0 UP_ARROW             ; \                                        | 42
sub r0 r3                   ;  check if port 7 is equal to right arrow | 43
jmp_zro 47                  ; /                                        | 45
jmp 52                      ; jump past if not equal                   | 47 

; move object up
mov r0 1                    ; \                                        | 49
add r1 r0                   ;  load x, add 1 to x, store x             | 50
str r1 X_VARIABLE           ; /                                        | 52

; check if input is left arrow
mov r0 UP_ARROW             ; \                                        | 54
sub r0 r3                   ;  check if port 7 is equal to left arrow  | 55
jmp_zro 59                  ; /                                        | 57
jmp 64                      ; jump past if not equal                   | 59 

; move object up
mov r0 1                    ; \                                        | 61
add r1 r0                   ;  load x, sub 1 from x, store x           | 62
str r1 X_VARIABLE           ; /                                        | 64

; draw object. y is r2, x is r1
out r1
mov r0 32
msg r0
out r2
mov r0 10
msg r0
mov r0 0
wprt r0 7

; end of loop
jmp 6                       ; jump back to start of loop               | 66
halt                        ; end program                              | 67