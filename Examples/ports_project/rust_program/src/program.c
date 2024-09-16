const UP_ARROW = 0b01011110;
const DOWN_ARROW = 0b00100111;
const LEFT_ARROW = 0b00111100;
const RIGHT_ARROW = 0b00111110;

while (true) {
    uint8 keyboard = read_port(7),

    if (keyboard == UP_ARROW) {
        println('u'),
        write_port(7, 0)
    }
}