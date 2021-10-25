;include 'macroses.inc'
macro stos0
{
    xor al, al
    stosb
}

macro align value {
    rb (value - 1) - ($ + value - 1) mod value
}

macro mov op1, op2, op3
{
    if op3 eq
        mov op1, op2
    else
        mov op1, op2
        mov op2, op3
    end if
}

macro movstr
{
    local move
  move:
    lodsb
    stosb
    test al, al
    jnz move
}

;vpsubb ymm0, ymm0, [esi]    ; вычитания 32х запакованных байтов
;vpavgw ymm3, ymm0, ymm2     ; среднее 16 битных целых

format ELF64 executable 4

segment readable executable

entry main

main:
    ; Загрузка эффективного адреса сообщения в регистр.
    lea rdi, [msg]

    ; Установка длины выводимого сообщения.
    mov rax, 14

    ; Второй параметр системного вызова.
    ; count
    mov rdx, rax

    ; Первый параметр системного вызова.
    ; buf
    mov rsi, rdi

    ; Нулевой параметр системного вызова.
    ; fd
    ; Значение равно stdout
    mov rdi, 1

    ; Номер системного вызова
    ; Вызов write()
    ; Расположен в fs/read_write.c
    mov rax, 1

    syscall

    xor rdi, rdi
    
    ; Номер системного вызова
    ; Вызов exit()
    mov rax, 60
    syscall

segment readable writable

msg db 'Hello, wordl', 10, 0
