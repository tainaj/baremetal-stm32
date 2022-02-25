#ifndef _VVC_GLOBAL_H
#define _VVC_GLOBAL_H

#include <stdio.h>

// Core includes.
#ifdef VVC_F0
  #include "stm32f0xx.h"
#elif VVC_L0
  #include "stm32l0xx.h"
#endif

// ----------------------
// Global variables and defines.
#define LED_PIN (3)
// (Core system clock speed; initial value depends on the chip.)
extern volatile uint32_t core_clock_hz;

// Bonus: CCR1, CCR2
/*#define TIM_X_CCR_AFy     (0x0)
#define TIM_X_CCR_PIN_1   (2)
#define TIM_X_CCR_PIN_2   (3)*/

#endif
