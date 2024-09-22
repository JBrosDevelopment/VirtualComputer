%ASSIGN UP_ARROW #01011110
%ASSIGN DOWN_ARROW #00100111
%ASSIGN LEFT_ARROW #00111100
%ASSIGN RIGHT_ARROW #00111110
%ASSIGN Q_CHARACTER #01010001
%ASSIGN X_VARIABLE #11111110
%ASSIGN Y_VARIABLE #11111101
%ASSIGN RED_VARIABLE #11111100
%ASSIGN GREEN_VARIABLE #11111001
%ASSIGN BLUE_VARIABLE #11111010

; store x and y variables
mov r0 30                   ; x and y                                  | 2
mov r1 200                  ; red                                      | 4
mov r2 110                  ; green                                    | 6
mov r3 20                   ; blue                                     | 8
str r0 X_VARIABLE           ; \                                        | 10
str r0 Y_VARIABLE           ;  \                                       | 12
str r1 RED_VARIABLE         ;   store variables                        | 14
str r2 GREEN_VARIABLE       ;  |                                       | 16
str r3 BLUE_VARIABLE        ; /                                        | 18

; loop until input is 'q'
rprt r3 7                   ; \                                        | 19
mov r0 0                    ;  read and clear port 7                   | 21
wprt r0 7                   ; /                                        | 22
mov r0 Q_CHARACTER          ; \                                        | 24
sub r0 r3                   ;  continue loop if port 7 is not 'Q'      | 25
jmp_zro 90                  ; /                                        | 27

ldr r1 X_VARIABLE           ; load x variable                          | 29
ldr r2 Y_VARIABLE           ; load y variable                          | 31

; check if input is up arrow
mov r0 UP_ARROW             ; \                                        | 33
sub r0 r3                   ;  check if port 7 is equal to up arrow    | 34
jmp_zro 38                  ; /                                        | 36
jmp 43                      ; jump past if not equal                   | 38

; move pixel up
mov r0 1                    ; \                                        | 40
sub r2 r0                   ;  load y, sub 1 from y, store y           | 41
str r2 Y_VARIABLE           ; /                                        | 43

; check if input is down arrow
mov r0 DOWN_ARROW           ; \                                        | 45
sub r0 r3                   ;  check if port 7 is equal to up arrow    | 46
jmp_zro 50                  ; /                                        | 48
jmp 55                      ; jump past if not equal                   | 50

; move pixel down
mov r0 1                    ; \                                        | 52
add r2 r0                   ;  load y, add 1 to y, store y             | 53
str r2 Y_VARIABLE           ; /                                        | 55

; check if input is right arrow
mov r0 RIGHT_ARROW          ; \                                        | 57
sub r0 r3                   ;  check if port 7 is equal to right arrow | 58
jmp_zro 62                  ; /                                        | 60
jmp 67                      ; jump past if not equal                   | 62

; move pixel right
mov r0 1                    ; \                                        | 64
add r1 r0                   ;  load x, add 1 to x, store x             | 65
str r1 X_VARIABLE           ; /                                        | 67

; check if input is left arrow
mov r0 LEFT_ARROW           ; \                                        | 69
sub r0 r3                   ;  check if port 7 is equal to left arrow  | 70
jmp_zro 74                  ; /                                        | 72
jmp 79                      ; jump past if not equal                   | 74

; move pixel left
mov r0 1                    ; \                                        | 76
sub r1 r0                   ;  load x, sub 1 from x, store x           | 77
str r1 X_VARIABLE           ; /                                        | 79

; draw pixel
wprt r1 0                   ; write X to pixel X port                  | 80
wprt r2 1                   ; write Y to pixel Y port                  | 81
ldr r0 RED_VARIABLE         ; load red                                 | 83
wprt r0 2                   ; write red to red channel port            | 88
ldr r1 GREEN_VARIABLE       ; load green                               | 85
wprt r1 3                   ; write green to green channel port        | 89
ldr r2 BLUE_VARIABLE        ; load blue                                | 87
wprt r2 4                   ; write blue to blue channel port          | 90

; end of loop
jmp 18                       ; jump back to start of loop               | 92
halt                         ; end program                              | 93