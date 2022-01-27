# Bonus 2: DS3231 RTC clock
This repository is a mini-project based on Vivonomicon's exploration of the DS3231 RTC:

https://vivonomicon.com/2018/04/30/when-is-now-the-ds3231-real-time-clock/

## Hardware
For this mini-project, I used development boards for the STM32F051R8 and STM32L031K6 chips.

Components used for this project:
* [DS3231 module](https://www.amazon.com/dp/B07Q7NZTQS?psc=1&ref=ppx_yo2_dt_b_product_details) (from Amazon)

## Software
Firmware packages for STM32F051R8T6 and STM32L031K6T6 have been imported from the previous lab.

## Procedure
The following activity is featured in this project:
1. Connecting VCC, GND, SCL, and SDA pins of the DS3231 module to the microcontroller.
2. Configuring GPIO pins B6 and B7 to use the I2C1 peripheral.
3. Understanding `util.c` / `util.h` functions. Compare Vivonomicon's implimentation with other authors.
4. Testing 'optional' features, including EEPROM write and read.

Additional notes for each step is listed below:

### Connecting DS3231 pins to STM32
The blog mentions the use of a disk battery to provide backup power to the module. Some modules, like mine, can run entirely on microcontroller power and not even need the battery. The module I linked in **Hardware** is one of those modules.

### Configuring pins to I2C1
The source material enabled internal pull-ups (25-55kOhms) for SCL and SDA lines, which are adequate for standard I2C clock speeds. For higher speeds, use external pull-ups with lower resistance (1-4.7kOhms) to reduce the rise time of the I2C lines.

This project's I2C clock speed is 100kHz, considered standard speed.

### Understanding util.c and util.h
Compare Vivinomicon's implementation of the following I2C functions with other authors:

**i2c_init()** (written in main.c)

VVC wrote this function's logic as part of `main`. GPIOB pins B6 and B7 are enabled, set to I2C1. Bits in `I2C->CR1`, `I2C->CR2`, and `I2C->ISR` are cleared to return registers to their reset value 0x00000000.

The pdf and the textbook also set `AUTOEND` and `NACK` in `I2C1->CR2`:
* `AUTOEND`: STOP condition automatically set after `NBYTES` data sent. May make the setting of `I2C_CR2_STOP` in `i2c_stop()` redundant.
* `NACK`: This was added as a catch-all for any failure of an I2C write transfer. See `i2c_senddata()` on the pdf to see its purpose.

**i2c_start()**

VVC simply sets `I2C_CR2_START` and waits for it to acknowledge:
```c
// Send 'Start' condition, and wait for acknowledge.
I2C1->CR2 |=  (I2C_CR2_START);
while ((I2C1->CR2 & I2C_CR2_START)) {}
```
Note: `START` auto-clears when the START condition is generated on the I2C lines. Not sure if we have to wait for that to happen before writing a byte, but let's keep it to be on the safe side.

The pdf and textbook also clear some `CR2` bits, set R/W, specifies `SADD` and `NBYTES`:
```c
// dir: 0 = master requests a write transfer
// dir: 1 = master requests a read transfer
uint32_t tmpreg = I2C1->CR2;
tmpreg &= ~(I2C_CR2_SADD | I2C_CR2_NBYTES |
            I2C_CR2_RELOAD | I2C_CR2_AUTOEND |
            I2C_CR2_RD_WRN | I2C_CR2_START | I2C_CR2_STOP);
if (dir == 1)
    tmpreg |= I2C_CR2_RD_WRN; // Read from slave
else
    tmpreg &= I2C_CR2_RD_WRN; // Write to slave
tmpreg |= ((devaddr<<1) & I2C_CR2_SADD) | ((size << 16) & I2C_CR2_NBYTES);
tmpreg |= I2C_CR2_START;
I2C1->CR2 = tmpreg
```

**i2c_stop()**
VVC, the pdf, and the textbook generate the STOP bit, wait for acknowledgement, and clears the STOPF flag with `I2C_ICR_STOPCF`.

VVC
```C
inline void i2c_stop(void) {
  // Send 'Stop' condition, and wait for acknowledge.
  I2C1->CR2 |=  (I2C_CR2_STOP);
  while ((I2C1->CR2 & I2C_CR2_STOP)) {}
  // Reset the ICR ('Interrupt Clear Register') event flag.
  I2C1->ICR |=  (I2C_ICR_STOPCF);
  while ((I2C1->ICR & I2C_ICR_STOPCF)) {}
}
```
Note: Generating a STOP condition causes `I2C_ISR_STOPF` to set when detected on the lines. Shouldn't the while loop wait for STOPF to set, so we guarantee `I2C_ICR_STOPCF` will clear it?

Pdf
```C
void i2c_stop(void) {
  if (I2C1->ISR & I2C_ISR_STOPF)
    return;
  // Master: Generate STOP bit after current byte has been transferred.
  I2C1->CR2 |= I2C_CR2_STOP;
  // Wait until STOPF flag is reset
  while( (I2C1->ISR & I2C_ISR_STOPF) == 0);
  I2C1->ICR |= I2C_ICR_STOPCF; // Write to clear STOPF flag
}
```
Note: Adding a return condition for an early STOPF is dangerous, especially if `I2C_CR2_AUTOEND` is set. What if the last byte transfer generates the auto STOP condition, triggering the STOPF flag before entering `i2c_stop`()? Then STOPF will not be cleared! Instead, I recommend the following:
```C
void i2c_stop(void) {
  if ((I2C1->ISR & I2C_ISR_STOPF) == 0) {
    // Generate STOP bit, wait until STOPF flag is reset
  }
  I2C1->ICR |= I2C_ICR_STOPCF; // Write to clear STOPF flag
}
```
Textbook is the same as the pdf, but removes the if statement. This is ideal when the AUTOEND flag is cleared, however.

### i2c_write_byte() and i2c_read_byte()
VVC reads and writes data one byte for each function call. After writing the byte to `I2C1->TXDR`, it checks the `I2C1->ISR` register to see if `TXIS` (ready for next byte) or `TC` (transfer complete) sets. The helper function `i2c_read_register()` reads a byte from an address `reg_addr` by first writing 0 bytes while also moving the read pointer to the desired address, and then reading a byte at that address.

The pdf and textbook's implementation of read and write can handle multiple-byte reads. At the end of each data transfer, they wait for either `TC` to set, indicating success, or `NACKF` to set, indicating failure.

## Testing optional features
After writing 'Hello, World!' to the EEPROM, reading the EEPROM should return the same string.

In `gdb-multiarch`, to read the value of `eeprom_str` containing the string read, you must load and let continue the program for some time before hitting `CTRL-C`, and then entering `u` several times until you reach the `main` stackframe, in which the variable `eeprom_str` is stored.

## Author's notes 
See my future blog to see comparisions in much greater detail than I am willing to provide in this README.
