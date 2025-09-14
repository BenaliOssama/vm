# Note

The provided VM has an extra flag `-v` which you can use to print the state of the VM at every cycle, this should greatly help you during development and debugging.

---

# General instruction

* [x] During a cycle the VM will load the instruction at the current PC and wait N cycles before executing it (N being the cost of the instruction).

---

# Game dynamics

* [ ] Use 2 or more players, change the counter to support that.

---

# Structure of the game

---

# End game

* [ ] Notify the VM that the player is live.
* [ ] worry about the integrety of the instruction's arguments
* [X] Live check during VM checks and remove dead processes.
* [X] Decrease the elapsed time since last check according to the algorithm.
* [ ] The VM will use some rules to decrease the elapsed time since the last check down to zero, this means the games can never be infinite and all processes will be killed at one point.

---

# The assembler

* [ ] If no parameters are passed it will print a help message.
* [ ] Takes a `file.s` as input.
* [ ] Returns a `file.cor` as output.
* [ ] If there is any error in the source file, the Assembler should exit with an error code, print a message on `stderr` (the more specific the better), and do not create any `file.cor`.
* [ ] `.name` and `.description` must appear before any instruction.
* [ ] The binary must be written in big-endian.

---

# The virtual machine

* [ ] If no parameters are passed it will print a help message.
* [ ] If one of the `.cor` files is corrupted, the VM should exit with an error code, print a message on stderr, and do not execute the programs.
* [ ] At the start of the battle the VM will print a welcome message as shown in the example:

```
For this match the players will be:
Player 1 ([X] bytes): [NAME] ([DESCRIPTION])
Player 2 ([X] bytes): [NAME] ([DESCRIPTION])
...
```

* [ ] At the end of the battle, the VM will write the winner (if any) as shown in the example:

```
cycle [X]: The winner is player [X]: [NAME]!
```

> If nobody executed a valid `live` statement the end message should be `cycle [X]: Nobody wins!`.

* [ ] The players will be loaded into the arena starting from the first byte and will be evenly spaced.
* [ ] The VM must handle a `-d [NB_CYCLES]` flag (dump). If specified, the VM stops execution at `NB_CYCLES` and dumps the arena memory in hexadecimal (32 bytes per row).
* [ ] The last program passed will be the first one executed during the cycle.
* [ ] Only when executing the instruction, the VM will check parameters. If incorrect, it will print an error on stderr and continue.
* [ ] If an instruction has incorrect parameters, the PC will be moved forward according to the size of the parameters.
* [ ] If the instruction doesn't exist in the instruction set, the PC will move forward by 1 byte.
* [ ] When a new process is forked, it will be placed at the end of the processes and start execution at the start of the next cycle (it will be first executed on the next cycle).
* [ ] The VM assumes the binary is in big-endian.
* [ ] Those are the cases where a file is considered corrupted:

  * Wrong signature code.
  * Declared size of the program not matching the actual size.
  * The size of the program is bigger than the maximum allowed size.
  * The total file size is smaller than the minimum size.
* [ ] The entire execution is deterministic: same inputs → same outputs.

---

# Circular memory space of the arena

* [x] The memory where the players will fight is circular. Moving forward from the last address (e.g., 4095) wraps to address 0.
* [ ] Moving backward from 0 wraps to address 4095.

---

# Stop process execution

* [ ] Every `CYCLE_TO_DIE` the VM will check every process and kill all that did not successfully execute any `live` instruction.
* [ ] To avoid infinite games, `CYCLES_TO_DIE` will be decremented by `CYCLE_DELTA` under certain conditions:

  * During the last life loop, if at least `NBR_LIVE` successfully executed.
  * If it has been `MAX_CHECKS` life loops since the last decrement.
* [ ] Smart players may trick another player into making `live` statements, so a player may still execute `live` after all processes are killed.

---

# Parameters types

* [ ] The VM will initialize registers to 0 for each player, except `r1` = `-PLAYER_ID` (first player: r1 = -1).
* [ ] All addresses are relative to the current PC of the process.
* [ ] Some instructions truncate addresses using `IDX_MOD` to prevent processes from attacking faraway memory directly (balance purposes).

---

# Labels

---

# Pcode field

* [ ] Some instructions accept different kinds of parameters, which may require different sizes. The assembler uses a byte after the opcode to inform the VM.
* [ ] Every two bits indicate parameter type:

  * `01` → register
  * `10` → direct
  * `11` → indirect
* [ ] Example: `01111000` → first: register, second: indirect, third: direct.
* [ ] Numbers are in binary (e.g., `10` binary = 2 decimal).

---

# Has IDX field

* [ ] Some instructions use 2-byte direct values instead of full 32-bit for addressing (when `Has Idx`).

---

# The Carry flag

* [ ] Commands modifying carry: `ld`, `add`, `sub`, `and`, `or`, `xor`.
* [ ] Carry = `true` if value written = 0; otherwise `false`.
* [ ] Only `zjmp` reads carry: jumps if `true`, does nothing otherwise.

---

# The file signature

---

# Your player

* [ ] Provide a basic player able to fight and win against `ameba.s`.
* [ ] A config file provides constants for both Assembler and VM (language-agnostic, easy to translate).

---

# A basic player

---

# Testing environment

---

# Some advices for the road

---

# Bonus

* [ ] Create a disassembler: binary → `.s`.
* [ ] Create a visualizer: real-time VM state.
* [ ] Add arithmetic operations in Assembly language.
* [ ] Add simple macro system in Assembly language.

---

# Additional notes

* [ ] use the constants instead of hardcoded values
* [ ] Make use of the instruction file.
* [ ] Work on multiple processes simultaneously (support 2+ files).
* [ ] Read about how modern CPUs do the fetch-execute cycle: [https://corewar-docs.readthedocs.io/en/latest/redcode/parser/](https://corewar-docs.readthedocs.io/en/latest/redcode/parser/)
