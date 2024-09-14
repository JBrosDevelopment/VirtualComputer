%ASSIGN FIVE_ADDRESS 254
%ASSIGN ZERO_ADDRESS 253
%ASSIGN A_ADDRESS 252
%ASSIGN START_ASCII_ADDRESS 251

; set up constants
MOV R0 5                    ; 2
STR R0 FIVE_ADDRESS         ; 4
MOV R0 48                   ; 6
STR R0 START_ASCII_ADDRESS  ; 8

; let a = 0;
LDR R0 ZERO_ADDRESS         ; 10
STR R0 A_ADDRESS            ; 12

; while (a < 5) {
LDR R1 FIVE_ADDRESS         ; 14
SUB R0 R1                   ; 15
JMP_NEG 19                  ; 17
JMP 30                      ; 19

; a = a + 1;
LDR R0 A_ADDRESS            ; 21
INC R0                      ; 22
STR R0 A_ADDRESS            ; 24

; print(a);
LDR R2 START_ASCII_ADDRESS  ; 26
ADD R2 R0                   ; 27
MSG R2                      ; 28

; }
JMP 14                      ; 30

; end program
HALT                        ; 31