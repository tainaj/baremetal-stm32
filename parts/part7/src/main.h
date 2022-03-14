#ifndef _STARm_MAIN_H
#define _STARm_MAIN_H

// Project includes.
#include "global.h"
#include "core.h"
#include "gpio.h"
#include "i2c.h"
#include "ssd1306.h"
#include "util.h"

// C++ memory regions for initializing statics.
// ('fini_array' is ignored because static destructors are rarely
//  important in embedded programs, which never really 'exit'.)
extern void (*_spreinit_array []) (void) __attribute__((weak));
extern void (*_epreinit_array [])(void) __attribute__((weak));
extern void (*_sinit_array [])(void) __attribute__((weak));
extern void (*_einit_array [])(void) __attribute__((weak));

// C++ pure virtual functions need the error handler
// '__cxa_pure_virtual' to invoke when called.
extern "C" void __cxa_pure_virtual() { while (1); }

#endif
