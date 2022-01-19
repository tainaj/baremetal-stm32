#ifndef _VVC_MAIN_H
#define _VVC_MAIN_H

#include <stdint.h>
#ifdef VVC_F0
  #include "stm32f0xx.h"
#elif  VVC_L0
  #include "stm32l0xx.h"
#endif

// Define GPIOB pin mappings for our button.
#ifdef VVC_F0
  // STM32F0 Discovery board (STM32F051R8)
  #define BUTTON_PIN (12)
#elif  VVC_L0
  // STM32L0 Nucleo board (STM32L031K6)
  #define BUTTON_PIN (1)
  #define LED_PIN    (3)
#endif

// Define GPIOC pin mappings for our LED.
#ifdef VVC_F0
  #define LED_PIN    (9)
#endif

#endif
