#ifndef __START_H__
#define __START_H__
#include "mm.h"
#include "common.h"
extern void main(void);
typedef enum{
    PMP_R = (1L << 0),
    PMP_W = (1L << 1),
    PMP_X = (1L << 2),
    PMP_TOR =  (1L << 3),
    PMP_NAPOT = (3L << 3)
}pmp_t;

#endif //__START_H__