#ifndef _VVC_UTIL_H
#define _VVC_UTIL_H

#include <stdint.h>
#ifdef VVC_F0
  #include "stm32f0xx.h"
#elif  VVC_L0
  #include "stm32l0xx.h"
#endif

void systick_init(uint32_t);
void timer_us_init(uint32_t);
uint16_t micros(void);
uint32_t millis(void);
void micro_wait(uint16_t delay);
void milli_wait(uint32_t delay);

#endif