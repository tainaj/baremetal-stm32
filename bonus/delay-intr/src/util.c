#include "util.h"

/**
 * @brief Bonus project: prove with an oscilloscope that sigfigs are being obeyed!
 * 
 */

/**
 * @brief Private variables.
 */

static volatile uint32_t _millis = 0;
static uint32_t _core_clock_mhz;

/**
 * @brief Interrupt service handlers (ISR)
 */

void SysTick_handler(void) {
    _millis++;
}

/**
 * @brief Public functions (Delay API)
 */

void systick_init(uint32_t _core_clock_hz) {
    SysTick_Config(_core_clock_hz / 1000);
    NVIC_SetPriority(SysTick_IRQn, 0);
    _core_clock_mhz  = _core_clock_hz / 1000000;
}

uint32_t millis(void) {
    return _millis;
}

/**
 * @brief Return time in micros (mod _max_micros)
 */
uint32_t micros(void) {
    return (uint32_t) ((_millis & 0xffff)*1000 + 1000 - (SysTick->VAL/_core_clock_mhz));
}

/**
 * @brief Delay time (d) with given micro time (margin of error = 0.875d to 1d)
 * 
 * @param delay   delay in us (keep below 40 seconds)
 * @param sig_fig number of sig figs required for accuracy (3000us -> 1, 3200us -> 2, 3230us -> 3 (max))
 */
void micro_wait(uint32_t delay, uint32_t sig_fig) {
    int i;
    uint32_t factor = 1;
    
    for (i=0; i<sig_fig; i++) {
        factor *= 10;
    }
    uint32_t delay_o = delay / factor;
    uint32_t time_start;
    for (i=0; i<factor; i++) {

        time_start = micros();
        while (micros() - time_start < delay_o);
    }
}

/**
 * @brief Delay time with given milli time
 * 
 * @param delay (keep below 46 days)
 *        
 */
void milli_wait(uint32_t delay) {
    uint32_t time_start = millis();
    while (millis() - time_start < delay);
}