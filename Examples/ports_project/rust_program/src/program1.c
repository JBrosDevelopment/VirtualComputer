// KEYBOARD;

const UP_ARROW = 0b01011110;
const DOWN_ARROW = 0b00100111;
const LEFT_ARROW = 0b00111100;
const RIGHT_ARROW = 0b00111110;
const SPEED = 1;

// BYTE 0;

uint8 x = 60;
uint8 y = 60;
uint8 r = 255;
uint8 g = 255;
uint8 b = 255;

// BYTE 20;

let read = read_port(7);
write_port(7, 0);

if (read == UP_ARROW) {
    y = y - SPEED,
    r = 0,
    g = 255,
    b = 255
};
if (read == DOWN_ARROW) {
    y = y + SPEED,
    r = 0,
    g = 255,
    b = 255
};
if (read == LEFT_ARROW) {
    x = x - SPEED,
    r = 255,
    g = 255,
    b = 0
};
if (read == RIGHT_ARROW) {
    x = x + SPEED,
    r = 255,
    g = 255,
    b = 0
};

if (x > 119) {
    x = 119
};
if (y > 119) {
    y = 119
};

// BYTE 140;

write_port(0, x);
write_port(1, y);
write_port(2, r);
write_port(3, g);
write_port(4, b);

// BYTE 155;

// Statements inside statements are not supported in the compiler, so a goto is used here;
goto(20);