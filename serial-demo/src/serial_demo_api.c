#include "serial_demo_api.h"
#include "stm32f4xx_hal.h"
#include "stm32f4xx_hal_uart.h"

void stm32_delay(uint32_t milli) {
	HAL_Delay(milli);
}


extern UART_HandleTypeDef huart2;

void usart2_send_string(uint8_t* str, uint16_t len) {
	HAL_UART_Transmit(&huart2, str, len, 1000);
}

void usart2_send_byte(uint8_t byte) {
	while (!(USART2->SR & UART_FLAG_TXE));
    USART2->DR = (byte & 0xFF);
}

int16_t usart2_try_get_byte() {
    volatile unsigned int vsr;
    vsr = USART2->SR;
    if (vsr & UART_FLAG_RXNE) {
    	USART2->SR &= ~(UART_FLAG_RXNE);
    	return (USART2->DR & 0x1FF);
    }

    return -1;
}


// debug leds
uint16_t stm32_led_to_pin(uint8_t led) {
	switch (led) {
		case 1:
			return GPIO_PIN_14;
		case 2:
			return GPIO_PIN_12;
		case 3:
			return GPIO_PIN_15;
		default:
			return GPIO_PIN_13;
	}
}


void stm32_toggle_led(uint8_t led) {
	HAL_GPIO_TogglePin(GPIOD, stm32_led_to_pin(led));
}

void stm32_enable_led(uint8_t led) {
	HAL_GPIO_WritePin(GPIOD, stm32_led_to_pin(led), GPIO_PIN_SET);
}

void stm32_disable_led(uint8_t led) {
	HAL_GPIO_WritePin(GPIOD, stm32_led_to_pin(led), GPIO_PIN_RESET);
}
