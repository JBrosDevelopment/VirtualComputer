uint8 a = 0;

while (a < 5) {
    a = a + 1,
    char c = to_char(a),
    print(c)
}

// assembly code in test.asm takes 31 bytes.
// this compiles to 38 bytes.
