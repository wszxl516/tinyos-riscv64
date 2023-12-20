.option norvc
.altmacro
.set NUM_G_REG, 32
.set REG_SIZE, 8

.macro SAVE_XN n, base
    sd x\n, (\n -1 )*REG_SIZE(\base)
.endm

.macro LOAD_XN n, base
    ld x\n , (\n -1 )*REG_SIZE(\base)
.endm


.macro PUTC n
    li      t0, 0x10000000
    li      t1, \n
    sd      t1, 0(t0)
.endm


.macro SAVE_G_REG base
    SAVE_XN 1, \base
    SAVE_XN 2, \base
    //ignore gp tp
    SAVE_XN 5, \base
    SAVE_XN 6, \base
    SAVE_XN 7, \base
    SAVE_XN 8, \base
    SAVE_XN 9, \base
    SAVE_XN 10, \base
    SAVE_XN 11, \base
    SAVE_XN 12, \base
    SAVE_XN 13, \base
    SAVE_XN 14, \base
    SAVE_XN 15, \base
    SAVE_XN 16, \base
    SAVE_XN 17, \base
    SAVE_XN 18, \base
    SAVE_XN 19, \base
    SAVE_XN 20, \base
    SAVE_XN 21, \base
    SAVE_XN 22, \base
    SAVE_XN 23, \base
    SAVE_XN 24, \base
    SAVE_XN 25, \base
    SAVE_XN 26, \base
    SAVE_XN 27, \base
    SAVE_XN 28, \base
    SAVE_XN 29, \base
    SAVE_XN 30, \base
    SAVE_XN 31, \base
.endm

.macro REST_G_REG base
    LOAD_XN 1, \base
    LOAD_XN 2, \base
    LOAD_XN 3, \base
    LOAD_XN 4, \base
    LOAD_XN 5, \base
    LOAD_XN 6, \base
    LOAD_XN 7, \base
    LOAD_XN 8, \base
    LOAD_XN 9, \base
    LOAD_XN 10, \base
    LOAD_XN 11, \base
    LOAD_XN 12, \base
    LOAD_XN 13, \base
    LOAD_XN 14, \base
    LOAD_XN 15, \base
    LOAD_XN 16, \base
    LOAD_XN 17, \base
    LOAD_XN 18, \base
    LOAD_XN 19, \base
    LOAD_XN 20, \base
    LOAD_XN 21, \base
    LOAD_XN 22, \base
    LOAD_XN 23, \base
    LOAD_XN 24, \base
    LOAD_XN 25, \base
    LOAD_XN 26, \base
    LOAD_XN 27, \base
    LOAD_XN 28, \base
    LOAD_XN 29, \base
    LOAD_XN 30, \base
    LOAD_XN 31, \base

.endm
