#ifndef _VVC_MAIN_H
#define _VVC_MAIN_H

#include <stdint.h>
#ifdef VVC_F0
  #include "stm32f0xx.h"
#elif  VVC_L0
  #include "stm32l0xx.h"
#endif

#include "util.h"

// Delay function (MCU_CLASS dependent)
//void delay(int);

#ifdef VVC_F0
  // Define GPIOC pin mappings for our LED.
  #define LED_PIN    (9)
#elif  VVC_L0
  // Define GPIOB pin mappings for our LED.
  #define LED_PIN    (3)
#endif

extern volatile uint32_t core_clock_hz;

#endif
