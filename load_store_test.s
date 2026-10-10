addi x5, x0, 0xff
sb x5, -1(x2)
lb x6, -1(x2)
lbu x7, -1(x2)

addi x28, x0, -2
sw x28, -16(x2)
lw x29, -16(x2)
lwu x30, -16(x2)

sd x28, -32(x2)
ld x31, -32(x2)
