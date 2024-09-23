// MOUSE PAD;

// BYTE 0;

uint8 x = 60;
uint8 y = 60;
uint8 r = 250;
uint8 g = 150;
uint8 b = 50;

// BYTE 20;

uint8 dif_x = read_port(5);
uint8 dif_y = read_port(6);
write_port(5, 125);
write_port(6, 125);

uint8 move_x = dif_x - 125;
uint8 move_y = dif_y - 125;

if (dif_x > 125) {
    x = x - move_x,
    r = r + move_x
};
if (dif_x < 125) {
    x = x + move_x,
    g = g + move_x
};
if (dif_y > 125) {
    y = y - move_y,
    b = b + move_y
};
if (dif_y < 125) {
    y = y + move_y,
    r = r - move_y,
    g = g - move_y,
    b = b - move_y
};

if (x > 119) {
    x = 119
};
if (y > 119) {
    y = 119
};

write_port(0, x);
write_port(1, y);
write_port(2, r);
write_port(3, g);
write_port(4, b);

// Statements inside statements are not supported in the compiler, so a goto is used here;
goto(20);