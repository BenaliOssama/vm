# Game dynamics

* [ ] Use 2 or more players.

---

# The virtual machine

* [ ] If one of the `.cor` files is corrupted, the VM should exit with an error code, //print a message on stderr, and do not execute the programs.
* [ ] Those are the cases where a file is considered corrupted:
  * Wrong signature code.
  * Declared size of the program not matching the actual size.
  * The size of the program is bigger than the maximum allowed size.
  * The total file size is smaller than the minimum size.
* [ ] At the start of the battle the VM will //print a welcome message as shown in the example:


* [ ] The last program passed will be the first one executed during the cycle.

* [ ] When a new process is forked, it will be placed at the end of the processes and start execution at the start of the next cycle (it will be first executed on the next cycle).

---

# Stop process execution

* [ ] Smart players may trick another player into making `live` statements, so a player may still execute `live` after all processes are killed.

---

# Parameters types

----> * [ ] Some instructions truncate addresses using `IDX_MOD` to prevent processes from attacking faraway memory directly (balance purposes).

---

# Your player

* [ ] Provide a basic player able to fight and win against `ameba.s`.

---

# Bonus

* [ ] Create a disassembler: binary → `.s`.
* [ ] Create a visualizer: real-time VM state.
* [ ] Add arithmetic operations in Assembly language.
* [ ] Add simple macro system in Assembly language.

---

# Additional notes

* [ ] use the constants instead of hardcoded values
* [ ] Make use of the instruction file. Work on multiple processes simultaneously (support 2+ files).
* [ ] Read about how modern CPUs do the fetch-execute cycle: [https://corewar-docs.readthedocs.io/en/latest/redcode/parser/](https://corewar-docs.readthedocs.io/en/latest/redcode/parser/)
