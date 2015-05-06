#include "serial_demo_api.h"
#include "stm32f4xx_hal.h"
#include "stm32f4xx_hal_uart.h"

void stm32_delay(uint16_t milli) {
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

