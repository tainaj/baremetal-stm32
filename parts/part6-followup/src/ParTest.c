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

