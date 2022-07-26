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

// Alt. Func. #
#ifdef VVC_F0
#define ALT_NO 1
#elif VVC_F3
#define ALT_NO 4
#elif VVC_L0
#define ALT_NO 1
#endif

// 128x64-pixel monochrome framebuffer.
#define SSD1306_W 128
#define SSD1306_H 64
#define SSD1306_A ( SSD1306_W * SSD1306_H ) / 8
uint8_t FRAMEBUFFER[ SSD1306_A ];
// Initialization commands for the SSD1306 display.
#define NUM_INIT_CMDS 25
const uint8_t INIT_CMDS[ NUM_INIT_CMDS ] = {
  // 0x00, to indicate command bytes.
  0x00,
  // Display clock division, multiplex (# rows)
  0xD5, 0x80, 0xA8, 0x3F,
  // Display offset, start line, charge pump on.
  0xD3, 0x00, 0x40, 0x8D, 0x14,
  // Memory mode, segment remap, desc. column scan.
  0x20, 0x00, 0xA1, 0xC8,
  // 'COMPINS', contrast.
  0xDA, 0x12, 0x81, 0x0A,
  // precharge, VCOM detection level.
  0xD9, 0xF1, 0xDB, 0x40,
  // Output follows RAM, normal mode, display on.
  0xA4, 0xA6, 0xAF
};

// Global variable to hold the core clock speed in Hertz.
uint32_t SystemCoreClock = 16000000;

// Simple imprecise delay method.
void __attribute__( ( optimize( "O0" ) ) )
delay_cycles( uint32_t cyc ) {
  for ( uint32_t d_i = 0; d_i < cyc; ++d_i ) { asm( "NOP" ); }
}

/**
 * Main program.
 */
int main(void) {
  // Enable peripherals: GPIOB, DMA, I2C1, SYSCFG.
  #ifdef VVC_F0
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
  #elif VVC_F3
    RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
  #elif VVC_L0
    RCC->IOPENR |= RCC_IOPENR_GPIOBEN;
  #endif
  RCC->AHBENR   |= RCC_AHBENR_DMA1EN;
  RCC->APB1ENR  |= RCC_APB1ENR_I2C1EN;
  RCC->APB2ENR  |= RCC_APB2ENR_SYSCFGEN;

  // Setup core clock to 16MHz.
  #ifdef VVC_F0
    // Reset the Flash 'Access Control Register', and
    // // then set 0 wait-states (default).
    // FLASH->ACR &= ~(0x00000017);
    // // Configure the PLL to (HSI / 2) * 4 = 16MHz.
    // // Use a PLLMUL of 0x2 for *4, and keep PLLSRC at 0
    // // to use (HSI / PREDIV) as the core source. HSI = 8MHz.
    // RCC->CFGR &= ~(RCC_CFGR_PLLMUL |
    //               RCC_CFGR_PLLSRC);
    // RCC->CFGR |= (RCC_CFGR_PLLSRC_HSI_DIV2 |
    //               RCC_CFGR_PLLMUL4);
    // // Turn the PLL on and wait for it to be ready.
    // RCC->CR |= (RCC_CR_PLLON);
    // while (!(RCC->CR & RCC_CR_PLLRDY))
    // {
    // };
    // // Select the PLL as the system clock source.
    // RCC->CFGR &= ~(RCC_CFGR_SW);
    // RCC->CFGR |= (RCC_CFGR_SW_PLL);
    // while (!(RCC->CFGR & RCC_CFGR_SWS_PLL))
    // {
    // };
    // // The system clock is now 16MHz.
    // SystemCoreClock = 16000000;
  #elif VVC_F3
    // Set 0 wait-states (default) and enable the prefetch buffer.
    FLASH->ACR |= (FLASH_ACR_PRFTBE);
    // Configure the PLL to (HSI / 2) * 4 = 16MHz.
    // Use a PLLMUL of 0x2 for *4, and keep PLLSRC at 0
    // to use (HSI / PREDIV) as the core source. HSI = 8MHz.
    RCC->CFGR &= ~(RCC_CFGR_PLLMUL |
                  RCC_CFGR_PLLSRC);
    RCC->CFGR |= (RCC_CFGR_PLLSRC_HSI_DIV2 |
                  RCC_CFGR_PLLMUL4);
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
    // The system clock is now 16MHz.
    SystemCoreClock = 16000000;
  #elif VVC_L0
    // Set voltage scaling range to Range 2 (default)
    // Note: VOS cannot be 00!
    PWR->CR = 0x00001000;
    // Set the Flash ACR to use 1 wait-state
    // and enable the prefetch buffer and pre-read.
    FLASH->ACR |=  (FLASH_ACR_LATENCY |
                    FLASH_ACR_PRFTEN |
                    FLASH_ACR_PRE_READ);
    // Enable the HSI oscillator, since the L0 series boots
    // to the MSI one.
    RCC->CR    |=  (RCC_CR_HSION);
    while (!(RCC->CR & RCC_CR_HSIRDY)) {};
    // Select the HSI16 as the system clock source.
    RCC->CFGR  &= ~(RCC_CFGR_SW);
    RCC->CFGR  |=  (RCC_CFGR_SW_HSI);
    while (!(RCC->CFGR & RCC_CFGR_SWS_HSI)) {};
    // Set the global clock speed variable.
    SystemCoreClock = 16000000;
  #endif

  // Pin B6/7 set to 'alt' mode, open-drain, with pull-up.
  GPIOB->MODER    &= ~( 0x3 << ( 6 * 2 ) |
                        0x3 << ( 7 * 2 ) );
  GPIOB->MODER    |=  ( 0x2 << ( 6 * 2 ) |
                        0x2 << ( 7 * 2 ) );
  GPIOB->AFR[ 0 ] &= ~( 0xF << ( 6 * 4 ) |
                        0xF << ( 7 * 4 ) );
  GPIOB->AFR[ 0 ] |=  ( ALT_NO << ( 6 * 4 ) |
                        ALT_NO << ( 7 * 4 ) );
  GPIOB->PUPDR    &= ~( 0x3 << ( 6 * 2 ) |
                        0x3 << ( 7 * 2 ) );
  GPIOB->OSPEEDR  |=  ( 0x3 << ( 6 * 2 ) |
                        0x3 << ( 7 * 2 ) );
  // GPIOB->PUPDR    |=  ( 0x1 << ( 6 * 2 ) |
  //                       0x1 << ( 7 * 2 ) );
  GPIOB->OTYPER   |=  ( 0x1 << 6 |
                        0x1 << 7 );

  // Enable the high sink driver capability by
  // setting I2C_PBx_FM+ bit in SYSCFG_CFGRy
  // #ifdef VVC_F0
  //   SYSCFG->CFGR1 |= (SYSCFG_CFGR1_I2C_FMP_PB6 |
  //                     SYSCFG_CFGR1_I2C_FMP_PB7);
  // #elif VVC_L0
  //   SYSCFG->CFGR2 |= (SYSCFG_CFGR2_I2C_PB6_FMP |
  //                     SYSCFG_CFGR2_I2C_PB7_FMP);
  // #else
  //   SYSCFG->CFGR1 |= (SYSCFG_CFGR1_I2C_PB6_FMP |
  //                     SYSCFG_CFGR1_I2C_PB7_FMP);
  // #endif

  // Set the 'I2C1_TX DMA remap' bit in SYSCFG_CFGR3,
  // so that I2C1_TX maps to DMA1_Ch2 instead of DMA1_Ch6 (default).
  // (Not all STM32F303 chips have a DMA2 peripheral)
  #ifdef VVC_F3
    SYSCFG->CFGR3 &= ~(SYSCFG_CFGR3_I2C1_TX_DMA_RMP);
    SYSCFG->CFGR3 |=  (SYSCFG_CFGR3_I2C1_TX_DMA_RMP_0);
  #endif

  // DMA configuration (channel 2).
  // CCR register:
  // - Memory-to-peripheral
  // - Circular mode disabled.
  // - Increment memory ptr, don't increment periph ptr.
  // - 8-bit data size for both source and destination.
  // - High priority.
  DMA1_Channel2->CCR &= ~( DMA_CCR_MEM2MEM |
                           DMA_CCR_PL |
                           DMA_CCR_MSIZE |
                           DMA_CCR_PSIZE |
                           DMA_CCR_PINC |
                           DMA_CCR_CIRC |
                           DMA_CCR_EN );
  DMA1_Channel2->CCR |=  ( ( 0x2 << DMA_CCR_PL_Pos ) |
                           DMA_CCR_MINC |
                           DMA_CCR_DIR );
  // Set DMA source and destination addresses.
  // Source: Address of the initialization commands.
  DMA1_Channel2->CMAR  = ( uint32_t )&INIT_CMDS;
  // Dest.: 'I2C1 transmit' register.
  DMA1_Channel2->CPAR  = ( uint32_t )&( I2C1->TXDR );
  // Set DMA data transfer length (# of init commands).
  DMA1_Channel2->CNDTR = ( uint16_t )NUM_INIT_CMDS;
  // Set DMA request mapping for I2C1_TX (L0-only)
  #ifdef VVC_L0
    DMA1_CSELR->CSELR = 0x6 << (4 * (2-1));
  #endif
  // Enable DMA1 Channel 1.
  DMA1_Channel2->CCR |= ( DMA_CCR_EN );

  // I2C1 configuration:
  // Timing register. For "Fast-Mode+" (1MHz), the RM says:
  // (@16MHz) presc=0, SCLL=4, SCLH=2, SDADEL=0, SCLDEL=2.
  // I2C1->TIMINGR  = 0x00200204;
  I2C1->CR1     &= ~I2C_CR1_PE;
  //I2C1->TIMINGR  = 0x30420F13; // Std mode (100kHz @ 16MHz)
  I2C1->TIMINGR  = 0x50100103; // Std mode (100kHz @ 8MHz)
  // Enable the peripheral.
  I2C1->CR1     |= I2C_CR1_PE;
  // Set the device address. Usually 0x78, can be 0x7A.
  // The I2C peripheral also needs to know how many bytes
  // to send before it starts transmitting.
  I2C1->CR2     &= ~( I2C_CR2_SADD |
                      I2C_CR2_NBYTES );
  I2C1->CR2     |=  ( 0x7A << I2C_CR2_SADD_Pos |
                      NUM_INIT_CMDS << I2C_CR2_NBYTES_Pos );
  // Enable I2C DMA requests.
  I2C1->CR1     |=  ( I2C_CR1_TXDMAEN );
  I2C1->CR1     |=  ( I2C_CR1_PE);
  // Send a start signal.
  I2C1->CR2     |=  ( I2C_CR2_START );
  // (DMA is now running.)
  // Wait for DMA to finish.
  while ( !( DMA1->ISR & DMA_ISR_TCIF1 ) ) {};
  DMA1->IFCR |= DMA_IFCR_CTCIF1;
  // Stop the I2C transmission.
  while ( !( I2C1->ISR & I2C_ISR_TC ) ) {};
  I2C1->CR2  |=  ( I2C_CR2_STOP );
  while ( I2C1->ISR & I2C_ISR_BUSY ) {};

  // Reconfigure DMA and I2C for sending the framebuffer.
  // Disable the DMA channel.
  DMA1_Channel2->CCR &= ~( DMA_CCR_EN );
  // Set DMA circular mode.
  DMA1_Channel2->CCR |=  ( DMA_CCR_CIRC );
  // Set I2C autoreload and the maximum 255 byte length.
  I2C1->CR2      &= ~( I2C_CR2_NBYTES );
  I2C1->CR2      |=  ( I2C_CR2_RELOAD |
                       255 << I2C_CR2_NBYTES_Pos );
  // Enable the I2C1 interrupt.
  NVIC_SetPriority( I2C1_IRQn, 0x03 );
  NVIC_EnableIRQ( I2C1_IRQn );
  // Enable the 'transfer complete' I2C interrupt.
  I2C1->CR1      |= I2C_CR1_TCIE;
  // Update DMA source/destination/size registers.
  // Source: Address of the framebuffer.
  DMA1_Channel2->CMAR  = ( uint32_t )&FRAMEBUFFER;
  // Dest.: 'I2C1 transmit' register.
  DMA1_Channel2->CPAR  = ( uint32_t )&( I2C1->TXDR );
  // Set DMA data transfer length (framebuffer length).
  DMA1_Channel2->CNDTR = ( uint16_t )SSD1306_A;
  // Send a start signal.
  I2C1->CR2     |=  ( I2C_CR2_START );
  while ( !( I2C1->CR2 & I2C_CR2_START ) ) {};
  // Send '0x40' to indicate display data.
  I2C1->TXDR = 0x40;
  // Re-enable DMA1 Channel 1.
  DMA1_Channel2->CCR |= ( DMA_CCR_EN );

  // Done; now draw patterns to the framebuffer.
  // The display is configured to hold 8 vertical pixels in
  // each byte, with the first 128 bytes representing
  // y-coordinates [0:7], the next 128 bytes [8:15], and so on.
  // So if we set each byte to the same value, it will look
  // like a pattern of horizontal lines of varying thickness.
  uint8_t val = 0x00;
  while (1) {
    // Draw the new pattern to the framebuffer.
    for ( size_t i = 0; i < SSD1306_A; ++i ) {
      FRAMEBUFFER[ i ] = val;
    }
    // Update the pattern.
    ++val;
    // Delay briefly.
    delay_cycles( 200000 );
  }
}

// I2C1 interrupt handler.
void I2C1_IRQ_handler( void ) {
  if ( I2C1->ISR & I2C_ISR_TCR ) {
    I2C1->CR2 &= ~( I2C_CR2_NBYTES );
    I2C1->CR2 |=  ( 255 << I2C_CR2_NBYTES_Pos );
  }
}
