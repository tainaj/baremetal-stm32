#include "main.h"

/**
 * Main program.
 */
int main(void) {
  #ifdef VVC_F0
    // Enable the GPIOC peripheral in 'RCC_AHBENR'.
    RCC->AHBENR |= RCC_AHBENR_GPIOCEN;
  #elif  VVC_L0
    RCC->IOPENR |= RCC_IOPENR_IOPBEN;
  #endif

  // Initialize the GPIOB and GPIOC pins. (F0/L0)
  // (C9/B3) is connected to an LED on the 'Discovery' board.
  //    It should be set to push-pull low-speed output.
  #ifdef VVC_F0
    GPIOC->MODER  &= ~(0x3 << (LED_PIN*2));
    GPIOC->MODER  |=  (0x1 << (LED_PIN*2));
    GPIOC->OTYPER &= ~(1 << LED_PIN);
    GPIOC->PUPDR  &= ~(0x3 << (LED_PIN*2));
  #elif  VVC_L0
    GPIOB->MODER  &= ~(0x3 << (LED_PIN*2));
    GPIOB->MODER  |=  (0x1 << (LED_PIN*2));
    GPIOB->OTYPER &= ~(1 << LED_PIN);
    GPIOB->PUPDR  &= ~(0x3 << (LED_PIN*2));
  #endif

  while (1) {
    // Toggle LED every 1 second (1000000 microticks)
    delay(1000000);
    #ifdef VVC_F0
      GPIOC->ODR ^= (1 << LED_PIN);
    #elif  VVC_L0
      GPIOB->ODR ^= (1 << LED_PIN);
    #endif
  }
}
