#include "main.h"

/**
 * Main program.
 */
int main(void) {
  // Initial clock setup.
  #ifdef VVC_F0
    // Reset the Flash 'Access Control Register', and
    // then set 1 wait-state and enable the prefetch buffer.
    // (The device header files only show 1 bit for the F0
    //  line, but the reference manual shows 3...)
    FLASH->ACR &= ~(0x00000017);
    FLASH->ACR |=  (FLASH_ACR_LATENCY |
                    FLASH_ACR_PRFTBE);
    // Configure the PLL to (HSI / 2) * 12 = 48MHz.
    // Use a PLLMUL of 0xA for *12, and keep PLLSRC at 0
    // to use (HSI / PREDIV) as the core source. HSI = 8MHz.
    RCC->CFGR  &= ~(RCC_CFGR_PLLMUL |
                    RCC_CFGR_PLLSRC);
    RCC->CFGR  |=  (RCC_CFGR_PLLSRC_HSI_DIV2 |
                    RCC_CFGR_PLLMUL12);
    // Turn the PLL on and wait for it to be ready.
    RCC->CR    |=  (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY)) {};
    // Select the PLL as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_PLL);
    while (!(RCC->CFGR & RCC_CFGR_SWS_PLL)) {};
    // Set the global clock speed variable.
    //core_clock_hz = 48000000;
  #elif VVC_L0
    // Set the Flash ACR to use 1 wait-state
    // and enable the prefetch buffer and pre-read.
    FLASH->ACR |=  (FLASH_ACR_LATENCY |
                    FLASH_ACR_PRFTEN |
                    FLASH_ACR_PRE_READ);
    // Enable the HSI oscillator, since the L0 series boots
    // to the MSI one.
    RCC->CR    |=  (RCC_CR_HSION);
    while (!(RCC->CR & RCC_CR_HSIRDY)) {};
    // Configure the PLL to use HSI16 with a PLLDIV of
    // 2 and PLLMUL of 4.
    RCC->CFGR  &= ~(RCC_CFGR_PLLDIV |
                    RCC_CFGR_PLLMUL |
                    RCC_CFGR_PLLSRC);
    RCC->CFGR  |=  (RCC_CFGR_PLLDIV2 |
                    RCC_CFGR_PLLMUL4 |
                    RCC_CFGR_PLLSRC_HSI);
    // Enable the PLL and wait for it to stabilize.
    RCC->CR    |=  (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY)) {};
    // Select the PLL as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_PLL);
    while (!(RCC->CFGR & RCC_CFGR_SWS_PLL)) {};
    // Set the global clock speed variable.
    //core_clock_hz = 32000000;
  #endif

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
