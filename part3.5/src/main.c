#include "main.h"

/**
 * Main program.
 */
int main(void) {
  #ifdef VVC_F0
    // Enable the GPIOB and GPIOC peripherals in 'RCC_AHBENR'.
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN | RCC_AHBENR_GPIOCEN;
  #elif  VVC_L0
    RCC->IOPENR |= RCC_IOPENR_IOPBEN | RCC_IOPENR_IOPCEN;
  #endif

  // Initialize the GPIOB and GPIOC pins. (F0/L0)
  // (B12/B1) should be set to 'input' mode with pull-up.
  GPIOB->MODER  &= ~(0x3 << (BUTTON_PIN*2));
  GPIOB->PUPDR  &= ~(0x3 << (BUTTON_PIN*2));
  GPIOB->PUPDR  |=  (0x1 << (BUTTON_PIN*2));
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

  // Keep track of whether the button is pressed.
  uint8_t button_down = 0;
  while (1) {
    // Invert the IDR register since '0' means 'pressed'.
    uint32_t idr_val = ~GPIOB->IDR;
    if (idr_val & (1 << BUTTON_PIN)) {
      // The button is pressed; if it was not already
      // pressed, change the LED state.
      if (!button_down) {
        #ifdef VVC_F0
          GPIOC->ODR ^= (1 << LED_PIN);
        #elif  VVC_L0
          GPIOB->ODR ^= (1 << LED_PIN);
        #endif
      }
      button_down = 1;
    }
    else {
      button_down = 0;
    }
  }
}
