/*
 * FreeRTOS V202112.00
 * Copyright (C) 2020 Amazon.com, Inc. or its affiliates.  All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * http://www.FreeRTOS.org
 * http://aws.amazon.com/freertos
 *
 * 1 tab == 4 spaces!
 */

/*-----------------------------------------------------------
 * Simple GPIO (parallel port) IO routines.
 *-----------------------------------------------------------*/

//#include "global.h"

/* Kernel includes. */
//#include "FreeRTOS.h"
//#include "task.h"

/* Standard demo include. */
#include "ParTest.h"

/* Hardware includes. */
//#include "stm320518_eval.h"
//#include "stm32f0xx.h"

/* Only the LEDs on one of the two seven segment displays are used. */
//#define partstMAX_LEDS		4

//static const Led_TypeDef xLEDs[ partstMAX_LEDS ] = { LED1, LED2, LED3, LED4 };

static void setup_clocks(void);
static void setup_clocks(void) {
  #ifdef VVC_F0
    // Reset the Flash 'Access Control Register', and
    // then set 1 wait-state and enable the prefetch buffer.
    // (The device header files only show 1 bit for the F0
    //  line, but the reference manual shows 3...)
    FLASH->ACR &= ~(0x00000017);
    FLASH->ACR |=  (FLASH_ACR_LATENCY |
                    FLASH_ACR_PRFTBE);
    // Configure the PLL to (HSI / 2) * 12 = 48MHz.
    // Use (HSI / PREDIV) as the core clock source. HSI = 8MHz.
    // ('Nucleo-32' boards do not ship with an HSE oscillator.)
    RCC->CFGR  &= ~(RCC_CFGR_PLLMUL |
                    RCC_CFGR_PLLSRC);
    RCC->CFGR  |=  (RCC_CFGR_PLLSRC_HSI_DIV2 |
                    RCC_CFGR_PLLMUL12);
    // Turn the PLL on and wait for it to be ready.
    RCC->CR    |=  (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY)) {};
    // Select the PLL as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_PLL);
    while (!(RCC->CFGR & RCC_CFGR_SWS_PLL)) {};
    // Set the global clock speed variable.
    core_clock_hz = 48000000;
  #elif VVC_F1
    // Set 2 wait states in flash and enable the prefetch buffer.
    FLASH->ACR &= ~(FLASH_ACR_LATENCY);
    FLASH->ACR |=  (FLASH_ACR_LATENCY_2 |
                    FLASH_ACR_PRFTBE);
    // Enable the HSE oscillator.
    // (This will be an infinite loop if your board doesn't have
    //  an HSE oscillator, but most cheap F103C8 boards seem to have one.)
    RCC->CR    |=  (RCC_CR_HSEON);
    while (!(RCC->CR & RCC_CR_HSERDY)) {};
    // Set the HSE oscillator as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_HSE);
    // Set the PLL multiplication factor to 9, for 8*9=72MHz.
    RCC->CFGR  &= ~(RCC_CFGR_PLLMULL);
    RCC->CFGR  |=  (RCC_CFGR_PLLMULL9);
    // Enable the PLL.
    RCC->CR    |=  (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY)) {};
    // Set the PLL as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_PLL);
    // The core clock is now 72MHz.
    core_clock_hz = 72000000;
  #elif VVC_F3
    // TODO.
    core_clock_hz = 8000000;
  #elif VVC_L0
    // Set the Flash ACR to use 1 wait-state
    // and enable the prefetch buffer and pre-read.
    FLASH->ACR |=  (FLASH_ACR_LATENCY |
                    FLASH_ACR_PRFTEN |
                    FLASH_ACR_PRE_READ);
    // Enable the HSI oscillator, since the L0 series boots
    // to the MSI one.
    // ('Nucleo-32' boards do not ship with an HSE oscillator.)
    RCC->CR    |=  (RCC_CR_HSION);
    while (!(RCC->CR & RCC_CR_HSIRDY)) {};
    // Configure the PLL to use HSI16 with a PLLDIV of
    // 2 and PLLMUL of 4.
    RCC->CFGR  &= ~(RCC_CFGR_PLLDIV |
                    RCC_CFGR_PLLMUL |
                    RCC_CFGR_PLLSRC);
    RCC->CFGR  |=  (RCC_CFGR_PLLDIV2 |
                    RCC_CFGR_PLLMUL4 |
                    RCC_CFGR_PLLSRC_HSI);
    // Enable the PLL and wait for it to stabilize.
    RCC->CR    |=  (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY)) {};
    // Select the PLL as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_PLL);
    while (!(RCC->CFGR & RCC_CFGR_SWS_PLL)) {};
    // The core clock is now 32MHz.
    core_clock_hz = 32000000;
  #endif
}

static void rcc_ahbenr_GPIOx(GPIO_TypeDef * GPIOx);
static void rcc_ahbenr_GPIOx(GPIO_TypeDef * GPIOx) {
  // Enable the LED GPIOx clock
  #ifdef VVC_F0
    if (GPIOx == GPIOA) {
      RCC->AHBENR |= RCC_AHBENR_GPIOAEN;
    }
    else if (GPIOx == GPIOB) {
      RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
    }
    else if (GPIOx == GPIOC) {
      RCC->AHBENR |= RCC_AHBENR_GPIOCEN;
    }
  #elif  VVC_L0
    if (GPIOx == GPIOA) {
      RCC->IOPENR |= RCC_IOPENR_IOPAEN;
    }
    else if (GPIOx == GPIOB) {
      RCC->IOPENR |= RCC_IOPENR_IOPBEN;
    }
    else if (GPIOx == GPIOC) {
      RCC->IOPENR |= RCC_IOPENR_IOPCEN;
    }
  #endif
}

#define ARG_DISABLE 0xff

#define MODER_INPUT   (0x0)
#define MODER_OUTPUT  (0x1)
#define MODER_ALT     (0x2)
#define MODER_ANALOG  (0x3)

#define PUPDR_NONE    (0x0)
#define PUPDR_UP      (0x1)
#define PUPDR_DOWN    (0x2)
#define PUPDR_UPDOWN  (0x3)

#define OTYPER_PP     (0x0)
#define OTYPER_OD     (0x1)

static void gpiox_config(GPIO_TypeDef * GPIOx, int pin_x, uint8_t moder, uint8_t pupdr, uint8_t otyper, uint8_t af_y);
static void gpiox_config(GPIO_TypeDef * GPIOx, int pin_x, uint8_t moder, uint8_t pupdr, uint8_t otyper, uint8_t af_y) {
  if (af_y != ARG_DISABLE) {
    GPIOx->AFR[pin_x/8] &= ~(0xF << ((pin_x % 8)*4));
    GPIOx->AFR[pin_x/8] |=  (af_y << ((pin_x % 8)*4));
  }
  if (moder != ARG_DISABLE) {
    GPIOx->MODER  &= ~(0x3 << (pin_x*2));
    GPIOx->MODER  |=  (moder << (pin_x*2));
  }
  if (pupdr |= ARG_DISABLE) {
    GPIOx->PUPDR  &= ~(0x3 << (pin_x*2));
    GPIOx->PUPDR  |=  (pupdr << (pin_x*2));
  }
  if (otyper != ARG_DISABLE) {
    GPIOx->OTYPER &= ~(1 << pin_x);
    GPIOx->OTYPER |=  (otyper << pin_x);
  }
}

/*-----------------------------------------------------------*/

void vParTestInitialise( void )
{
  // Initial clock setup.
  setup_clocks();

	// Initialise the lone LED
	rcc_ahbenr_GPIOx(LED_BANK);
	gpiox_config(LED_BANK, LED_PIN, MODER_OUTPUT, PUPDR_NONE, OTYPER_PP, ARG_DISABLE);
}
/*-----------------------------------------------------------*/

void vParTestSetLED( GPIO_TypeDef * GPIOx, uint8_t turnOn )
{
	if (turnOn == 0) {
		GPIOx->ODR &= ~(1 << LED_PIN);
	}
	else {
		GPIOx->ODR |= (1 << LED_PIN);
	}
}
/*-----------------------------------------------------------*/

void vParTestToggleLED( GPIO_TypeDef * GPIOx )
{
	taskENTER_CRITICAL();
	{
		GPIOx->ODR ^= (1 << LED_PIN);
	}
	taskEXIT_CRITICAL();
}
/*-----------------------------------------------------------*/

