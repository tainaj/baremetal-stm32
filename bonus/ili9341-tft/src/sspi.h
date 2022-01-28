#ifndef _VVC_SSPI_H
#define _VVC_SSPI_H

#include "global.h"

// Define GPIOB pin mappings for software '4-wire' SPI.
#define PB_MOSI (5)
#define PB_SCK  (3)
#define PB_DC   (4)
#define PA_CS   (4)
#define PA_RST  (5)

/* Software SPI functions. */
// Write a byte of data using software SPI.
inline void sspi_w(uint8_t dat) {
  uint8_t sspi_i;
  // Send 8 bits, with the MSB first.
  for (sspi_i = 0x80; sspi_i != 0x00; sspi_i >>= 1) {
    GPIOB->ODR &= ~(1 << PB_SCK);
    if (dat & sspi_i) {
      GPIOB->ODR |=  (1 << PB_MOSI);
    }
    else {
      GPIOB->ODR &= ~(1 << PB_MOSI);
    }
    GPIOB->ODR |=  (1 << PB_SCK);
  }
}
// Write a 'command' byte for 4-wire SPI interfaces.
inline void sspi_cmd(uint8_t cdat) {
  // Pull the 'D/C' pin low, write data, pull 'D/C' high.
  GPIOB->ODR &= ~(1 << PB_DC);
  sspi_w(cdat);
  GPIOB->ODR |=  (1 << PB_DC);
}

/* Hardware SPI functions. */
void hspi_init(SPI_TypeDef *SPIx);

inline void hspi_w8(SPI_TypeDef *SPIx, uint8_t dat) {
  // Wait for TXE.
  while (!(SPIx->SR & SPI_SR_TXE)) {};
  // Send the byte.
  *(uint8_t*)&(SPIx->DR) = dat;
}

inline void hspi_w16(SPI_TypeDef *SPIx, uint16_t dat) {
#ifdef VVC_F0
  // Wait for TXE.
  while (!(SPIx->SR & SPI_SR_TXE)) {};
  // Send the data.
  // (Flip the bytes for the little-endian ARM core.)
  dat = (((dat & 0x00FF) << 8) | ((dat & 0xFF00) >> 8));
  *(uint16_t*)&(SPIx->DR) = dat;
#elif  VVC_L0
  hspi_w8(SPIx, (uint8_t)(dat >> 8));
  hspi_w8(SPIx, (uint8_t)(dat & 0xFF));
#endif
}

inline void hspi_cmd(SPI_TypeDef *SPIx, uint8_t cmd) {
  while ((SPIx->SR & SPI_SR_BSY)) {};
  GPIOB->ODR &= ~(1 << PB_DC);
  hspi_w8(SPIx, cmd);
  while ((SPIx->SR & SPI_SR_BSY)) {};
  GPIOB->ODR |=  (1 << PB_DC);
}

#endif
