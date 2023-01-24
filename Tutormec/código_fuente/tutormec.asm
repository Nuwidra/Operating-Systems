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
%define current_char_index_array 0x500 ; Lista de los indices de char_object_array que se muestran en pantalla, por defecto únicamente tiene uno.
%define boxes_x_cordenates_array 0x500 ; Almacena una estructura de datos de dos bytes. 
                                       ; En el primer byte está el límite horizontal izquierdo de la caja
                                       ; y el segundo el límite horizontal derecho. De modo, que por cada caja hay dos bytes.
%define max_amount_chars 27 
%define chars_on_screen 1 
%define char_object_size 5 ; Lista de todas los objetos array, cada objeto array tiene un tamaño de 5bytes, 
                           ; donde se almacena: el carácter, la bandera de si fue presionada, 
                           ; la posición x, la posición y. E y de salida, en caso de que el programa 
                           ; mostrara más de un carácter.
%define next_char_on_screen 0x1 ;  Almacena el indice del siguiente caracter que se mostrara en la pantalla.

ORG TUTORMEC_RUN_OFS

BITS 16

;############## FIRST PAGE ###################
start:
    ; Se eliminó el segmento y el código de pila.
    call clear_screen_menu ; Se llama la llamada de limpiar el menu
    mov si, text_string ; Poner la posición de la cuerda en SI
    call print_string   ; Llame a nuestra rutina de impresión de cadenas
    mov ah,0h  ; bytes que se almacenan/inicializan en el almacenamiento en el registro de ah
    int 16h ; Forma abreviada de la interrupción 0x16
       ; Las opciones que tiene el menu
       cmp al, '1' ; Comparación del registro al con 1
       je start_tutormec ; Se inicia el tutormec
       cmp al, '2' ; Comparación del registro al con 2
       je reboot ; Se realiza un reboot
       cmp al, '3' ; Comparación del registro al con 3
       je shutdown ; Se cierra el programa
       cmp al, '4' ; Comparación del registro al con 4
       je about    ; Información sobre el programa

       jmp $            

; Solo bootea la el programa de manera para que regrese para seleccionar el SO
reboot:
mov ax, 0
int 19h

; Solamente se cierra el programa incializando los registros
shutdown:
mov ax, 0x1000
mov ax, ss
mov sp, 0xf000
mov ax, 0x5307
mov bx, 0x0001
mov cx, 0x0003
int 0x15

about:
call cls
mov si, about_string    ; Poner la posición de la cuerda en SI
call print_string   ; Llame a nuestra rutina de impresión de cadenas
call wait_for_key_press ; La espera para que llegue a presionar una tecla
je start

print_string:           ; Rutina: cadena de salida en SI a pantalla
    mov ah, 0Eh     ; función int 10h 'imprimir carácter'

.repeat:
    lodsb           ; Obtener carácter de cadena
    cmp al, 0
    je .done        ; Si char es cero, final de la cadena
    int 10h         ; De lo contrario, imprímelo
    jmp .repeat

.done:
    ret

cls:
  pusha
  mov ah, 0x00
  mov al, 0x03  ; modo texto 80x25 16 colores
  int 0x10
  popa
  ret

;############## SECCIONES DE UTILIDAD ###################

clear_screen_menu:
        pusha
        call cls
        mov ah, 06h    ; Función de desplazamiento hacia arriba
        xor al, al     ; Borrar toda la pantalla
        xor cx, cx     ; Esquina superior izquierda CH=fila, CL=columna
        mov dx, 184Fh  ; Esquina inferior derecha DH=fila, DL=columna
        mov bh, 7  
        int 10h
        popa
		ret wait_for_key_press ; La espera para que llegue a presionar una tecla


; La espera para que llegue a presionar una tecla
wait_for_key_press:
	pusha 
	mov ah, 00h
	int 16h
	popa 
	ret 

; Limpia la pantalla
clear_screen_tty:
    pusha ; Hacen referencia al mismo código de operación
    xor dx, dx ; La instrucción XOR realiza una operación OR exclusiva 
               ; bit a bit entre los bits correspondientes  al registro
    call move_cursor ; Se llama el movimiento del cursor
    mov bh, 7 
    mov al, 0 
    mov ah, 6 
    mov cx, 0 
    mov dx, 184Fh 
    int 10h 
    popa ; Restaura los registros generales a sus valores antes de ejecutar un pusha anterior
    ret  ; Se retorna el limpiado de la pantalla

hide_input_cursor:
    pusha ; Hacen referencia al mismo código de operación
    mov ch, 32 ; Opción invisible
    mov ah, 01h ; Función 01h
    mov al, 3 ; Modo vídeo
    int 10h
    popa ; Restaura los registros generales a sus valores antes de ejecutar un pusha anterior
    ret  ; Se retorna las entradas ocultas
    
; Movimiento del cursor
move_cursor:
    pusha ; Hacen referencia al mismo código de operación
    mov bh, 0 
    mov ah, 2 
    int 10h	
    popa ; Restaura los registros generales a sus valores antes de ejecutar un pusha anterior
    ret  ; Retornar dicho movimiento

; Se detecta que una tecla sea presionada en el transcurso de la ejecucion
detect_keypress:
    mov ah, 01h 
    int 16h
    jz is_buffer_empty ; En el caso que este basico el buffer
    xor ah, ah 
    int 16h
    jmp done_detect_keypress ; Detectar la pulsación de la tecla 
    is_buffer_empty:
        xor ax, ax  
    done_detect_keypress:   ; El retorno de la pulsación de la tecla
        ret 

; Se imprimen los valores numericos
print_numeric_number:
    add al, '0'
    mov ah, 0eh	
    int 0x10


short_delay:
    pusha ; Hacen referencia al mismo código de operación
    short_delayTimer:     
    mov ax, 10000
    long_loop_delay: 
        mov bx, 1000
    do_program_tick:
        dec bx ;  Se utiliza para decrementar un operando en uno
        jnz do_program_tick ; Saltar si no es igual Saltar si no es cero
        dec ax ;  Se utiliza para decrementar un operando en uno
        jnz long_loop_delay ; Saltar si no es igual Saltar si no es cero
        
    short_delayDone:
        popa ; Restaura los registros generales a sus valores antes de ejecutar un pusha anterior
    ret

debug_tty: ;input al
    push dx ; Coloca su operando en la parte superior de la pila compatible con hardware en la memoria
    mov dl, 5
    mov dh, 5
    call move_cursor ; Se llama el movimiento del cursor
    call print_tty_char  ; Se llama el print de los char
    pop dx ; Elimina el elemento de datos de 4 bytes de la parte superior de la pila compatible
    ret


print_string_tty:
	pusha ; Hacen referencia al mismo código de operación
	mov bp, sp 
	print_string_tty_loop:
		lodsb ; El tamaño de los operandos de origen y destino se selecciona con el mnemotécnico
		or al, al 
		jz print_string_tty_loop_done
		mov ah, 0eh	
		int 10h 
		jmp print_string_tty_loop 

	print_string_tty_loop_done:
		mov sp, bp
		popa ; Restaura los registros generales a sus valores antes de ejecutar un pusha anterior
		ret 

print_tty_char: ; Entrada al
	mov ah, 0eh	
	int 10h 
    ret ; Se retorna la impresión


; Devuelve el puntero del objeto de carácter en bx de acuerdo con la entrada bx
get_character_object:
    push dx ; Se pushea el registro dx
    push ax ; Se pushea el registro ax
    mov dx, bx
    dec dx
    mov al, char_object_size
    mul dl ;  ax = dl*al.
    
    mov bx, char_object_array
    add bx, ax 
    pop ax ; Se hace pop al registro ax
    pop dx ; Se hace pop al registro dx
    ret

; Entrada de dx
; Escribe caracteres donde quieras
print_number_tty:
    push ax ; Se pushea el registro ax
    push bx ; Se pushea el registro bx
    push cx ; Se pushea el registro cx
    push dx ; Se pushea el registro dx
    mov cx, 0
    mov bx, 10 
    push_numbers:
        mov dx, 0 
        div bx 
        push bx ; Se pushea el registro bx
        mov bx, cx
        mov byte[string_buffer + bx], dl ; Es indexar para obtener ese valor en la memoria y pasarlo al registro
        pop bx
        inc cx 
        test ax, ax 
        jnz push_numbers 
    pop dx
    inc cx
    mov ah, dl
    print_number_loop:
        mov bx, cx
        dec bx
        mov al, byte[string_buffer + bx] ; Es indexar para obtener ese valor en la memoria y pasarlo al registro ax
        add al, '0' 
        mov dl, ah
        sub dl, bl
        call move_cursor ; Llama del movimiento del cursor
        call print_tty_char ; Llamada de la impresion de los caracteres
        loop print_number_loop ; Impresion en un ciclo para los numeros
    pop cx ; Se hace pop al registro cx
    pop bx ; Se hace pop al registro bx
    pop ax
    ret 




;############## TUTORMEC  ###################

; El inicio del tutormec ya que hace una llamada a todas las anteriores funciones establecidas
start_tutormec:
    call initialize_data
    call show_initial_screen
    call hide_input_cursor
    call clear_screen_tty
    call draw_screen
    infinite_loop:

        call detect_keypress
        cmp ax, 011Bh
        je exit_tutormec
        call check_input_buffer
        call move_char_objects
        call clear_screen_tty
        call draw_screen 
        call short_delay
        jmp infinite_loop

; Para salir de tutormec solamente se limpia la pantalla y se llama a inicio
exit_tutormec:
	call clear_screen_tty
	jmp start 

 
 ;############## TUTORMEC INITIALIZE DATA###################

show_initial_screen:
   call cls
    mov si, welcome_string    ; Poner la posición de la cuerda en SI
    call print_string   ; Llame a nuestra rutina de impresión de cadenas
    call wait_for_key_press
    ret 

; Se inicializa los datos para el programa
initialize_data:
    call initialize_next_char_on_screen ; Se inicializa el siguiente caracter en la pantalla
    call initialize_current_char_index_array ; Inicializar matriz de índice de caracteres actual
    call initialize_boxes_x_cordenates_array ; Inicializar cajas x array cordenados
    ret ; Retorno de las llamadas establecidas

; Se inicializa los bloques con respecta a las coordenadas en X
initialize_boxes_x_cordenates_array:
    push dx
    mov dl, 4
    mov dh, 12
    mov word [boxes_x_cordenates_array + 0], dx
    mov dl, 20
    mov dh, 28
    mov word [boxes_x_cordenates_array + 2], dx
    mov dl, 36
    mov dh, 44
    mov word [boxes_x_cordenates_array + 4], dx
    mov dl, 52
    mov dh, 60
    mov word [boxes_x_cordenates_array + 6], dx
    mov dl, 68
    mov dh, 76
    mov word [boxes_x_cordenates_array + 8], dx
    pop dx
    ret;

; Se inicializa la siguiente letra en la pantalla
initialize_next_char_on_screen:
    push cx
    mov cl, chars_on_screen
    inc cl
    mov byte[next_char_on_screen], cl
    pop cx
    ret

initialize_current_char_index_array:
    pusha
    mov cx, chars_on_screen
    initialize_current_char_index_array_loop:
        mov bx, cx
        dec bx
        mov byte [current_char_index_array + bx], cl ; Llenar el primer caracter
        ; Es indexar para obtener ese valor en la memoria y pasarlo al registro 
        loop initialize_current_char_index_array_loop
    popa
    ret ;

 ;############## TUTORMEC CALCULATE MOVEMENT ###################

 ; Lee buffer de entrada y toma el primer caracter que encuentre. 
 ; Y verifica si el caracter es mismo que el de la pantalla. 
 ; De ser el caso, enciende la bandera del objecto char correspondiente.
check_input_buffer:
    pusha
    cmp ax, 0 
    je exit_check_input_buffer ; Saltar si cmp es igual
    cmp al, 97 ; al ASCII charter
    jl exit_check_input_buffer ; La instrucción es un salto condicional que sigue a una prueba.
    cmp al, 122 
    jg exit_check_input_buffer ; Salta si es mayor
    sub al, 32
    xor dx, dx 
    xor bx, bx 
    mov cx, chars_on_screen
     
    check_input_buffer_loop:
        mov bx, cx
        dec bx
        mov bx, [current_char_index_array + bx] ; Es indexar para obtener ese valor en la memoria y pasarlo al registro bx
        call get_character_object
        

        cmp al, byte [bx] 
        je set_character_captured_flag 
        loop check_input_buffer_loop

    jmp exit_check_input_buffer 

    set_character_captured_flag:
        mov byte [bx + 1], 1  ; Es indexar para obtener ese valor en la memoria y pasarlo al registro 
    exit_check_input_buffer:
        popa
        ret 


; Actualiza las cordenadas del objecto char que se encuentra en la pantalla: 
; Se mueve uno hacia la izquierda.
; Si la bandera esta encendida deja de moverse a la izquierda, y pasa a moverse hacia abajo.
; Si no se encuentra con ninguna caja entonces le suma uno al score. Y continua con la siguiente letra.
; Si se encuentra con hace lo mismo que el anterior, pero no suma nada al puntaje.
move_char_objects:
    pusha
    mov cx , max_amount_chars
    look_for_char_loop:
        mov bx, cx
        dec bx
        mov bx, [current_char_index_array + bx] ;La memoria estática y un valor variable de bx, es indexar para obtener ese valor en la memoria y pasarlo al registro de bx
        call get_character_object
        jmp move_char_object
        

        continue_look_for_char_loop:
            loop look_for_char_loop

    exitMove:
        popa
        ret 
    move_char_object:
        mov al, [ bx + 1]; obtener la bandera capturada
        cmp al , 1 
        jne is_horizontal_move ; Mover la letra hacia abajo
    
    ; El movimiento hacia abajo
    is_down_move:
        mov al, [bx + 3] ; obtener su posición
        mov ah, [bx + 4] ; obtener su posición antes de ser capturado
        cmp ah, 0
        jne continue_is_down_move
        mov byte [bx + 4], al ; establecer la posición y antes de ser capturado en y real
        
        continue_is_down_move:
        cmp al, 11 ; comparar si comienza bloque de impacto
        jne move_down_aux 
        xor dx, dx
        push cx
        mov cx, 5

    ; Basicamente es verificar la area de impacto la cual contempla el bloque del programa
    verify_impact_area_loop:
        push bx
        push ax
        mov bx, cx
        dec bx
        mov ax, 2
        mul bx
        mov bx, ax
        mov dx, word [boxes_x_cordenates_array + bx]
        pop ax
        pop bx

        cmp byte [bx + 2], dl ;Compare si char x es basura que min x area
        jl continue_verify_impact_area_loop ; si byte [bx + 2] < dl
        cmp byte [bx + 2], dh 
        jg continue_verify_impact_area_loop ; si el byte [bx + 2] > dh
        pop cx
        jmp set_new_char_object

        continue_verify_impact_area_loop:
        loop verify_impact_area_loop 
        pop cx

    move_down_aux:
        cmp al, 14
        jb move_down 
        add word[score_count], 1 ; Se añade un contador al puntaje del mismo movimiento hacia abajo
        jmp set_new_char_object
        
    is_horizontal_move:
        mov al, [bx + 2] ; obtener la posición x
        cmp al , 0 ; 
        ja move_left ; moverse si la posición x es mayor que 0
        jmp restart_char_object 
    move_left:
        dec al 
        mov [bx + 2] , al
        jmp continue_look_for_char_loop 
    
    
    set_new_char_object:
        
        push dx
        mov dl, byte [bx + 4] ; obtener Y después de ser capturado
        push bx
        mov dh, byte [next_char_on_screen]; obtener el siguiente carácter en la pantalla
        xor bx, bx
        mov bl, dh
        call get_character_object
        mov byte [bx + 3], dl ; establecer la nueva posición Y del objeto de carácter
        mov bx, cx
        dec bx 
        mov byte[current_char_index_array + bx], dh ; establecer el siguiente carácter en la pantalla
        inc dh
        cmp dh, 26
        jne set_new_char_continue ; si dh es 26, entonces se reinicia en 0
        xor dh, dh
        set_new_char_continue:
        mov byte [next_char_on_screen], dh
        pop bx
        pop dx
        jmp restart_char_object; 
        
    move_down:
        inc al 
        mov [bx + 3], al
        jmp continue_look_for_char_loop

    restart_char_object:
        mov byte [bx + 1], 0 ; establecer el indicador capturado en 0
        mov byte [bx + 2], 79 ; establece x en 79
        mov byte [bx + 3], 0 ; establecer el indicador capturado en 0
        mov byte [bx + 4], 0 ; establece x en 79
        jmp continue_look_for_char_loop


 ;############## TUTORMEC DRAW OBJECTS ###################

 ; Dibuja todos los objetos en la pantalla, el label, 
 ; la puntuación, las cajas y el carácter que se está moviendo.
draw_screen:
    call draw_boxes
    xor cx, cx 
    xor ax, ax 
    mov cx, chars_on_screen
    draw_characters_loop:

        mov bx, cx 
        dec bx
        mov bx, [current_char_index_array + bx] 

        
        call get_character_object 
    
        mov dl, byte[bx + 2] ;
        mov dh, byte[bx + 3] 
        call move_cursor
        mov al, [bx]
        call print_tty_char  
        
        loop draw_characters_loop
        pusha
        mov dl, 0
        mov dh, 22
        call move_cursor 
        mov si, score_label
        call print_string_tty 

        mov dl, 0
        mov dh, 22
        mov ax, [score_count] 
        call print_number_tty 

        popa
		ret 

; Se dibijan las cajas para que las letras puedan chocar
draw_boxes:
			pusha 
			mov bl, 120 
			mov si, 8; ancho de caja 6
	        mov di, 15 ;Inicia x
	        mov dh, 11 ;Fila 11
	        mov dl, 4 ;Colunma 5
	        call draw_box_area ; Se llama a draw_box_area
	        mov dl, 20 ; La cordenada del bloque 
	        call draw_box_area ; Se llama a draw_box_area
	        mov dl, 36 ; La cordenada del bloque 
	        call draw_box_area ; Se llama a draw_box_area
	        mov dl, 52 ; La cordenada del bloque 
	        call draw_box_area ; Se llama a draw_box_area
	        mov dl, 68 ; La cordenada del bloque 
	        call draw_box_area ; Se llama a draw_box_area
	        popa 
	        ret ; Se retorna dicha impresion


draw_box_area:
		pusha 
		draw:
			call move_cursor ; Se llama al cursor
			mov ah, 09h ; función 09h.
			mov bh, 0 ; establecer el valor de la página 0
			mov cx, si ; times 
			mov al, ' ' 
			int 10h ; imprime <al> carácter <si> Veces.

			inc dh ; Siguiente fila

			xor ax, ax
			mov al, dh
			cmp ax, di
			jne draw 

			popa 
			ret 


 ;############## INITIALIZED DATA ###################
welcome_string:  db 'Bienvenido a Tutormec',13,10,'Presionar [esc] para salir.',13,10,'Presionar [enter] para empezar.', 0
text_string db 'Nuwidra OS',13,10,'1) Tutormec',13,10,'2) Reboot',13,10,'3) Cerrar',13,10,'4) Acerca',0
about_string db 'Tutormec para el curso de Principios de Sistemas Operativos por Nuwidra',0
score_label db "Puntaje: ", 10, 0X0D\
              ,"Presionar con anular y dedo medio izquierdo", 0

string_buffer times 5 db 0
score_count dw 0

; Las opcioens posibles de letras que puedan surgir desde la esquina derecha de arriba de la pantalla
char_object_array: 
          db 'X', 0, 79, 0, 0 ; 23
		  db 'E', 0, 79, 9, 9 ; 4
          db 'A', 0, 79, 1, 1 ; 0
		  db 'R', 0, 79, 0, 0 ; 17
		  db 'N', 0, 79, 0, 0 ; 13
		  db 'V', 0, 79, 0, 0 ; 21
		  db 'K', 0, 79, 0, 0 ; 10
		  db 'F', 0, 79, 0, 0 ; 5
		  db 'G', 0, 79, 0, 0 ; 6
		  db 'Y', 0, 79, 0, 0 ; 24
		  db 'U', 0, 79, 0, 0 ; 20
		  db 'L', 0, 79, 0, 0 ; 11
		  db 'B', 0, 79, 3, 3 ; 1
		  db 'Z', 0, 79, 0, 0 ; 25 
		  db 'H', 0, 79, 0, 0 ; 7
		  db 'I', 0, 79, 0, 0 ; 8
		  db 'T', 0, 79, 0, 0 ; 19
		  db 'S', 0, 79, 0, 0 ; 18
		  db 'D', 0, 79, 7, 7 ; 3
		  db 'O', 0, 79, 0, 0 ; 14
		  db 'W', 0, 79, 0, 0 ; 22
		  db 'Q', 0, 79, 0, 0 ; 16
		  db 'P', 0, 79, 0, 0 ; 15
		  db 'M', 0, 79, 0, 0 ; 12
		  db 'C', 0, 79, 5, 5 ; 2
		  db 'J', 0, 79, 0, 0 ; 9