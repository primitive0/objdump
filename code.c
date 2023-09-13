#include <stdint.h>
#include <avr/iom328p.h>

int main(void) {
    DDRD |= (1 << DDD6);
    DDRD |= (1 << DDD1);
    PORTD &= ~(1 << PORTD1);

    while (1) {
        // xor?
        if ((PIND >> PIND1) & 1 == 0) {
            PORTD |= (1 << PORTD6);
        } else {
            PORTD &= ~(1 << PORTD6);
        }

        // 16_000_000
        
        // sleep
        // uint8_t i = 98;
        // while (i != 0) {
        //     i--;
        // }
        _delay_ms(???);
    }
}
