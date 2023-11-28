.altmacro
.macro SAVE_XN n
    sd x\n, (\n -1 )*8(sp)
.endm

.macro LOAD_XN n
    ld x\n , (\n -1 )*8(sp)
.endm

.macro SAVE_G_REG start end
.set n, start
.rept end - start
        SAVE_XN %n
        .set n, n + 1
.endr
.endm

.macro REST_G_REG start end
.set n, start
.rept end - start
        LOAD_XN %n
        .set n, n + 1
.endr
.endm
