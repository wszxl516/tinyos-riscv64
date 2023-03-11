#ifndef __BOARD_H__
#define __BOARD_H__
#include "common.h"


#define SHUTDONW() REG_WRITE32(SYSCON_POWEROFF, POWEROFF_CODE)

#define REBOOT() REG_WRITE32(SYSCON_REBOOT, REBOOT_CODE)

#endif //__BOARD_H__