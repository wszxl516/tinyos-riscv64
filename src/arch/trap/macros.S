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

.macro SAVE_G_REG base
    .irp reg, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
        SAVE_XN \reg, \base
    .endr
    .irp reg, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        SAVE_XN \reg, \base
    .endr
.endm

.macro REST_G_REG base
    .irp reg, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
        LOAD_XN \reg, \base
    .endr
    .irp reg, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
        LOAD_XN \reg, \base
    .endr
.endm
