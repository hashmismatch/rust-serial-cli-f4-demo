#ifndef SERIAL_DEMO_API_H_
#define SERIAL_DEMO_API_H_

#include "stm32f4xx_hal.h"

void stm32_delay(uint16_t milli);
void usart2_send_string(uint8_t* str, uint16_t len);
void usart2_send_byte(uint8_t byte);
int16_t usart2_try_get_byte();



#endif
