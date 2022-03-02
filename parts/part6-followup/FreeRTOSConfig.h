/*
 * FreeRTOS V202112.00
 * Copyright (C) 2020 Amazon.com, Inc. or its affiliates.  All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * http://www.FreeRTOS.org
 * http://aws.amazon.com/freertos
 *
 * 1 tab == 4 spaces!
 */


#ifndef FREERTOS_CONFIG_H
#define FREERTOS_CONFIG_H

/*-----------------------------------------------------------
 * Application specific definitions.
 *
 * These definitions should be adjusted for your particular hardware and
 * application requirements.
 *
 * THESE PARAMETERS ARE DESCRIBED WITHIN THE 'CONFIGURATION' SECTION OF THE
 * FreeRTOS API DOCUMENTATION AVAILABLE ON THE FreeRTOS.org WEB SITE.
 *
 * See http://www.freertos.org/a00110.html
 *----------------------------------------------------------*/

/* Ensure stdint is only used by the compiler, and not the assembler. */
#ifdef __ICCARM__
	#include <stdint.h>
	extern uint32_t SystemCoreClock;
#endif

#define configUSE_PREEMPTION                    1
#define configUSE_PORT_OPTIMISED_TASK_SELECTION 0 /* Default */
#define configUSE_TICKLESS_IDLE                 0 /* Default */
#ifdef VVC_F0
  #define configCPU_CLOCK_HZ              ( ( unsigned long ) 48000000 )  
  #define configSYSTICK_CLOCK_HZ          ( configCPU_CLOCK_HZ / 8 )
#elif  VVC_F1
  #define configCPU_CLOCK_HZ              ( ( unsigned long ) 72000000 )  
  #define configSYSTICK_CLOCK_HZ          ( configCPU_CLOCK_HZ / 8 )
#elif  VVC_F3
  #define configCPU_CLOCK_HZ              ( ( unsigned long ) 8000000 )  
  #define configSYSTICK_CLOCK_HZ          ( configCPU_CLOCK_HZ / 8 )
#elif  VVC_L0
  #define configCPU_CLOCK_HZ              ( ( unsigned long ) 32000000 )  
  #define configSYSTICK_CLOCK_HZ          ( configCPU_CLOCK_HZ / 8 )
#endif
#define configTICK_RATE_HZ                      1000 // changed
#define configMAX_PRIORITIES                    5
#define configMINIMAL_STACK_SIZE                60 // changed
#define configMAX_TASK_NAME_LEN                 5 // changed, default is 16
#define configUSE_16_BIT_TICKS                  0
#define configIDLE_SHOULD_YIELD                 1 /* Default */
#define configUSE_TASK_NOTIFICATIONS            0 /* Default is 1 */
#define configTASK_NOTIFICATION_ARRAY_ENTRIES   1 /* Default */
#define configUSE_MUTEXES                       1 // changed, default is 0
#define configUSE_RECURSIVE_MUTEXES             1 // changed, default is 0
#define configUSE_COUNTING_SEMAPHORES           1 // changed, default is 0
#define configUSE_ALTERNATIVE_API               0 /* Default, deprecated */
#define configQUEUE_REGISTRY_SIZE               8  /* Default is 0 */
#define configUSE_QUEUE_SETS                    0 /* Default */
#define configUSE_TIME_SLICING                  0 /* Default is 1 */
#define configUSE_NEWLIB_REENTRANT              0 /* Default */
#define configENABLE_BACKWARD_COMPATIBILITY     0 /* Default is 1 */
#define configNUM_THREAD_LOCAL_STORAGE_POINTERS 5 /* Default is 0 */
#define configSTACK_DEPTH_TYPE                  uint16_t /* Default */
#define configMESSAGE_BUFFER_LENGTH_TYPE        size_t /* Default */

/* Memory allocation related definitions. */
#define configSUPPORT_STATIC_ALLOCATION         0 /* Default */
#define configSUPPORT_DYNAMIC_ALLOCATION        1 /* Default */
#if defined(VVC_F0) || defined(VVC_L0)
  #define configTOTAL_HEAP_SIZE           ( ( size_t ) ( 2 * 1024 ) )
#else
  #define configTOTAL_HEAP_SIZE           ( ( size_t ) ( 8 * 1024 ) )
#endif
#define configAPPLICATION_ALLOCATED_HEAP        0 /* Default */
#define configSTACK_ALLOCATION_FROM_SEPERATE_HEAP 0 /* Default */

/* Hook function related definitions. */
#define configUSE_IDLE_HOOK                     0
#define configUSE_TICK_HOOK                     1 // changed
#define configCHECK_FOR_STACK_OVERFLOW          2 // changed, default is 0
#define configUSE_MALLOC_FAILED_HOOK            1 // changed, default is 0
#define configUSE_DAEMON_TASK_STARTUP_HOOK      0 /* Default */

/* Run time and task stats gathering related definitions. */
#define configGENERATE_RUN_TIME_STATS           0
#define configUSE_TRACE_FACILITY                1 // changed
#define configUSE_STATS_FORMATTING_FUNCTIONS    0

/* Co-routine related definitions. */
#define configUSE_CO_ROUTINES                   0
#define configMAX_CO_ROUTINE_PRIORITIES         2 // changed

/* Software timer related definitions. */
#define configUSE_TIMERS                        1 // changed
#define configTIMER_TASK_PRIORITY               2 // changed
#define configTIMER_QUEUE_LENGTH                5 // changed
#define configTIMER_TASK_STACK_DEPTH            80 // changed, default was MINIMAL_STACK_SIZE

/* Interrupt nesting behaviour configuration. */
/* Cortex-M specific definitions. */
#ifdef __NVIC_PRIO_BITS
  #define configPRIO_BITS         __NVIC_PRIO_BITS
#else
  #define configPRIO_BITS         4
#endif
#define configLIBRARY_LOWEST_INTERRUPT_PRIORITY      15 /* Only used by STM32F303 */
#define configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY 5  /* Only used by STM32F303 */
#define configKERNEL_INTERRUPT_PRIORITY   ( configLIBRARY_LOWEST_INTERRUPT_PRIORITY << (8 - configPRIO_BITS) )
/* !!!! configMAX_SYSCALL_INTERRUPT_PRIORITY must not be set to zero !!!!
See http://www.FreeRTOS.org/RTOS-Cortex-M3-M4.html. */
#define configMAX_SYSCALL_INTERRUPT_PRIORITY  ( configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY << (8 - configPRIO_BITS) )
//#define configMAX_API_CALL_INTERRUPT_PRIORITY   ( configMAX_SYSCALL_INTERRUPT_PRIORITY ) /* Only used by newer ports */

/* Define to trap errors during development. */
/* Normal assert() semantics without relying on the provision of an assert.h
header file. */
#define configASSERT( x ) if( ( x ) == 0 ) { taskDISABLE_INTERRUPTS(); for( ;; ); }

/* FreeRTOS MPU specific definitions. */
#define configINCLUDE_APPLICATION_DEFINED_PRIVILEGED_FUNCTIONS  0
//#define configTOTAL_MPU_REGIONS                                 8 /* Default value. */
//#define configTEX_S_C_B_FLASH                                   0x07UL  /* Default value. */
//#define configTEX_S_C_B_SRAM                                    0x07UL  /* Default value. */
//#define configENFORCE_SYSTEM_CALLS_FROM_KERNEL_ONLY             1
//#define configALLOW_UNPRIVILEGED_CRITICAL_SECTIONS              1

/* ARMv8-M secure side port related functions. */
//#define secureconfigMAX_SECURE_CONTEXTS         8 /* Default value. */

/* Optional functions - most linkers will remove unused functions anyway. */
#define INCLUDE_vTaskPrioritySet                1 // added
#define INCLUDE_uxTaskPriorityGet               1 // added
#define INCLUDE_vTaskDelete                     1 // added
#define INCLUDE_vTaskCleanUpResources			1 // added, fresh
#define INCLUDE_vTaskSuspend                    1 // added
#define INCLUDE_xResumeFromISR                  0
#define INCLUDE_vTaskDelayUntil                 1 // added
#define INCLUDE_vTaskDelay                      1
#define INCLUDE_xTaskGetSchedulerState          0
#define INCLUDE_xTaskGetCurrentTaskHandle       0
#define INCLUDE_uxTaskGetStackHighWaterMark     0
#define INCLUDE_xTaskGetIdleTaskHandle          0
#define INCLUDE_eTaskGetState                   0
#define INCLUDE_xEventGroupSetBitFromISR        0
#define INCLUDE_xTimerPendFunctionCall          0
#define INCLUDE_xTaskAbortDelay                 0
#define INCLUDE_xTaskGetHandle                  0
#define INCLUDE_xTaskResumeFromISR              0

/* Redirect FreeRTOS port interrupts. */
#define vPortSVCHandler     SVC_handler
#define xPortPendSVHandler  pending_SV_handler
#define xPortSysTickHandler SysTick_handler

#endif /* FREERTOS_CONFIG_H */