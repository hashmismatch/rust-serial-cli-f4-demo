################################################################################
# Automatically-generated file. Do not edit!
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
../Src/gpio.c \
../Src/main.c \
../Src/serial_demo_api.c \
../Src/stm32f4xx_hal_msp.c \
../Src/stm32f4xx_it.c \
../Src/usart.c \
../Src/usb_device.c \
../Src/usbd_cdc_if.c \
../Src/usbd_conf.c \
../Src/usbd_desc.c 

OBJS += \
./Src/gpio.o \
./Src/main.o \
./Src/serial_demo_api.o \
./Src/stm32f4xx_hal_msp.o \
./Src/stm32f4xx_it.o \
./Src/usart.o \
./Src/usb_device.o \
./Src/usbd_cdc_if.o \
./Src/usbd_conf.o \
./Src/usbd_desc.o 

C_DEPS += \
./Src/gpio.d \
./Src/main.d \
./Src/serial_demo_api.d \
./Src/stm32f4xx_hal_msp.d \
./Src/stm32f4xx_it.d \
./Src/usart.d \
./Src/usb_device.d \
./Src/usbd_cdc_if.d \
./Src/usbd_conf.d \
./Src/usbd_desc.d 


# Each subdirectory must supply rules for building sources it contributes
Src/%.o: ../Src/%.c
	@echo 'Building file: $<'
	@echo 'Invoking: Cross ARM C Compiler'
	arm-none-eabi-gcc -mcpu=cortex-m4 -mthumb -mthumb-interwork -mfloat-abi=soft -Og -fmessage-length=0 -fsigned-char -ffunction-sections -fdata-sections -ffreestanding -fno-move-loop-invariants -Wall -Wextra  -g3 -DDEBUG -DUSE_FULL_ASSERT -DSTM32F407xx -DUSE_HAL_DRIVER -DHSE_VALUE=8000000 -I"C:\code\rust-serial-f4-demo\serial-demo\Inc" -I"C:\code\rust-serial-f4-demo\serial-demo\Middlewares\ST\STM32_USB_Device_Library\Core\Inc" -I"C:\code\rust-serial-f4-demo\serial-demo\Middlewares\ST\STM32_USB_Device_Library\Class\CDC\Inc" -I"C:\code\rust-serial-f4-demo\serial-demo\Drivers\CMSIS\Include" -I"C:\code\rust-serial-f4-demo\serial-demo\Drivers\CMSIS\Device\ST\STM32F4xx\Include" -I"C:\code\rust-serial-f4-demo\serial-demo\Drivers\STM32F4xx_HAL_Driver\Inc" -std=gnu11 -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@" -c -o "$@" "$<"
	@echo 'Finished building: $<'
	@echo ' '


