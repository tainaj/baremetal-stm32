// Standard library includes.
#include <stdint.h>
#include <stdlib.h>
// Vendor-provided device header file.
#ifdef VVC_F0
#include "stm32f0xx.h"
#elif VVC_F3
#include "stm32f3xx.h"
#elif VVC_L0
#include "stm32l0xx.h"
#endif

// Configurable NZR settings
#ifdef VVC_L0
#define NZR_BIT_0 0xC0
#define NZR_BIT_1 0xE0
#else
#define NZR_BIT_0 0xE0
#define NZR_BIT_1 0xF8
#endif

#define NZR_RST_PULSE 192

// Array of LED colors. G/R/B/G/R/B/...
#define NUM_LEDS (3)
#define LED_BYTES ((NUM_LEDS * 3 * 8) + NZR_RST_PULSE)
uint8_t COLORS[LED_BYTES];

// Global variable to hold the core clock speed in Hertz.
uint32_t SystemCoreClock = 16000000;

// Simple imprecise delay method.
void __attribute__((optimize("O0")))
delay_cycles(uint32_t cyc)
{
  for (uint32_t d_i = 0; d_i < cyc; ++d_i)
  {
    asm("NOP");
  }
}

uint32_t get_rgb_color(uint8_t r, uint8_t g, uint8_t b)
{
  return (g << 16 | r << 8 | b);
}

void set_color(size_t led_index, uint32_t col)
{
  size_t led_base = led_index * 24;
  uint8_t r = (col >> 8) & 0xFF;
  uint8_t g = (col >> 16) & 0xFF;
  uint8_t b = (col)&0xFF;
  for (size_t i = 0; i < 8; ++i)
  {
    if (g & (1 << (7 - i)))
    {
      COLORS[i + led_base] = NZR_BIT_1;
    }
    else
    {
      COLORS[i + led_base] = NZR_BIT_0;
    }
  }
  for (size_t i = 0; i < 8; ++i)
  {
    if (r & (1 << (7 - i)))
    {
      COLORS[i + led_base + 8] = NZR_BIT_1;
    }
    else
    {
      COLORS[i + led_base + 8] = NZR_BIT_0;
    }
  }
  for (size_t i = 0; i < 8; ++i)
  {
    if (b & (1 << (7 - i)))
    {
      COLORS[i + led_base + 16] = NZR_BIT_1;
    }
    else
    {
      COLORS[i + led_base + 16] = NZR_BIT_0;
    }
  }
}

// Get the red component of an LED color.
uint8_t get_led_r(size_t led_num)
{
  uint8_t r = 0x00;
  for (size_t i = 0; i < 8; ++i)
  {
    if (COLORS[(led_num * 24) + 8 + i] != NZR_BIT_0)
    {
      r = r | (1 << (7 - i));
    }
  }
  return r;
}

// Get the green component of an LED color.
uint8_t get_led_g(size_t led_num)
{
  uint8_t g = 0x00;
  for (size_t i = 0; i < 8; ++i)
  {
    if (COLORS[(led_num * 24) + i] != NZR_BIT_0)
    {
      g = g | (1 << (7 - i));
    }
  }
  return g;
}

// Get the blue component of an LED color.
uint8_t get_led_b(size_t led_num)
{
  uint8_t b = 0x00;
  for (size_t i = 0; i < 8; ++i)
  {
    if (COLORS[(led_num * 24) + 16 + i] != NZR_BIT_0)
    {
      b = b | (1 << (7 - i));
    }
  }
  return b;
}

// Max brightness (out of a possible 255)
#define MAX_B (63)
// How quickly to increment/decrement the colors.
#define B_INC (1)
// Cycle the array of colors through a rainbow.
// Red -> Purple -> Blue -> Teal -> Green -> Yellow -> Red
// - If red > 0 and < max, if blue is 0, add red.
// - If red is max and blue is < max, add blue.
// - If blue is max and red is > 0, remove red.
// - If blue is max and green < 0, add green.
// - If green is max and blue > 0, remove blue.
// - If green is max and red < 0, add red.
// - If red is max and green is > 0, remove green.
void rainbow(void)
{
  uint8_t r = get_led_r(0);
  uint8_t g = get_led_g(0);
  uint8_t b = get_led_b(0);
  for (int i = 0; i < NUM_LEDS; ++i)
  {
    if (r == 0 && g == 0 && b == 0)
    {
      r = B_INC;
    }
    if (r > 0 && r < MAX_B && b == 0)
    {
      r += B_INC;
    }
    else if (r >= MAX_B && b < MAX_B && g == 0)
    {
      b += B_INC;
    }
    else if (b >= MAX_B && r > 0)
    {
      r -= B_INC;
    }
    else if (b >= MAX_B && g < MAX_B)
    {
      g += B_INC;
    }
    else if (g >= MAX_B && b > 0)
    {
      b -= B_INC;
    }
    else if (g >= MAX_B && r < MAX_B)
    {
      r += B_INC;
    }
    else if (r >= MAX_B && g > 0)
    {
      g -= B_INC;
    }
    else
    {
      r = 0;
      g = 0;
      b = 0;
    }
    set_color(i, get_rgb_color(r, g, b));
  }
}

/**
 * Main program.
 */
int main(void)
{
  // Set initial colors to 'off'.
  for (size_t i = 0; i < NUM_LEDS; ++i)
  {
    set_color(i, get_rgb_color(0x00, 0x00, 0x00));
  }
  // Set the latching period to all 0s.
  for (size_t i = LED_BYTES - NZR_RST_PULSE; i < LED_BYTES; ++i)
  {
    COLORS[i] = 0x00;
  }
  // Enable peripherals: GPIOB, DMA, SPI1.
  // GPIOB, DMA1, SPI1
  #ifdef VVC_F0
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
  #elif VVC_F3
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
  #elif VVC_L0
    RCC->IOPENR |= RCC_IOPENR_GPIOBEN;
  #endif
  RCC->AHBENR |= RCC_AHBENR_DMA1EN;
  RCC->APB2ENR |= RCC_APB2ENR_SPI1EN;

  // Setup core clock to 48MHz (32MHz for L0).
  #ifdef VVC_F0
    // Reset the Flash 'Access Control Register', and
    // then set 1 wait-state and enable the prefetch buffer.
    // (The device header files only show 1 bit for the F0
    //  line, but the reference manual shows 3...)
    FLASH->ACR &= ~(0x00000017);
    FLASH->ACR |= (FLASH_ACR_LATENCY |
                  FLASH_ACR_PRFTBE);
    // Configure the PLL to (HSI / 2) * 12 = 48MHz.
    // Use a PLLMUL of 0xA for *12, and keep PLLSRC at 0
    // to use (HSI / PREDIV) as the core source. HSI = 8MHz.
    RCC->CFGR &= ~(RCC_CFGR_PLLMUL |
                  RCC_CFGR_PLLSRC);
    RCC->CFGR |= (RCC_CFGR_PLLSRC_HSI_DIV2 |
                  RCC_CFGR_PLLMUL12);
    // Turn the PLL on and wait for it to be ready.
    RCC->CR |= (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY))
    {
    };
    // Select the PLL as the system clock source.
    RCC->CFGR &= ~(RCC_CFGR_SW);
    RCC->CFGR |= (RCC_CFGR_SW_PLL);
    while (!(RCC->CFGR & RCC_CFGR_SWS_PLL))
    {
    };
    // The system clock is now 48MHz.
    SystemCoreClock = 48000000;
  #elif VVC_F3
    // Set 1 wait-state and enable the prefetch buffer.
    FLASH->ACR |= (FLASH_ACR_LATENCY_0 |
                  FLASH_ACR_PRFTBE);
    // Configure the PLL to (HSI / 2) * 12 = 48MHz.
    // Use a PLLMUL of 0xA for *12, and keep PLLSRC at 0
    // to use (HSI / PREDIV) as the core source. HSI = 8MHz.
    RCC->CFGR &= ~(RCC_CFGR_PLLMUL |
                  RCC_CFGR_PLLSRC);
    RCC->CFGR |= (RCC_CFGR_PLLSRC_HSI_DIV2 |
                  RCC_CFGR_PLLMUL12);
    // Turn the PLL on and wait for it to be ready.
    RCC->CR |= (RCC_CR_PLLON);
    while (!(RCC->CR & RCC_CR_PLLRDY))
    {
    };
    // Select the PLL as the system clock source.
    RCC->CFGR &= ~(RCC_CFGR_SW);
    RCC->CFGR |= (RCC_CFGR_SW_PLL);
    while (!(RCC->CFGR & RCC_CFGR_SWS_PLL))
    {
    };
    // The system clock is now 48MHz.
    SystemCoreClock = 48000000;
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
    SystemCoreClock = 32000000;
  #endif

  // Setup pin: PB5 is AF#0 (SPI1 SDO), or AF#5 for F3.
  #ifdef VVC_F0
    GPIOB->MODER &=  ~(0x3 << (5 * 2));
    GPIOB->MODER |=   (0x2 << (5 * 2));
    GPIOB->AFR[0] &= ~(0xF << (5 * 4));
  #elif VVC_F3
    GPIOB->MODER &=  ~(0x3 << (5 * 2));
    GPIOB->MODER |=   (0x2 << (5 * 2));
    GPIOB->AFR[0] &= ~(0xF << (5 * 4));
    GPIOB->AFR[0] |=  (0x5 << (5 * 4));
  #elif VVC_L0
    GPIOB->MODER &=  ~(0x3 << (5 * 2));
    GPIOB->MODER |=   (0x2 << (5 * 2));
    GPIOB->AFR[0] &= ~(0xF << (5 * 4));
  #endif

  // DMA configuration (channel 3).
  // CCR register:
  // - Memory-to-peripheral
  // - Circular mode enabled.
  // - Increment memory ptr, don't increment periph ptr.
  // - 8-bit data size for both source and destination.
  // - High priority.
  DMA1_Channel3->CCR &= ~(DMA_CCR_MEM2MEM |
                          DMA_CCR_PL |
                          DMA_CCR_MSIZE |
                          DMA_CCR_PSIZE |
                          DMA_CCR_PINC |
                          DMA_CCR_EN);
  DMA1_Channel3->CCR |= ((0x2 << DMA_CCR_PL_Pos) |
                         DMA_CCR_MINC |
                         DMA_CCR_CIRC |
                         DMA_CCR_DIR);
  // Set DMA source and destination addresses.
  // Source: Address of the framebuffer.
  DMA1_Channel3->CMAR = ( uint32_t )&COLORS;
  // Destination: SPI1 data register.
  DMA1_Channel3->CPAR = ( uint32_t ) & (SPI1->DR);
  // Set DMA data transfer length (framebuffer length).
  DMA1_Channel3->CNDTR = ( uint16_t )LED_BYTES;
  // Set DMA request mapping for SPI1_TX (L0-only)
  #ifdef VVC_L0
    DMA1_CSELR->CSELR = 1 << (4 * (3-1));
  #endif

  // SPI1 configuration:
  // - Clock phase/polarity: 1/1
  // - Assert internal CS signal (software CS pin control)
  // - MSB-first
  // - 8-bit frames
  // - Baud rate prescaler of 8 (for a 6MHz bit-clock, 4MHz for L0)
  // - TX DMA requests enabled.
  SPI1->CR1 &= ~(SPI_CR1_LSBFIRST |
                 SPI_CR1_BR);
  SPI1->CR1 |= (SPI_CR1_SSM |
                SPI_CR1_SSI |
                0x2 << SPI_CR1_BR_Pos |
                SPI_CR1_MSTR |
                SPI_CR1_CPOL |
                SPI_CR1_CPHA);
  SPI1->CR2 |=  (SPI_CR2_TXDMAEN);
  #ifdef VVC_F0
    SPI1->CR2 &= ~(SPI_CR2_DS);
    SPI1->CR2 |= (0x7 << SPI_CR2_DS_Pos);
  #elif VVC_F3
    SPI1->CR2 &= ~(SPI_CR2_DS);
    SPI1->CR2 |= (0x7 << SPI_CR2_DS_Pos);
  #elif VVC_L0
    SPI1->CR1 &= ~(SPI_CR1_DFF);
  #endif

  // Enable the SPI peripheral.
  SPI1->CR1 |= (SPI_CR1_SPE);

  // Enable DMA1 Channel 1 to start sending colors.
  DMA1_Channel3->CCR |= (DMA_CCR_EN);

  // Done; now just cycle between colors.
  while (1)
  {
    rainbow();
    delay_cycles(10000);
  }
}
