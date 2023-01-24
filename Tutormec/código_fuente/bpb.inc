    jmp boot_start
    TIMES 3-($-$$) DB 0x90   ; Support 2 or 3 byte encoded JMPs before BPB.

bpb_disk_info:
    sectorsPerTrack:   dw    18
    numHeads:          dw    2