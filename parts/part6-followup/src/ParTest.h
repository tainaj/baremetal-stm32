#ifndef _VVC_PARTEST_H
#define _VVC_PARTEST_H

#include "global.h"

void vParTestInitialise(void);
void vParTestSetLED(GPIO_TypeDef * GPIOx, uint8_t turnOn);
void vParTestToggleLED(GPIO_TypeDef * GPIOx);

#endif