##
# Push value to application stack.
# PARAM: Registry with value.
##
.macro	PUSH (%reg)
	addi	sp,sp,-4              # decrement stack pointer (stack builds "downwards" in memory)
	sw	%reg,0(sp)             # save value to stack
.end_macro

##
# Pop value from application stack.
# PARAM: Registry which to save value to.
##
.macro	POP (%reg)
	lw	%reg,0(sp)             # load value from stack to given registry
	addi	sp,sp,4               # increment stack pointer (stack builds "downwards" in memory)
.end_macro

.data
.text

main:
	li 	t0, 0
	li 	t1, 1
	
	# get input
    li  a7, 5                         # set system call code to "read integer"
    ecall                             # read integer from standard input stream to a0
    
   	beq	a0, t0, print
   	beq	a0, t1, print
    
    add a0 a2
    
    jal fib
    
    # print output
    li  a7, 1                          # set system call code to "print integer"
    ecall  
 
	
	# exit program
    j       exit_program
    nop

fib:
	beq 
	
	# manage saved regitries
    add		s1, ra, zero            # save return address
    PUSH(s1)                       # save parent routine's saved value to stack

    # call print_hello_world subroutine
    jal		ra, print_hello_world	   # jump to print_hello_world and save position to ra

    # manage saved regitries
    add		ra, s1, zero            # restore return adress
    POP(s1)                       # restore parent routine's saved value from stack
	
	# return
    jr      ra                     # return to where hello_world was called from


print:
	# print output
    li  a7, 1                          # set system call code to "print integer"
    ecall  
 
	# exit program
    j       exit_program
    nop

exit_program:
    # exit program
    li	a7, 10                      # set system call code to "terminate program"
    ecall 
