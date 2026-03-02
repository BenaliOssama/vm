# i consider the project done at this moment. if you found a bug, have an idea to imporove it or have a question. don't hesitate to ask. thank you.

# Game dynamics

* [X] Use 2 or more players.

---

# The virtual machine



* [X]When a new process is forked, it will be placed at the end of the processes and start execution at the start of the next cycle (it will be first executed on the next cycle).

---

# Stop process execution

* [X]Smart players may trick another player into making `live` statements, so a player may still execute `live` after all processes are killed.

---

# Parameters types

----> * [X]Some instructions truncate addresses using `IDX_MOD` to prevent processes from attacking faraway memory directly (balance purposes).

---

# Your player

* [X]Provide a basic player able to fight and win against `ameba.s`.

---

# Bonus

* [X]Create a disassembler: binary → `.s`.
* [X]Create a visualizer: real-time VM state.
* [X]Add arithmetic operations in Assembly language.
* [X]Add simple macro system in Assembly language.

---

# Additional notes

* [X]use the constants instead of hardcoded values
* [X]Make use of the instruction file. Work on multiple processes simultaneously (support 2+ files).
* [X]Read about how modern CPUs do the fetch-execute cycle: [https://corewar-docs.readthedocs.io/en/latest/redcode/parser/](https://corewar-docs.readthedocs.io/en/latest/redcode/parser/)
