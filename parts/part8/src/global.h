#ifndef _VVC_GLOBAL_H
#define _VVC_GLOBAL_H

#include <stdint.h>

#ifdef VVC_F0
  #include "stm32f0xx.h"
#elif  VVC_L0
  #include "stm32l0xx.h"
#endif

// Global defines.
#ifdef STM32F051x8
  #define PB_LED   (0)
  #define GP_TIM   (TIM16)
  #define NPX_P0   (1)
  #define NPX_P1   (20)
#else
  #error "Unrecognized MCU definition."
#endif
#define NUM_LEDS   (3)

// Global variables.
extern char     done;
extern uint32_t core_clock_hz;
extern int      ledi, ledb, ledt;
extern uint32_t grbs[NUM_LEDS];

#endif
