#include <stdbool.h>
#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);

static void spin_sleep_us(unsigned int us) {
  for (unsigned int i = 0; i < us * 6; i++) {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms) {
  spin_sleep_us(ms * 1000);
}

static void set_fsel_output() {
	*GPIO_FSEL1 |= 0x40000;
}

static void set_led_on() {
	*GPIO_SET0 |= 0x10000;
}

static void set_led_off() {
	*GPIO_CLR0 |= 0x10000;
}

int main(void) {
	set_fsel_output();

	bool on = true;
	while(true) {
		if (on) {
			set_led_on();
		}
		else {
			set_led_off();
		}
		spin_sleep_ms(100);
		on = !on;
	}
}
