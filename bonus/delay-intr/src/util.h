#ifndef _VVC_UTIL_H
#define _VVC_UTIL_H

#include <stdint.h>
#ifdef VVC_F0
  #include "stm32f0xx.h"
#elif  VVC_L0
  #include "stm32l0xx.h"
#endif

//#define MILLIS_U16_MASK 0xFFFF                          // max millis value for micros() purposes
//#define MAX_MICROS      (MILLIS_U16_MASK * 1000 + 1000) // =0x03E80000

void systick_init(uint32_t);
uint32_t millis(void);
uint32_t micros(void);
void micro_wait(uint32_t delay, uint32_t sig_fig);
void milli_wait(uint32_t delay);

#endif