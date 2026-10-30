##################################################################
#
#   RISC-V assembly code example
#   - Insturction examples
#
#   Author: Dmitry Chirin <dmitryc@kth.se>
#
#   Created: 2024-11-01
#
#   See: RISC-V Syscall Sheet (https://github.com/TheThirdOne/rars/wiki/Environment-Calls)
#   See: RISC-V Instruction Sheet (https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)
#
##################################################################


# Immediate basic instructions
addi 	t0, zero, 1		# Assign t0 = 01 = 1
ori 	t1, t0, 2		# Assign t1 = t0 | 10 = 01 | 10 = 11 = 3
xori	t2, t0, 3		# Assign t2 = t0 ^ 3 = 01 ^ 11 = 10 = 2
andi 	t3, t2, 7		# Assign t3 = t3 & 111 = 011 ^ 111 = 011 = 3

# Adding negative numbers for subtraction
addi 	t3, t1, -1		# Assign t3 = t1 + (-1) = 3 + (-1) = 2  

# Regester basic instructions
add 	t4, t0, t2		# t4 = t0 + t2
or	 	t4, t0, t1		# t4 = t0 | t1
xor		t4, t1, t2		# t4 = t1 ^ t2
and		t4, t2, t1		# t4 = t2 & t1

# Pseudoinstuctions
sub 	t4, t2, t1		# t4 = t2 - t1
div		t4, t1, t0		# t4 = t1 / t0
mul		t4, t1, t1		# t4 = t1 * t1

li		t3, 10000000	# Loads directly to 32 bits (a word, 4 bytes) for large numbers

# Loads upper part of the regester (upper 16 bits)
lui		t3, 321

# Stack manipulation
sw		t3, 0(sp)		# Loads a word into the position of stack pointer
lw		s0, 0(sp)		# Reads a word from the position of stack pointer

addi	sp, sp, -4		# Advances the stack pointer to the next word address 
						# OBS! The stack decreases in position, by 4 bytes each time
lw		s1, 4(sp)		# Loads from stack with an offset of 4 bytes

# Conditionals and jumps
beq		t1, t2, exit	# If t1 == t2, then branch/jump/goto exit, otherwise continue
# bne		t1, t2, exit	# If t1 != t2 branch to exit

slt		t3, t1, t2		# Set t3 to 1 if t1 < t2, else if t1 > t2 then set t3 to 0
slti	t3, zero, 10	# Same, but with immediate


jal		test
j 		exit
# jarl	exit


test:
	j 	test2


test2:
	jr 	ra


exit:
	# exit program
    li	a7, 10      	# Set system call to 10 (exit)
    ecall 				# Call system call

