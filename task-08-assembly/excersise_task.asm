##################################################################
#
#   RISC-V assembly code example
#   - I/O
#
#   Author: Tobias Hansson <tohans@kth.se>, Viola Söderlund <violaso@kth.se>
#
#   Last updated: 2020-10-24
#   Translated from MIPS to RISC-V 2024-10-31 by Benjamin Widman <bwidman@kth.se>
#
#   See: RISC-V Syscall Sheet (https://github.com/TheThirdOne/rars/wiki/Environment-Calls)
#   See: RISC-V Instruction Sheet (https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)
#
##################################################################

.data                               
star:     	.asciz "*"     #define label HW as our hello world string
nl:			.asciz "\n"


.text

main: 
    # get input
    li  		a7, 5                   # set system call code to "read integer"
    ecall                             	# read integer from standard input stream to a0
	
	# Pass user input to function
   	add 		a1, zero, a0
    
    jal 		ra, print_straight
    
    # exit program
    li 			a7, 10                  # set system call code to "terminate program"
    ecall                         		# terminate program
    
    
print_straight:
	li  		a7, 4                   # set system call code to "print integer"
    la			a0, star
    ecall 

    la			a0, nl
    ecall 
    
    addi		a1, a1, -1
    bne			a1, zero, print_straight
    
    # Jump back to main after done printing
    jr ra
    nop

print_trig_down:
	li  		a7, 4                   # set system call code to "print integer"
    la			a0, star
    ecall 

    la			a0, nl
    ecall 
    
    addi		a1, a1, -1
    bne			a1, zero, print_straight
    
    # Jump back to main after done printing
    jr ra
    nop

##################################################################
#
#   NOTE:
#       The Executable Code Section is the default section. Therefore ".text" isn't needed.
#
##################################################################
