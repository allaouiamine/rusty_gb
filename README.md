
# Test roms

* 01-special.gb PASSED
* 02-interrupts.gb PASSED
* 03-op sp,hl.gb PASSED
* 04-op r,imm.gb unimplemented instruction: NONE, opcode: DE -- SBC
* 05-op rp.gb PASSED
* 06-ld r,r.gb PASSED
* 07-jr,jp,call,ret,rst.gb unimplemented instruction: NONE, opcode: D9 -- RETI
* 08-misc instrs.gb PASS
* 09-op r,r.gb unimplemented instruction: NONE, opcode: 2F -- CPL
* 10-bit ops.gb PASSED
* 11-op a,(hl).gb unimplemented instruction: NONE, opcode: 9E -- SBC
* cpu_instrs.gb 01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  12:ok  13:ok  14:ok  15:ok  16:ok 17:ok  18:ok  19:ok  20:ok  21:ok  22:ok  23:ok  24:ok  25:ok  26:ok  27:ok  28:ok  29:ok  30:ok  31:ok  32: ok  33 unimplemented instruction: NONE, opcode: 07 -- RLCA
* dmg-acid2.gb -- infinite!!! investigate later!
* mem_timing.gb unimplemented instruction: NONE, opcode: 9E -- SBC