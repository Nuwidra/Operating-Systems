; ======================================================================================
;    ____    ____           ___    ____   ___   ___
;   / ___/  / __ \         |__ \  / __ \ |__ \ |__ \ 
;   \__ \  / / / /  _____  __/ / / / / / __/ / __/ /
;  ___/ / / /_/ /  /____/ / __/ / /_/ / / __/ / __/
; /____/  \____/         /____/ \____/ /____//____/
;
; Instituto Tecnológico de Costa Rica
; Carrera: 
;         Bachillerato en Ingeniería en Computación
; Curso: 
;         Principios de Sistemas Operativos
; Profesor: 
;         Kevin Moraga García
; Alumno: 
;         Jonathan Quesada Salas
; Tarea Corta 2: 
;         Tutormec
;
; ======================================================================================

; ======================================================================================
; Ejecución en Terminal para el funcionamiento de la tarea:
;                       [1] "make"
;                       [2] "qemu-system-x86_64 bin/os.bin"
;                       [3] "sudo dd if=bin/os.bin of=/dev/sdb"
;                       [4] "sudo qemu-system-x86_64 -hda /dev/sdb"
; ======================================================================================
%include "tutormecInfo.inc"

TUTORMEC_LOAD_SEG  equ TUTORMEC_ABS_ADDR>>4
                                ; Segmento para comenzar a leer Stage2 en
                                ;     justo después del gestor de arranque
TUTORMEC_LBA_START equ 1          ; Dirección de bloque lógico (LBA) Etapa 2 comienza en
                                ;     LBA 1 = sector después del sector de arranque
TUTORMEC_LBA_END   equ TUTORMEC_LBA_START + NUM_TUTORMEC_SECTORS
                                ; Dirección de bloque lógico (LBA) Etapa 2 termina en
DISK_RETRIES     equ 3          ; Número de veces para reintentar en error de disco

bits 16
ORG 0x7c00

; Incluya un BPB (disquete de 1,44 MB con FAT12) para que sea más compatible con los medios de disquete USB
%include "bpb.inc"

boot_start:
    xor ax, ax                  ; DS=SS=ES=0 for stage2 loading
    mov ds, ax
    mov ss, ax                  ; Stack at 0x0000:0x7c00
    mov sp, 0x7c00
    cld                         ; Establecer instrucciones de cadena para usar el movimiento hacia adelante

    ; Lea Stage2 1 sector a la vez hasta que Stage2 esté completamente cargado
load_stage2:
    mov [bootDevice], dl        ; Guardar unidad de arranque
    mov di, TUTORMEC_LOAD_SEG     ;DI = Segmento actual para leer
    mov si, TUTORMEC_LBA_START    ; SI = LBA que la etapa 2 comienza en
    jmp .chk_for_last_lba       ; Comprueba si somos el último sector en la etapa 2

.read_sector_loop:
    mov bp, DISK_RETRIES        ; Establecer recuento de reintentos de disco

    call lba_to_chs             ; Convertir LBA actual a CHS
    mov es, di                  ; Establezca ES en el número de segmento actual para leer
    xor bx, bx                  ; Offset cero en segmento

.retry:
    mov ax, 0x0201              ; Función de llamada 0x02 de int 13h (sectores de lectura)
                                ;     AL = 1 = Sectores a leer
    int 0x13                    ; Llamada de interrupción de disco BIOS
    jc .disk_error              ; Si se establece CF, entonces error de disco

.success:
    add di, 512>>4              ; Avanzar al siguiente segmento de 512 bytes (0x20*16=512)
    inc si                      ; Siguiente LBA

.chk_for_last_lba:
    cmp si, TUTORMEC_LBA_END      ; ¿Hemos llegado al último sector de la etapa 2?
    jl .read_sector_loop        ;     Si no lo hemos hecho, lea el siguiente sector

.stage2_loaded:
    mov ax, TUTORMEC_RUN_SEG      ; Configure los segmentos apropiados para que Stage2 se ejecute
    mov ds, ax
    mov es, ax

    ; FAR JMP al punto de entrada Stage2 en la dirección física 0x07e00
    jmp TUTORMEC_RUN_SEG:TUTORMEC_RUN_OFS

.disk_error:
    xor ah, ah                  ; Int13h/AH=0 es reinicio de la unidad
    int 0x13
    dec bp                      ; Reducir el recuento de reintentos
    jge .retry                  ; Si no se supera el número de reintentos, vuelva a intentarlo

error_end:
    ; Error irrecuperable; error de la unidad de impresión; entrar en bucle infinito
    mov si, diskErrorMsg        ; Mostrar mensaje de error de disco
    call print_string
    cli
.error_loop:
    hlt
    jmp .error_loop


print_string:
    mov ah, 0x0e                
    xor bx, bx                 
    jmp .getch
.repeat:
    int 0x10                   
.getch:
    lodsb                       
    test al,al                  
    jnz .repeat                 
.end:
    ret

; Función: lba_to_chs
; Descripción: Traduce la dirección del bloque lógico a CHS (Cilindro, Cabeza, Sector).
; Funciona para todas las geometrías de disco compatibles con FAT12 válidas.
;
; Recursos: http://www.ctyme.com/intr/rb-0607.htm
; https://en.wikipedia.org/wiki/Logical_block_addressing#CHS_conversion
; https://stackoverflow.com/q/45434899/3857942
; Sector = (LBA mod SPT) + 1
; Cabeza = (LBA / SPT) mod CABEZAS
; Cilindro = (LBA / SPT) / CABEZAS
;
; Entradas: SI = LBA
; Salidas: DL = Número de unidad de arranque
;   DH = Cabeza
;   CH = Cilindro (8 puntas inferiores de cilindro de 10 puntas)
;   CL = Sector/Cilindro
;   2 bits superiores de Cilindros de 10 bits en 2 bits superiores de CL
; Sector en los 6 bits inferiores de CL
;
; Notas: Los registros de salida coinciden con las expectativas de Int 13h/AH=2 entradas
;
lba_to_chs:
    push ax                     ; Preservar AX
    mov ax, si                  ; Copiar LBA a AX
    xor dx, dx                  ; 16 bits superiores de valor de 32 bits establecidos en 0 para DIV
    div word [sectorsPerTrack]  ; DIV de 32 bits por 16 bits: LBA / SPT
    mov cl, dl                  ; CL = S = LBA mod SPT
    inc cl                      ; CL = S = (LBA mod SPT) + 1
    xor dx, dx                  ; 16 bits superiores de valor de 32 bits establecidos en 0 para DIV
    div word [numHeads]         ; 32-bit by 16-bit DIV : (LBA / SPT) / HEADS
    mov dh, dl                  ; DH = H = (LBA / SPT) mod HEADS
    mov dl, [bootDevice]        ; dispositivo de arranque, no es necesario configurarlo pero es conveniente
    mov ch, al                  ; CH = C(lower 8 bits) = (LBA / SPT) / HEADS
    shl ah, 6                   ; Guarde las 2 puntas superiores del Cilindro de 10 puntas en
    or  cl, ah                  ;     2 bits superiores de Sector (CL)
    pop ax                      ; Restaurar registros de scratch
    ret


bootDevice:      db 0x00
diskErrorMsg:    db "Unrecoverable disk error!", 0

; Rellene el sector de arranque a 510 bytes y agregue una firma de 
; arranque de 2 bytes para un total de 512 bytes
TIMES 510-($-$$) db  0
dw 0xaa55

; Se corre tutormec
NUM_TUTORMEC_SECTORS equ (tutormec_end-tutormec_start+511) / 512
                                ; Número de usos de etapa 2 de sectores de 512 bytes.

tutormec_start:
    ;     Size = tutormec_end-tutormec_start
    incbin "bin/tutormec.bin"

tutormec_end: