#include "peripherals.h"

/* Timer Peripherals */
/*
 * 'Start Basic Timer'
 * This method starts a new 'basic' timer, which triggers
 * an 'UPDATE' interrupt every time that it triggers.
 * It should trigger every N milliseconds.
 */
void start_timer(TIM_TypeDef *TIMx, uint16_t ms) {
  // Start by making sure the timer's 'counter' is off.
  TIMx->CR1 &= ~(TIM_CR1_CEN);
  // Next, reset the peripheral. (This is where a HAL can help)
  if (TIMx == TIM2) {
    RCC->APB1RSTR |=  (RCC_APB1RSTR_TIM2RST);
    RCC->APB1RSTR &= ~(RCC_APB1RSTR_TIM2RST);
  }
  #ifdef VVC_F0
  else if (TIMx == TIM14) {
    RCC->APB1RSTR |=  (RCC_APB1RSTR_TIM14RST);
    RCC->APB1RSTR &= ~(RCC_APB1RSTR_TIM14RST);
  }
  else if (TIMx == TIM16) {
    RCC->APB2RSTR |=  (RCC_APB2RSTR_TIM16RST);
    RCC->APB2RSTR &= ~(RCC_APB2RSTR_TIM16RST);
  }
  else if (TIMx == TIM17) {
    RCC->APB2RSTR |=  (RCC_APB2RSTR_TIM17RST);
    RCC->APB2RSTR &= ~(RCC_APB2RSTR_TIM17RST);
  }
  #elif VVC_L0
  if (TIMx == TIM21) {
    RCC->APB2RSTR |=  (RCC_APB2RSTR_TIM21RST);
    RCC->APB2RSTR &= ~(RCC_APB2RSTR_TIM21RST);
  }
  if (TIMx == TIM22) {
    RCC->APB2RSTR |=  (RCC_APB2RSTR_TIM22RST);
    RCC->APB2RSTR &= ~(RCC_APB2RSTR_TIM22RST);
  }
  #endif

  // Bonus: AF0. GPIOA (CH1 = PA2, CH2 = PA3)
  // Enable the GPIOB clock.
  /*RCC->IOPENR   |= RCC_IOPENR_IOPAEN;

  GPIOA->AFR[TIM_X_CCR_PIN_1/8] &= ~(0xF << ((TIM_X_CCR_PIN_1 % 8)*4));
  GPIOA->AFR[TIM_X_CCR_PIN_1/8] |=  (TIM_X_CCR_AFy << ((TIM_X_CCR_PIN_1 % 8)*4));

  GPIOA->MODER  &= ~(0x3 << (TIM_X_CCR_PIN_1*2));
  GPIOA->MODER  |=  (0x2 << (TIM_X_CCR_PIN_1*2));
  GPIOA->PUPDR  &= ~(0x3 << (TIM_X_CCR_PIN_1*2));
  GPIOA->OTYPER &= ~(1 << TIM_X_CCR_PIN_1);

  GPIOA->AFR[TIM_X_CCR_PIN_2/8] &= ~(0xF << ((TIM_X_CCR_PIN_2 % 8)*4));
  GPIOA->AFR[TIM_X_CCR_PIN_2/8] |=  (TIM_X_CCR_AFy << ((TIM_X_CCR_PIN_2 % 8)*4));

  GPIOA->MODER  &= ~(0x3 << (TIM_X_CCR_PIN_2*2));
  GPIOA->MODER  |=  (0x2 << (TIM_X_CCR_PIN_2*2));
  GPIOA->PUPDR  &= ~(0x3 << (TIM_X_CCR_PIN_2*2));
  GPIOA->OTYPER &= ~(1 << TIM_X_CCR_PIN_2);*/

  // Set the timer prescaler/autoreload timing registers.
  // (These are 16-bit timers, so this won't work with >65MHz.)
  TIMx->PSC   = core_clock_hz / 1000;
  TIMx->ARR   = ms;
  // Send an update event to reset the timer and apply settings.
  TIMx->EGR  |= TIM_EGR_UG;
  // Enable the hardware interrupt.
  TIMx->DIER |= TIM_DIER_UIE;

  // Bonus: CCR1, CCR2 = 333ms, 666ms
  /*TIMx->CCR1  = 333-1;
  TIMx->CCMR1 |= TIM_CCMR1_OC1M_0 | TIM_CCMR1_OC1M_1;
  TIMx->CCER  |= TIM_CCER_CC1E;
  TIMx->CCR2  = 666-1;
  TIMx->CCMR1 |= TIM_CCMR1_OC2M_0 | TIM_CCMR1_OC2M_1;
  TIMx->CCER  |= TIM_CCER_CC2E;*/

  // Enable the timer.
  TIMx->CR1  |= TIM_CR1_CEN;
}

/*
 * Stop a running timer on a given Timer peripheral.
 */
void stop_timer(TIM_TypeDef *TIMx) {
  // Turn off the timer's 'counter'.
  TIMx->CR1 &= ~(TIM_CR1_CEN);
  // Clear the 'pending update interrupt' flag.
  TIMx->SR  &= ~(TIM_SR_UIF);
}
