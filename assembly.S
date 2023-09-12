
text-sec2.elf:     file format elf32-avr


Disassembly of section .sec1:

00000000 <.sec1>:
   0:	0c 94 34 00 	jmp	0x68	;  0x68 RESET
   4:	0c 94 3e 00 	jmp	0x7c	;  0x7c
   8:	0c 94 3e 00 	jmp	0x7c	;  0x7c
   c:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  10:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  14:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  18:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  1c:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  20:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  24:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  28:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  2c:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  30:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  34:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  38:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  3c:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  40:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  44:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  48:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  4c:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  50:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  54:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  58:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  5c:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  60:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  64:	0c 94 3e 00 	jmp	0x7c	;  0x7c
  68:	11 24       	eor	r1, r1
  6a:	1f be       	out	0x3f, r1	; 63
  6c:	cf ef       	ldi	r28, 0xFF	; 255
  6e:	d8 e0       	ldi	r29, 0x08	; 8
  70:	de bf       	out	0x3e, r29	; 62
  72:	cd bf       	out	0x3d, r28	; 61
  74:	0e 94 40 00 	call	0x80	;  0x80
  78:	0c 94 52 00 	jmp	0xa4	;  0xa4
  7c:	0c 94 00 00 	jmp	0	;  0x0
  80:	56 9a       	sbi	0x0a, 6	; 10
  82:	51 98       	cbi	0x0a, 1	; 10
  84:	59 9a       	sbi	0x0b, 1	; 11
  86:	49 99       	sbic	0x09, 1	; 9
  88:	02 c0       	rjmp	.+4      	;  0x8e
  8a:	5e 9a       	sbi	0x0b, 6	; 11
  8c:	01 c0       	rjmp	.+2      	;  0x90
  8e:	5e 98       	cbi	0x0b, 6	; 11
  90:	22 e6       	ldi	r18, 0x62	; 98
  92:	82 ef       	ldi	r24, 0xF2	; 242
  94:	90 e2       	ldi	r25, 0x20	; 32
  96:	21 50       	subi	r18, 0x01	; 1
  98:	80 40       	sbci	r24, 0x00	; 0
  9a:	90 40       	sbci	r25, 0x00	; 0
  9c:	e1 f7       	brne	.-8      	;  0x96
  9e:	00 c0       	rjmp	.+0      	;  0xa0
  a0:	00 c0       	rjmp	.+0      	;  0xa2
  a2:	f1 cf       	rjmp	.-30     	;  0x86
  a4:	f8 94       	cli
  a6:	ff cf       	rjmp	.-2      	;  0xa6
