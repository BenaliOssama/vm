.name "dwarf"
.description "robust bomber avoiding self-overwrite"

        sti r1, %:live, %1    # Initialize live argument safely
        and r1, %0, r1        # Set carry for jumps

live:   live %2
        ld %0, r2             # r2: initial bomb offset
        ld %4, r3             # r3: increment for bomb offset
        fork %:live           # spawn child processes

bomb:   sti r1, %:bombz, r2  # Write to safe area, not code
        add r3, r2, r2            # Update offset
        zjmp %:bomb               # Loop forever

# Safe bombing zone far from code
bombz: nop r10
