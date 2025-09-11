# general instruction
- [X] During a cycle the VM will load the instruction at the current PC and wait N cycles before to execute it (N being the cost of the instruction).
_____________________________________________________________________________
# game dynamics
- [ ] use 2 or more players, change the counter to support that.
_____________________________________________________________________________
# structure of the game
_____________________________________________________________________________
# end game
- [ ] notify the vm that the player is live
- [ ] live check during vm checks and remove dead process.
- [ ] decrease the elapsed time sens last check algo
- [ ] The VM will use some rules to decrease the elapsed time since the last check down to zero, this means the games can never be infinite and all processes will be killed at one point.
_____________________________________________________________________________
# the assembler
_____________________________________________________________________________
# the virtual machine
- [ ] If no parameters are passed it will print a help message.
- [ ] If one of the .cor file is corrupted the VM should exit with an error code, print a message on stderr and do not execute the programs.
- [ ] At the start of the battle the VM will print a welcome message as the one showed in the example.
For this match the players will be:
Player 1 ([X] bytes): [NAME] ([DESCRIPTION])
Player 2 ([X] bytes): [NAME] ([DESCRIPTION])
...
- [ ] At the end of the battle the VM will write the winner (if any) as the one showed in the example.
At the end of the game the vm should print cycle [X]: The winner is player [X]: [NAME]!.

If nobody executed a valid live statement the end message should be cycle [X]: Nobody wins!.
- [ ] The players will be loaded into the arena starting by the first byte and they will be evenly spaced from each other.
- [ ] The VM must handle a -d [NB_CYCLES] flag (stating for "dump"). If the flag is specified the VM will stop execution at NB_CYCLES and dump the memory in the arena in hexadecimal, writing 32 bytes per row.
- [ ] The last program passed will be the first one executed during the cycle. 
- [ ] Only when executing the instruction the VM will check for the parameters it may have and it will execute it only if the parameters are correct, otherwise it will print an error on stderr and continue to execute.
- [ ] If an instruction has incorrect parameters the PC will be moved forward according to the size of the parameters.
- [ ]  If the instruction doesn't exist in the instruction set the PC will be moved forward of 1 byte.
- [ ] When a new process is forked it will be placed at the end of the processes and start its execution at the start of the next cycle (which means it will be the first one being executed on the next cycle).
- [ ] the vm assums the binary is in big-endian and read it accordingly
- [ ] Those are the cases where a file is considered corrupted:
Wrong signature code.
Declared size of the program not matching the actual size.
The size of the program is bigger that the maximum allowed size.
The total file size is smaller than the minimum size.
_____________________________________________________________________________
# greeting and end gmae message
_____________________________________________________________________________
# circular memory space of the arena
- [X] The memory where the players will fight is circular, this means if we want to move forward from the last address in memory (for example 4095) we will arrive at address 0.
_____________________________________________________________________________
# stop process executioin
- [ ] Every CYCLE_TO_DIE the VM will check every process and kill all the processes that did not successfully execute any live instruction.

To avoid infinite games, CYCLES_TO_DIE will be decremented by CYCLE_DELTA under certain conditions:

If during the last life loop there were at least NBR_LIVE successfully executed by the players.
If it has been MAX_CHECKS life loops since it was decremented last time.
Notice that some smart players may trick another player to start making live statements for them, this virtually means a player can still make live statements after all its processes were killed.
_____________________________________________________________________________
# the assembly language
_____________________________________________________________________________
# parameters types
- [ ] The VM will initialize the registers to 0 for each player except for r1 which will have - PLAYER_ID, so for the first player r1 == -1.
- [ ] All addresses are relative to the current PC of the process.
- [ ] Some instructions have to truncate the addresses they are given by applying modulo IDX_MOD. This feature prevents players from reaching spaces in memory that are too far from the current PC, which means processes won't be able to attack each other straight away but will need to move by doing smaller steps, this help having more balanced games.
_____________________________________________________________________________
# labels
_____________________________________________________________________________
# pcode field
- [ ] Some instruction may accept different kind of parameters, and those parameters may require different sizes. For those instructions the assembler will use a byte after the opcode to inform the VM which kind of parameters to expect.
Every two bits of this byte will inform the VM about the type of a parameter.

01 will be used for a register parameter.
10 will be used for a direct parameter.
11 will be used for an indirect parameter.
An example of a pcode byte could be 01111000:

Here the first parameter is expected to be a register, the second should be an indirect value and the third a direct value.

Those numbers are in binary base, for example 10 in binary is 2 in decimal.
_____________________________________________________________________________
# has idx field
- [ ] Some instruction will sum up direct values and/or use them as addresses to lookup into the arena memory space. For this reason we know that a full 32 bits integer won't be necessary and to save space we do save direct values on only two bytes in those cases

_____________________________________________________________________________
# the carry flag 
- [ ] Those commands will set the carry to true if the value written in the register is 0, and set it to false otherwise.
- [ ] Only one command reads the carry and it's zjmp, it will do the jump if the carry is true and will do nothing otherwise.
_____________________________________________________________________________
# the file signature






_____________________________________________________________________________
- [ ] The entire execution is deterministic, so with the same inputs you must always have the same outputs.
- [ ] The binary must be written in big-endian.
- [ ] The Virtual Machine role is to execute the binary programs (players) provided as .cor files (for a maximum of 4 players).
- [ ] At the end of the game the vm should print cycle [X]: The winner is player [X]: [NAME]!.

If nobody executed a valid live statement the end message should be cycle [X]: Nobody wins!. 
- [ ] It implies that moving backward of one position from 0 will bring us to the address 4095.
- [ ] To avoid infinite games, CYCLES_TO_DIE will be decremented by CYCLE_DELTA under certain conditions:

If during the last life loop there were at least NBR_LIVE successfully executed by the players.
If it has been MAX_CHECKS life loops since it was decremented last time.
Notice that some smart players may trick another player to start making live statements for them, this virtually means a player can still make live statements after all its processes were killed. 
- [ ] The VM will initialize the registers to 0 for each player except for r1 which will have - PLAYER_ID, so for the first player r1 == -1. 
- [ ] Some instruction may accept different kind of parameters, and those parameters may require different sizes. For those instructions the assembler will use a byte after the opcode to inform the VM which kind of parameters to expect. Some instruction may accept different kind of parameters, and those parameters may require different sizes. For those instructions the assembler will use a byte after the opcode to inform the VM which kind of parameters to expect.

Every two bits of this byte will inform the VM about the type of a parameter.

01 will be used for a register parameter.
10 will be used for a direct parameter.
11 will be used for an indirect parameter.
An example of a pcode byte could be 01111000:

Here the first parameter is expected to be a register, the second should be an indirect value and the third a direct value.

Those numbers are in binary base, for example 10 in binary is 2 in decimal.
/*___________________________________________________________________________*/
* The Carry flag

- [ ] 
- [ ]
- [ ]
- [ ]



1- make use of the instruction file

5- work on multip process at the simetime so the user can give two or more files
6- read about how modern cpus do the fetch execute cycle...
https://corewar-docs.readthedocs.io/en/latest/redcode/parser/
