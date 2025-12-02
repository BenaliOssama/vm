RUSTFLAGS="-Awarnings" cargo run playground/players_src/pierino_add.cor #_ind_ind.cor
| id |  name   |  comment   | start address | size |                                                 code                                                  |
|----+---------+------------+---------------+------+-------------------------------------------------------------------------------------------------------|
| -1 | pierino | stay alive |       0       |  34  | 01 ff ff ff ff 02 90 00 00 00 02 02 02 90 00 00 00 03 03 04 54 02 03 04 02 90 00 00 00 00 03 09 ff e1 |


| PC | Carry | Current Instruction | Remaining Cycles |                Lives Status                |
|----+-------+---------------------+------------------+--------------------------------------------|
| 0  | false |        None         |        0         | executed: false, player_id: 0, nbr_live: 0 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:0, R3:0, R4:0, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


| Addr | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 0A | 0B | 0C | 0D | 0E | 0F |
|------+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----|
| 0000 | 01 | FF | FF | FF | FF | 02 | 90 | 00 | 00 | 00 | 02 | 02 | 02 | 90 | 00 | 00 |
| 0010 | 00 | 03 | 03 | 04 | 54 | 02 | 03 | 04 | 02 | 90 | 00 | 00 | 00 | 00 | 03 | 09 |
| 0020 | FF | E1 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 |
| 0030 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 | 00 |


address 1 instruction : 1
we are going to fetch 4
the value we fetched bytes is [255, 255, 255, 255]
the value we fetched is -1
[32m------------------------------------------------------------------------------------[0m 
Cycle 1 || Cycles before life check: 1535 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   9 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 2 || Cycles before life check: 1534 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   8 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 3 || Cycles before life check: 1533 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   7 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 4 || Cycles before life check: 1532 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   6 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 5 || Cycles before life check: 1531 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   5 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 6 || Cycles before life check: 1530 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   4 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 7 || Cycles before life check: 1529 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   3 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 8 || Cycles before life check: 1528 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   2 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 9 || Cycles before life check: 1527 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   1 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 10 || Cycles before life check: 1526 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |false |live   |   0 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 1, parameters: [Direct(-1)], opcode_addr: 0 })
[34mLIVE[0m
heeeey!!! i'm alive :)
[32m------------------------------------------------------------------------------------[0m 
address 6 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 11 || Cycles before life check: 1525 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |false |ld     |   4 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 12 || Cycles before life check: 1524 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |false |ld     |   3 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 13 || Cycles before life check: 1523 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |false |ld     |   2 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 14 || Cycles before life check: 1522 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |false |ld     |   1 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 15 || Cycles before life check: 1521 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |false |ld     |   0 | 1:ffffffff  2:0  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(2), Register(2), None], opcode_addr: 5 })
[34mLD[0m
ld: r2 ← 2
| PC | Carry |                                  Current Instruction                                  | Remaining Cycles |                Lives Status                |
|----+-------+---------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 12 | false | Instruction { opcode: 2, parameters: [Direct(2), Register(2), None], opcode_addr: 5 } |        0         | executed: true, player_id: -1, nbr_live: 1 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:0, R4:0, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 13 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 16 || Cycles before life check: 1520 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   4 | 1:ffffffff  2:2  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 17 || Cycles before life check: 1519 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   3 | 1:ffffffff  2:2  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 18 || Cycles before life check: 1518 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   2 | 1:ffffffff  2:2  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 19 || Cycles before life check: 1517 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   1 | 1:ffffffff  2:2  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 20 || Cycles before life check: 1516 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   0 | 1:ffffffff  2:2  3:0  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(3), Register(3), None], opcode_addr: 12 })
[34mLD[0m
ld: r3 ← 3
| PC | Carry |                                  Current Instruction                                   | Remaining Cycles |                Lives Status                |
|----+-------+----------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 19 | false | Instruction { opcode: 2, parameters: [Direct(3), Register(3), None], opcode_addr: 12 } |        0         | executed: true, player_id: -1, nbr_live: 1 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:3, R4:0, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 20 instruction : 4
[32m------------------------------------------------------------------------------------[0m 
Cycle 21 || Cycles before life check: 1515 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   9 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 22 || Cycles before life check: 1514 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   8 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 23 || Cycles before life check: 1513 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   7 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 24 || Cycles before life check: 1512 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   6 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 25 || Cycles before life check: 1511 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   5 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 26 || Cycles before life check: 1510 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   4 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 27 || Cycles before life check: 1509 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   3 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 28 || Cycles before life check: 1508 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   2 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 29 || Cycles before life check: 1507 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   1 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 30 || Cycles before life check: 1506 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   0 | 1:ffffffff  2:2  3:3  4:0  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 4, parameters: [Register(2), Register(3), Register(4)], opcode_addr: 19 })
[34mADD[0m
add : r4 ← r2 + r3
| PC | Carry |                                       Current Instruction                                       | Remaining Cycles |                Lives Status                |
|----+-------+-------------------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 24 | false | Instruction { opcode: 4, parameters: [Register(2), Register(3), Register(4)], opcode_addr: 19 } |        0         | executed: true, player_id: -1, nbr_live: 1 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:3, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 25 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 31 || Cycles before life check: 1505 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   4 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 32 || Cycles before life check: 1504 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   3 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 33 || Cycles before life check: 1503 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   2 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 34 || Cycles before life check: 1502 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   1 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 35 || Cycles before life check: 1501 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   0 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(0), Register(3), None], opcode_addr: 24 })
[34mLD[0m
ld: r3 ← 0
| PC | Carry |                                  Current Instruction                                   | Remaining Cycles |                Lives Status                |
|----+-------+----------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 31 | true  | Instruction { opcode: 2, parameters: [Direct(0), Register(3), None], opcode_addr: 24 } |        0         | executed: true, player_id: -1, nbr_live: 1 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:0, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 32 instruction : 9
we are going to fetch just 2
the value we fetched bytes is [255, 225]
the value we fetched is -31
[32m------------------------------------------------------------------------------------[0m 
Cycle 36 || Cycles before life check: 1500 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  19 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 37 || Cycles before life check: 1499 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  18 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 38 || Cycles before life check: 1498 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  17 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 39 || Cycles before life check: 1497 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  16 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 40 || Cycles before life check: 1496 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  15 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 41 || Cycles before life check: 1495 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  14 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 42 || Cycles before life check: 1494 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  13 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 43 || Cycles before life check: 1493 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  12 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 44 || Cycles before life check: 1492 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  11 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 45 || Cycles before life check: 1491 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  10 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 46 || Cycles before life check: 1490 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   9 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 47 || Cycles before life check: 1489 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   8 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 48 || Cycles before life check: 1488 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   7 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 49 || Cycles before life check: 1487 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   6 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 50 || Cycles before life check: 1486 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   5 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 51 || Cycles before life check: 1485 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 52 || Cycles before life check: 1484 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 53 || Cycles before life check: 1483 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 54 || Cycles before life check: 1482 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 55 || Cycles before life check: 1481 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 9, parameters: [Direct(-31)], opcode_addr: 31 })
[34mZJMP[0m
[33mbefor jump :[0m 34
[33mafter jump :[0m 0
heeeey!!! i jumped or didn't :)
[32m------------------------------------------------------------------------------------[0m 
address 1 instruction : 1
we are going to fetch 4
the value we fetched bytes is [255, 255, 255, 255]
the value we fetched is -1
[32m------------------------------------------------------------------------------------[0m 
Cycle 56 || Cycles before life check: 1480 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   9 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 57 || Cycles before life check: 1479 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   8 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 58 || Cycles before life check: 1478 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   7 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 59 || Cycles before life check: 1477 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   6 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 60 || Cycles before life check: 1476 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   5 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 61 || Cycles before life check: 1475 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 62 || Cycles before life check: 1474 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 63 || Cycles before life check: 1473 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 64 || Cycles before life check: 1472 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 65 || Cycles before life check: 1471 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 1, parameters: [Direct(-1)], opcode_addr: 0 })
[34mLIVE[0m
heeeey!!! i'm alive :)
[32m------------------------------------------------------------------------------------[0m 
address 6 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 66 || Cycles before life check: 1470 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 67 || Cycles before life check: 1469 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 68 || Cycles before life check: 1468 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 69 || Cycles before life check: 1467 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 70 || Cycles before life check: 1466 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(2), Register(2), None], opcode_addr: 5 })
[34mLD[0m
ld: r2 ← 2
| PC | Carry |                                  Current Instruction                                  | Remaining Cycles |                Lives Status                |
|----+-------+---------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 12 | false | Instruction { opcode: 2, parameters: [Direct(2), Register(2), None], opcode_addr: 5 } |        0         | executed: true, player_id: -1, nbr_live: 2 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:0, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 13 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 71 || Cycles before life check: 1465 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 72 || Cycles before life check: 1464 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 73 || Cycles before life check: 1463 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 74 || Cycles before life check: 1462 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 75 || Cycles before life check: 1461 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(3), Register(3), None], opcode_addr: 12 })
[34mLD[0m
ld: r3 ← 3
| PC | Carry |                                  Current Instruction                                   | Remaining Cycles |                Lives Status                |
|----+-------+----------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 19 | false | Instruction { opcode: 2, parameters: [Direct(3), Register(3), None], opcode_addr: 12 } |        0         | executed: true, player_id: -1, nbr_live: 2 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:3, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 20 instruction : 4
[32m------------------------------------------------------------------------------------[0m 
Cycle 76 || Cycles before life check: 1460 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   9 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 77 || Cycles before life check: 1459 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   8 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 78 || Cycles before life check: 1458 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   7 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 79 || Cycles before life check: 1457 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   6 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 80 || Cycles before life check: 1456 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   5 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 81 || Cycles before life check: 1455 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   4 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 82 || Cycles before life check: 1454 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   3 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 83 || Cycles before life check: 1453 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   2 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 84 || Cycles before life check: 1452 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   1 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 85 || Cycles before life check: 1451 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   0 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 4, parameters: [Register(2), Register(3), Register(4)], opcode_addr: 19 })
[34mADD[0m
add : r4 ← r2 + r3
| PC | Carry |                                       Current Instruction                                       | Remaining Cycles |                Lives Status                |
|----+-------+-------------------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 24 | false | Instruction { opcode: 4, parameters: [Register(2), Register(3), Register(4)], opcode_addr: 19 } |        0         | executed: true, player_id: -1, nbr_live: 2 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:3, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 25 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 86 || Cycles before life check: 1450 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   4 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 87 || Cycles before life check: 1449 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   3 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 88 || Cycles before life check: 1448 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   2 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 89 || Cycles before life check: 1447 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   1 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 90 || Cycles before life check: 1446 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   0 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(0), Register(3), None], opcode_addr: 24 })
[34mLD[0m
ld: r3 ← 0
| PC | Carry |                                  Current Instruction                                   | Remaining Cycles |                Lives Status                |
|----+-------+----------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 31 | true  | Instruction { opcode: 2, parameters: [Direct(0), Register(3), None], opcode_addr: 24 } |        0         | executed: true, player_id: -1, nbr_live: 2 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:0, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 32 instruction : 9
we are going to fetch just 2
the value we fetched bytes is [255, 225]
the value we fetched is -31
[32m------------------------------------------------------------------------------------[0m 
Cycle 91 || Cycles before life check: 1445 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  19 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 92 || Cycles before life check: 1444 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  18 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 93 || Cycles before life check: 1443 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  17 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 94 || Cycles before life check: 1442 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  16 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 95 || Cycles before life check: 1441 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  15 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 96 || Cycles before life check: 1440 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  14 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 97 || Cycles before life check: 1439 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  13 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 98 || Cycles before life check: 1438 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  12 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 99 || Cycles before life check: 1437 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  11 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 100 || Cycles before life check: 1436 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |  10 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 101 || Cycles before life check: 1435 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   9 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 102 || Cycles before life check: 1434 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   8 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 103 || Cycles before life check: 1433 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   7 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 104 || Cycles before life check: 1432 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   6 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 105 || Cycles before life check: 1431 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   5 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 106 || Cycles before life check: 1430 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 107 || Cycles before life check: 1429 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 108 || Cycles before life check: 1428 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 109 || Cycles before life check: 1427 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 110 || Cycles before life check: 1426 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  31 |true  |zjmp   |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 9, parameters: [Direct(-31)], opcode_addr: 31 })
[34mZJMP[0m
[33mbefor jump :[0m 34
[33mafter jump :[0m 0
heeeey!!! i jumped or didn't :)
[32m------------------------------------------------------------------------------------[0m 
address 1 instruction : 1
we are going to fetch 4
the value we fetched bytes is [255, 255, 255, 255]
the value we fetched is -1
[32m------------------------------------------------------------------------------------[0m 
Cycle 111 || Cycles before life check: 1425 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   9 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 112 || Cycles before life check: 1424 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   8 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 113 || Cycles before life check: 1423 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   7 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 114 || Cycles before life check: 1422 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   6 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 115 || Cycles before life check: 1421 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   5 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 116 || Cycles before life check: 1420 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 117 || Cycles before life check: 1419 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 118 || Cycles before life check: 1418 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 119 || Cycles before life check: 1417 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 120 || Cycles before life check: 1416 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   0 |true  |live   |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 1, parameters: [Direct(-1)], opcode_addr: 0 })
[34mLIVE[0m
heeeey!!! i'm alive :)
[32m------------------------------------------------------------------------------------[0m 
address 6 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 121 || Cycles before life check: 1415 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 122 || Cycles before life check: 1414 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 123 || Cycles before life check: 1413 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 124 || Cycles before life check: 1412 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 125 || Cycles before life check: 1411 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |   5 |true  |ld     |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(2), Register(2), None], opcode_addr: 5 })
[34mLD[0m
ld: r2 ← 2
| PC | Carry |                                  Current Instruction                                  | Remaining Cycles |                Lives Status                |
|----+-------+---------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 12 | false | Instruction { opcode: 2, parameters: [Direct(2), Register(2), None], opcode_addr: 5 } |        0         | executed: true, player_id: -1, nbr_live: 3 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:0, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 13 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 126 || Cycles before life check: 1410 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   4 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 127 || Cycles before life check: 1409 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   3 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 128 || Cycles before life check: 1408 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   2 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 129 || Cycles before life check: 1407 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   1 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 130 || Cycles before life check: 1406 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  12 |false |ld     |   0 | 1:ffffffff  2:2  3:0  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(3), Register(3), None], opcode_addr: 12 })
[34mLD[0m
ld: r3 ← 3
| PC | Carry |                                  Current Instruction                                   | Remaining Cycles |                Lives Status                |
|----+-------+----------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 19 | false | Instruction { opcode: 2, parameters: [Direct(3), Register(3), None], opcode_addr: 12 } |        0         | executed: true, player_id: -1, nbr_live: 3 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:3, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 20 instruction : 4
[32m------------------------------------------------------------------------------------[0m 
Cycle 131 || Cycles before life check: 1405 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   9 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 132 || Cycles before life check: 1404 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   8 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 133 || Cycles before life check: 1403 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   7 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 134 || Cycles before life check: 1402 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   6 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 135 || Cycles before life check: 1401 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   5 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 136 || Cycles before life check: 1400 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   4 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 137 || Cycles before life check: 1399 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   3 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 138 || Cycles before life check: 1398 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   2 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 139 || Cycles before life check: 1397 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   1 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 140 || Cycles before life check: 1396 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  19 |false |add    |   0 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 4, parameters: [Register(2), Register(3), Register(4)], opcode_addr: 19 })
[34mADD[0m
add : r4 ← r2 + r3
| PC | Carry |                                       Current Instruction                                       | Remaining Cycles |                Lives Status                |
|----+-------+-------------------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 24 | false | Instruction { opcode: 4, parameters: [Register(2), Register(3), Register(4)], opcode_addr: 19 } |        0         | executed: true, player_id: -1, nbr_live: 3 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:3, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


[32m------------------------------------------------------------------------------------[0m 
address 25 instruction : 2
[32m------------------------------------------------------------------------------------[0m 
Cycle 141 || Cycles before life check: 1395 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   4 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 142 || Cycles before life check: 1394 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   3 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 143 || Cycles before life check: 1393 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   2 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 144 || Cycles before life check: 1392 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   1 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
waiting...
[32m------------------------------------------------------------------------------------[0m 
[32m------------------------------------------------------------------------------------[0m 
Cycle 145 || Cycles before life check: 1391 || Cycles between checks: 1536
Processes:
Id |Player Id |Pc   |Carry |Instr  |Wait |Registers
 0 |        1 |  24 |false |ld     |   0 | 1:ffffffff  2:2  3:3  4:5  5:0  6:0  7:0  8:0  9:0  10:0  11:0  12:0  13:0  14:0  15:0  16:0  
[31mrunning process[0m 0
executing...
instruction Some(Instruction { opcode: 2, parameters: [Direct(0), Register(3), None], opcode_addr: 24 })
[34mLD[0m
ld: r3 ← 0
| PC | Carry |                                  Current Instruction                                   | Remaining Cycles |                Lives Status                |
|----+-------+----------------------------------------------------------------------------------------+------------------+--------------------------------------------|
| 31 | true  | Instruction { opcode: 2, parameters: [Direct(0), Register(3), None], opcode_addr: 24 } |        0         | executed: true, player_id: -1, nbr_live: 3 |

|                                               Registers                                                |
|--------------------------------------------------------------------------------------------------------|
| R1:-1, R2:2, R3:0, R4:5, R5:0, R6:0, R7:0, R8:0, R9:0, R10:0, R11:0, R12:0, R13:0, R14:0, R15:0, R16:0 |


