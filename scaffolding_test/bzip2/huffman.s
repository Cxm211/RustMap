	.file	"huffman.c"
	.text
.Ltext0:
	.file 0 "/root/rustmap/bzip2-real-test" "huffman.c"
	.p2align 4
	.globl	BZ2_hbMakeCodeLengths
	.type	BZ2_hbMakeCodeLengths, @function
BZ2_hbMakeCodeLengths:
.LVL0:
.LFB53:
	.file 1 "huffman.c"
	.loc 1 67 1 view -0
	.cfi_startproc
	.loc 1 67 1 is_stmt 0 view .LVU1
	endbr64
	pushq	%r15
	.cfi_def_cfa_offset 16
	.cfi_offset 15, -16
	pushq	%r14
	.cfi_def_cfa_offset 24
	.cfi_offset 14, -24
	pushq	%r13
	.cfi_def_cfa_offset 32
	.cfi_offset 13, -32
	pushq	%r12
	.cfi_def_cfa_offset 40
	.cfi_offset 12, -40
	pushq	%rbp
	.cfi_def_cfa_offset 48
	.cfi_offset 6, -48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	.cfi_offset 3, -56
	subq	$4096, %rsp
	.cfi_def_cfa_offset 4152
	orq	$0, (%rsp)
	subq	$1176, %rsp
	.cfi_def_cfa_offset 5328
	movl	$256, %r8d
	.loc 1 67 1 view .LVU2
	movq	%rdi, 32(%rsp)
	movslq	%edx, %rdi
.LVL1:
	.loc 1 67 1 view .LVU3
	leaq	1120(%rsp), %r9
	movl	%edi, 40(%rsp)
	movl	%ecx, 44(%rsp)
	movq	%fs:40, %rax
	movq	%rax, 5256(%rsp)
	xorl	%eax, %eax
	.loc 1 72 4 is_stmt 1 view .LVU4
	.loc 1 73 4 view .LVU5
	.loc 1 75 4 view .LVU6
	.loc 1 76 4 view .LVU7
	.loc 1 77 4 view .LVU8
	.loc 1 79 4 view .LVU9
.LVL2:
	.loc 1 79 18 view .LVU10
	testl	%edi, %edi
	jle	.L6
.LVL3:
	.p2align 4,,10
	.p2align 3
.L5:
	.loc 1 80 7 view .LVU11
	.loc 1 80 26 is_stmt 0 view .LVU12
	movl	(%rsi,%rax,4), %ecx
	.loc 1 80 50 view .LVU13
	movl	%ecx, %edx
	sall	$8, %edx
	testl	%ecx, %ecx
	cmove	%r8d, %edx
	.loc 1 80 19 view .LVU14
	movl	%edx, 4(%r9,%rax,4)
	.loc 1 79 32 is_stmt 1 view .LVU15
.LVL4:
	.loc 1 79 18 view .LVU16
	addq	$1, %rax
.LVL5:
	.loc 1 79 18 is_stmt 0 view .LVU17
	cmpq	%rdi, %rax
	jne	.L5
.L6:
	.loc 1 79 18 view .LVU18
	movl	40(%rsp), %ebx
	leaq	80(%rsp), %rax
	movq	%rax, 48(%rsp)
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 24(%rsp)
	leal	-1(%rbx), %eax
	leaq	1128(%rsp,%rax,4), %rdi
.LVL6:
	.loc 1 79 18 view .LVU19
	addq	$2, %rax
	movq	%rax, 56(%rsp)
	leal	1(%rbx), %eax
	movl	%eax, 64(%rsp)
	.loc 1 103 16 view .LVU20
	leal	-1(%rbx,%rbx), %eax
	movq	%rdi, 72(%rsp)
	movl	%eax, 68(%rsp)
.LVL7:
	.p2align 4,,10
	.p2align 3
.L3:
	.loc 1 82 4 is_stmt 1 view .LVU21
	.loc 1 84 7 view .LVU22
	.loc 1 85 7 view .LVU23
	.loc 1 87 7 view .LVU24
	.loc 1 91 21 is_stmt 0 view .LVU25
	movl	40(%rsp), %eax
	.loc 1 87 15 view .LVU26
	movl	$0, 80(%rsp)
	.loc 1 88 7 is_stmt 1 view .LVU27
	.loc 1 88 17 is_stmt 0 view .LVU28
	movl	$0, 1120(%rsp)
	.loc 1 89 7 is_stmt 1 view .LVU29
	.loc 1 89 17 is_stmt 0 view .LVU30
	movl	$-2, 3184(%rsp)
	.loc 1 91 7 is_stmt 1 view .LVU31
.LVL8:
	.loc 1 91 21 view .LVU32
	testl	%eax, %eax
	jle	.L1
	movq	48(%rsp), %r11
	movq	56(%rsp), %r12
	leaq	1120(%rsp), %rbp
	leaq	3184(%rsp), %rbx
	movq	%rbp, 16(%rsp)
	movl	$1, %r8d
	movq	%rbx, 8(%rsp)
.LVL9:
	.p2align 4,,10
	.p2align 3
.L10:
.LBB2:
	.loc 1 95 91 is_stmt 0 view .LVU33
	movl	%r8d, %edx
.LBE2:
	.loc 1 94 22 view .LVU34
	movl	%r8d, (%r11,%r8,4)
.LBB3:
	.loc 1 95 68 view .LVU35
	movl	0(%rbp,%r8,4), %r9d
	movl	%r8d, %r10d
.LVL10:
	.loc 1 95 68 view .LVU36
.LBE3:
	.loc 1 92 10 is_stmt 1 view .LVU37
.LBB4:
	.loc 1 95 91 is_stmt 0 view .LVU38
	sarl	%edx
.LVL11:
	.loc 1 95 91 view .LVU39
.LBE4:
	.loc 1 92 20 view .LVU40
	movl	$-1, (%rbx,%r8,4)
	.loc 1 93 10 is_stmt 1 view .LVU41
.LVL12:
	.loc 1 94 10 view .LVU42
.LBB5:
	.loc 1 95 12 view .LVU43
	.loc 1 95 27 view .LVU44
	.loc 1 95 39 view .LVU45
	.loc 1 95 55 view .LVU46
	.loc 1 95 74 view .LVU47
	.loc 1 95 87 is_stmt 0 view .LVU48
	movslq	%edx, %rax
	movslq	80(%rsp,%rax,4), %rax
	movq	%rax, %rcx
	.loc 1 95 74 view .LVU49
	cmpl	1120(%rsp,%rax,4), %r9d
	jge	.L31
.LBE5:
	.loc 1 93 15 view .LVU50
	movslq	%r8d, %rax
	jmp	.L9
.LVL13:
	.p2align 4,,10
	.p2align 3
.L32:
.LBB6:
	.loc 1 95 91 view .LVU51
	movl	%esi, %edx
.LVL14:
.L9:
	.loc 1 95 101 is_stmt 1 discriminator 3 view .LVU52
	.loc 1 95 91 is_stmt 0 discriminator 3 view .LVU53
	movl	%edx, %esi
	.loc 1 95 110 discriminator 3 view .LVU54
	movl	%ecx, 80(%rsp,%rax,4)
	.loc 1 95 127 is_stmt 1 discriminator 3 view .LVU55
.LVL15:
	.loc 1 95 74 discriminator 3 view .LVU56
	.loc 1 95 91 is_stmt 0 discriminator 3 view .LVU57
	sarl	%esi
	.loc 1 95 87 discriminator 3 view .LVU58
	movslq	%esi, %rax
	movslq	80(%rsp,%rax,4), %rdi
	movslq	%edx, %rax
	movq	%rdi, %rcx
	.loc 1 95 74 discriminator 3 view .LVU59
	cmpl	%r9d, 1120(%rsp,%rdi,4)
	jg	.L32
.LVL16:
.L8:
	.loc 1 95 139 is_stmt 1 discriminator 4 view .LVU60
	.loc 1 95 148 is_stmt 0 discriminator 4 view .LVU61
	movslq	%edx, %rdx
.LBE6:
	.loc 1 91 21 discriminator 4 view .LVU62
	addq	$1, %r8
.LVL17:
.LBB7:
	.loc 1 95 148 discriminator 4 view .LVU63
	movl	%r10d, 80(%rsp,%rdx,4)
.LBE7:
	.loc 1 95 156 is_stmt 1 discriminator 4 view .LVU64
	.loc 1 91 36 discriminator 4 view .LVU65
.LVL18:
	.loc 1 91 21 discriminator 4 view .LVU66
	cmpq	%r12, %r8
	jne	.L10
.LVL19:
	.loc 1 109 156 view .LVU67
	.loc 1 100 20 view .LVU68
	movl	40(%rsp), %eax
	cmpl	$1, %eax
	je	.L11
	movslq	64(%rsp), %rbp
	movq	24(%rsp), %r12
	leal	-2(%rax), %r9d
	movl	%eax, %r8d
.LVL20:
	.p2align 4,,10
	.p2align 3
.L20:
	.loc 1 101 10 view .LVU69
	.loc 1 101 38 is_stmt 0 view .LVU70
	movl	(%r12), %r13d
	.loc 1 101 52 view .LVU71
	subl	$1, %r8d
.LVL21:
	.loc 1 101 13 view .LVU72
	movslq	84(%rsp), %r14
.LVL22:
	.loc 1 101 24 is_stmt 1 view .LVU73
	.loc 1 101 32 is_stmt 0 view .LVU74
	movl	%r13d, 84(%rsp)
	.loc 1 101 47 is_stmt 1 view .LVU75
.LVL23:
.LBB8:
	.loc 1 101 58 view .LVU76
	.loc 1 101 77 view .LVU77
	.loc 1 101 85 view .LVU78
	.loc 1 101 101 view .LVU79
	.loc 1 101 121 view .LVU80
	.loc 1 101 135 view .LVU81
	.loc 1 101 138 is_stmt 0 view .LVU82
	cmpl	$1, %r8d
	je	.L33
	.loc 1 101 231 view .LVU83
	movslq	%r13d, %rax
	.loc 1 101 124 view .LVU84
	movl	$2, %edx
	.loc 1 101 80 view .LVU85
	movl	$1, %ecx
	.loc 1 101 231 view .LVU86
	movl	1120(%rsp,%rax,4), %ebx
	jmp	.L14
.LVL24:
	.p2align 4,,10
	.p2align 3
.L64:
	.loc 1 101 173 view .LVU87
	movl	%edx, %eax
.LVL25:
.L13:
	.loc 1 101 221 is_stmt 1 discriminator 8 view .LVU88
	.loc 1 101 224 is_stmt 0 discriminator 8 view .LVU89
	cmpl	%esi, %ebx
	jl	.L12
.L65:
	.loc 1 101 264 is_stmt 1 discriminator 10 view .LVU90
	.loc 1 101 124 is_stmt 0 discriminator 10 view .LVU91
	leal	(%rax,%rax), %edx
	.loc 1 101 273 discriminator 10 view .LVU92
	movl	%edi, 80(%rsp,%rcx,4)
	.loc 1 101 285 is_stmt 1 discriminator 10 view .LVU93
.LVL26:
	.loc 1 101 107 discriminator 10 view .LVU94
	.loc 1 101 101 discriminator 10 view .LVU95
	.loc 1 101 121 discriminator 10 view .LVU96
	.loc 1 101 135 discriminator 10 view .LVU97
	movslq	%eax, %rcx
	.loc 1 101 138 is_stmt 0 discriminator 10 view .LVU98
	cmpl	%edx, %r8d
	jl	.L12
.LVL27:
.L14:
	.loc 1 101 158 is_stmt 1 discriminator 3 view .LVU99
	.loc 1 101 208 is_stmt 0 discriminator 3 view .LVU100
	movslq	%edx, %rax
	movslq	80(%rsp,%rax,4), %rax
	.loc 1 101 203 discriminator 3 view .LVU101
	movl	1120(%rsp,%rax,4), %esi
	.loc 1 101 208 discriminator 3 view .LVU102
	movq	%rax, %rdi
	movl	%edx, %eax
	.loc 1 101 161 discriminator 3 view .LVU103
	cmpl	%edx, %r8d
	jle	.L13
	.loc 1 101 190 discriminator 4 view .LVU104
	addl	$1, %eax
	.loc 1 101 187 discriminator 4 view .LVU105
	movslq	%eax, %r10
	movslq	80(%rsp,%r10,4), %r11
	movq	%r11, %r10
	.loc 1 101 182 discriminator 4 view .LVU106
	movl	1120(%rsp,%r11,4), %r11d
	.loc 1 101 173 discriminator 4 view .LVU107
	cmpl	%esi, %r11d
	jge	.L64
	movl	%r11d, %esi
	movl	%r10d, %edi
.LVL28:
	.loc 1 101 221 is_stmt 1 view .LVU108
	.loc 1 101 224 is_stmt 0 view .LVU109
	cmpl	%esi, %ebx
	jge	.L65
.LVL29:
	.p2align 4,,10
	.p2align 3
.L12:
	.loc 1 101 296 is_stmt 1 discriminator 9 view .LVU110
	.loc 1 101 305 is_stmt 0 discriminator 9 view .LVU111
	movl	%r13d, 80(%rsp,%rcx,4)
.LBE8:
	.loc 1 101 313 is_stmt 1 discriminator 9 view .LVU112
	.loc 1 102 10 discriminator 9 view .LVU113
	.loc 1 102 38 is_stmt 0 discriminator 9 view .LVU114
	movl	-4(%r12), %r15d
	.loc 1 102 13 discriminator 9 view .LVU115
	movslq	84(%rsp), %r13
.LVL30:
	.loc 1 102 24 is_stmt 1 discriminator 9 view .LVU116
	.loc 1 102 32 is_stmt 0 discriminator 9 view .LVU117
	movl	%r15d, 84(%rsp)
.LVL31:
	.loc 1 102 47 is_stmt 1 discriminator 9 view .LVU118
.LBB9:
	.loc 1 102 58 discriminator 9 view .LVU119
	.loc 1 102 77 discriminator 9 view .LVU120
	.loc 1 102 85 discriminator 9 view .LVU121
	.loc 1 102 101 discriminator 9 view .LVU122
	.loc 1 102 121 discriminator 9 view .LVU123
	.loc 1 102 135 discriminator 9 view .LVU124
	.loc 1 102 138 is_stmt 0 discriminator 9 view .LVU125
	cmpl	$1, %r9d
	jle	.L37
	.loc 1 102 231 view .LVU126
	movslq	%r15d, %rax
	.loc 1 102 124 view .LVU127
	movl	$2, %edx
	.loc 1 102 80 view .LVU128
	movl	$1, %ecx
	.loc 1 102 231 view .LVU129
	movl	1120(%rsp,%rax,4), %ebx
	jmp	.L17
.LVL32:
	.p2align 4,,10
	.p2align 3
.L66:
	.loc 1 102 173 view .LVU130
	movl	%edx, %eax
.LVL33:
.L16:
	.loc 1 102 221 is_stmt 1 discriminator 8 view .LVU131
	.loc 1 102 224 is_stmt 0 discriminator 8 view .LVU132
	cmpl	%esi, %ebx
	jl	.L15
.L67:
	.loc 1 102 264 is_stmt 1 discriminator 10 view .LVU133
	.loc 1 102 124 is_stmt 0 discriminator 10 view .LVU134
	leal	(%rax,%rax), %edx
	.loc 1 102 273 discriminator 10 view .LVU135
	movl	%edi, 80(%rsp,%rcx,4)
	.loc 1 102 285 is_stmt 1 discriminator 10 view .LVU136
.LVL34:
	.loc 1 102 107 discriminator 10 view .LVU137
	.loc 1 102 101 discriminator 10 view .LVU138
	.loc 1 102 121 discriminator 10 view .LVU139
	.loc 1 102 135 discriminator 10 view .LVU140
	movslq	%eax, %rcx
	.loc 1 102 138 is_stmt 0 discriminator 10 view .LVU141
	cmpl	%r9d, %edx
	jg	.L15
.LVL35:
.L17:
	.loc 1 102 158 is_stmt 1 discriminator 3 view .LVU142
	.loc 1 102 208 is_stmt 0 discriminator 3 view .LVU143
	movslq	%edx, %rax
	movslq	80(%rsp,%rax,4), %rax
	.loc 1 102 203 discriminator 3 view .LVU144
	movl	1120(%rsp,%rax,4), %esi
	.loc 1 102 208 discriminator 3 view .LVU145
	movq	%rax, %rdi
	movl	%edx, %eax
	.loc 1 102 161 discriminator 3 view .LVU146
	cmpl	%edx, %r9d
	jle	.L16
	.loc 1 102 190 discriminator 4 view .LVU147
	addl	$1, %eax
	.loc 1 102 187 discriminator 4 view .LVU148
	movslq	%eax, %r10
	movslq	80(%rsp,%r10,4), %r11
	movq	%r11, %r10
	.loc 1 102 182 discriminator 4 view .LVU149
	movl	1120(%rsp,%r11,4), %r11d
	.loc 1 102 173 discriminator 4 view .LVU150
	cmpl	%esi, %r11d
	jge	.L66
	movl	%r11d, %esi
	movl	%r10d, %edi
.LVL36:
	.loc 1 102 221 is_stmt 1 view .LVU151
	.loc 1 102 224 is_stmt 0 view .LVU152
	cmpl	%esi, %ebx
	jge	.L67
.LVL37:
	.p2align 4,,10
	.p2align 3
.L15:
	.loc 1 102 296 is_stmt 1 discriminator 9 view .LVU153
.LBE9:
	.loc 1 105 36 is_stmt 0 discriminator 9 view .LVU154
	movl	1120(%rsp,%r14,4), %eax
.LBB10:
	.loc 1 102 305 discriminator 9 view .LVU155
	movl	%r15d, 80(%rsp,%rcx,4)
.LBE10:
	.loc 1 102 313 is_stmt 1 discriminator 9 view .LVU156
	.loc 1 103 10 discriminator 9 view .LVU157
	.loc 1 103 16 is_stmt 0 discriminator 9 view .LVU158
	movl	%ebp, %edi
.LVL38:
	.loc 1 104 10 is_stmt 1 discriminator 9 view .LVU159
	.loc 1 105 64 is_stmt 0 discriminator 9 view .LVU160
	movl	1120(%rsp,%r13,4), %edx
	.loc 1 104 34 discriminator 9 view .LVU161
	movl	%ebp, 3184(%rsp,%r13,4)
	.loc 1 105 108 discriminator 9 view .LVU162
	movzbl	%al, %ecx
	.loc 1 104 21 discriminator 9 view .LVU163
	movl	%ebp, 3184(%rsp,%r14,4)
	.loc 1 105 10 is_stmt 1 discriminator 9 view .LVU164
	.loc 1 105 140 is_stmt 0 discriminator 9 view .LVU165
	movzbl	%dl, %r10d
	.loc 1 108 22 discriminator 9 view .LVU166
	movl	%ebp, -4(%r12)
	.loc 1 105 187 discriminator 9 view .LVU167
	cmpl	%ecx, %r10d
	cmovl	%ecx, %r10d
	.loc 1 105 70 discriminator 9 view .LVU168
	xorb	%dl, %dl
	.loc 1 105 42 discriminator 9 view .LVU169
	xorb	%al, %al
	.loc 1 105 55 discriminator 9 view .LVU170
	addl	%edx, %eax
.LBB11:
	.loc 1 109 91 discriminator 9 view .LVU171
	movl	%r8d, %edx
.LBE11:
	.loc 1 105 90 discriminator 9 view .LVU172
	addl	$1, %r10d
.LBB12:
	.loc 1 109 91 discriminator 9 view .LVU173
	sarl	%edx
.LBE12:
	.loc 1 105 85 discriminator 9 view .LVU174
	orl	%eax, %r10d
	.loc 1 105 25 discriminator 9 view .LVU175
	movq	16(%rsp), %rax
	movl	%r10d, (%rax,%rbp,4)
	.loc 1 106 10 is_stmt 1 discriminator 9 view .LVU176
	.loc 1 106 25 is_stmt 0 discriminator 9 view .LVU177
	movq	8(%rsp), %rax
	movl	$-1, (%rax,%rbp,4)
	.loc 1 107 10 is_stmt 1 discriminator 9 view .LVU178
.LVL39:
	.loc 1 108 10 discriminator 9 view .LVU179
.LBB13:
	.loc 1 109 12 discriminator 9 view .LVU180
	.loc 1 109 27 discriminator 9 view .LVU181
	.loc 1 109 39 discriminator 9 view .LVU182
	.loc 1 109 55 discriminator 9 view .LVU183
	.loc 1 109 74 discriminator 9 view .LVU184
	.loc 1 109 87 is_stmt 0 discriminator 9 view .LVU185
	movslq	%edx, %rax
	movslq	80(%rsp,%rax,4), %rax
	movq	%rax, %rcx
	.loc 1 109 74 discriminator 9 view .LVU186
	cmpl	1120(%rsp,%rax,4), %r10d
	jge	.L41
	.loc 1 109 74 discriminator 9 view .LVU187
.LBE13:
	.loc 1 101 52 view .LVU188
	movslq	%r8d, %rax
	jmp	.L19
.LVL40:
	.p2align 4,,10
	.p2align 3
.L42:
.LBB14:
	.loc 1 109 91 view .LVU189
	movl	%esi, %edx
.LVL41:
.L19:
	.loc 1 109 101 is_stmt 1 discriminator 3 view .LVU190
	.loc 1 109 91 is_stmt 0 discriminator 3 view .LVU191
	movl	%edx, %esi
	.loc 1 109 110 discriminator 3 view .LVU192
	movl	%ecx, 80(%rsp,%rax,4)
	.loc 1 109 127 is_stmt 1 discriminator 3 view .LVU193
.LVL42:
	.loc 1 109 74 discriminator 3 view .LVU194
	.loc 1 109 91 is_stmt 0 discriminator 3 view .LVU195
	sarl	%esi
	.loc 1 109 87 discriminator 3 view .LVU196
	movslq	%esi, %rax
	movslq	80(%rsp,%rax,4), %r11
	movslq	%edx, %rax
	movq	%r11, %rcx
	.loc 1 109 74 discriminator 3 view .LVU197
	cmpl	1120(%rsp,%r11,4), %r10d
	jl	.L42
.L18:
	.loc 1 109 139 is_stmt 1 discriminator 4 view .LVU198
	.loc 1 109 148 is_stmt 0 discriminator 4 view .LVU199
	movslq	%edx, %rdx
	.loc 1 109 148 discriminator 4 view .LVU200
.LBE14:
	.loc 1 100 20 discriminator 4 view .LVU201
	addq	$1, %rbp
	subq	$4, %r12
	subl	$1, %r9d
.LBB15:
	.loc 1 109 148 discriminator 4 view .LVU202
	movl	%edi, 80(%rsp,%rdx,4)
	.loc 1 109 148 discriminator 4 view .LVU203
.LBE15:
	.loc 1 109 156 is_stmt 1 discriminator 4 view .LVU204
	.loc 1 100 20 discriminator 4 view .LVU205
	cmpl	$1, %r8d
	jne	.L20
	.loc 1 112 9 view .LVU206
	.loc 1 112 12 is_stmt 0 view .LVU207
	cmpl	$515, 68(%rsp)
	jle	.L11
	.loc 1 112 36 is_stmt 1 view .LVU208
	movl	$2002, %edi
.LVL43:
	.loc 1 112 36 is_stmt 0 view .LVU209
	call	BZ2_bz__AssertH__fail@PLT
.LVL44:
	.loc 1 115 21 is_stmt 1 view .LVU210
.L11:
	.loc 1 115 21 is_stmt 0 view .LVU211
	movq	32(%rsp), %r8
	movl	40(%rsp), %r9d
	.loc 1 120 34 view .LVU212
	movl	$1, %ecx
	xorl	%esi, %esi
	movl	44(%rsp), %r10d
	movq	8(%rsp), %r11
	movl	$1, %edi
	.p2align 4,,10
	.p2align 3
.L22:
.LVL45:
	.loc 1 118 27 is_stmt 1 view .LVU213
	.loc 1 118 23 is_stmt 0 view .LVU214
	movslq	(%r11,%rcx,4), %rax
	.loc 1 116 12 view .LVU215
	xorl	%edx, %edx
	.loc 1 118 27 view .LVU216
	testl	%eax, %eax
	js	.L68
.LVL46:
	.p2align 4,,10
	.p2align 3
.L23:
	.loc 1 118 35 is_stmt 1 discriminator 2 view .LVU217
	.loc 1 118 50 discriminator 2 view .LVU218
	.loc 1 118 23 is_stmt 0 discriminator 2 view .LVU219
	movslq	3184(%rsp,%rax,4), %rax
.LVL47:
	.loc 1 118 51 discriminator 2 view .LVU220
	addl	$1, %edx
.LVL48:
	.loc 1 118 27 is_stmt 1 discriminator 2 view .LVU221
	testl	%eax, %eax
	jns	.L23
	.loc 1 119 19 is_stmt 0 view .LVU222
	movl	%edx, %eax
.LVL49:
.L26:
	.loc 1 119 10 is_stmt 1 view .LVU223
	.loc 1 120 34 is_stmt 0 view .LVU224
	cmpl	%edx, %r10d
	.loc 1 119 19 view .LVU225
	movb	%al, -1(%r8,%rcx)
	.loc 1 120 10 is_stmt 1 view .LVU226
	.loc 1 120 34 is_stmt 0 view .LVU227
	cmovl	%edi, %esi
.LVL50:
	.loc 1 115 36 is_stmt 1 view .LVU228
	.loc 1 115 21 view .LVU229
	addq	$1, %rcx
.LVL51:
	.loc 1 115 21 is_stmt 0 view .LVU230
	cmpl	%ecx, %r9d
	jge	.L22
	.loc 1 123 7 is_stmt 1 view .LVU231
	.loc 1 123 10 is_stmt 0 view .LVU232
	testb	%sil, %sil
	je	.L1
.LVL52:
	.loc 1 142 21 is_stmt 1 view .LVU233
	movq	72(%rsp), %rsi
.LVL53:
	.loc 1 142 21 is_stmt 0 view .LVU234
	leaq	1124(%rsp), %rdx
.LVL54:
	.p2align 4,,10
	.p2align 3
.L27:
	.loc 1 143 10 is_stmt 1 discriminator 3 view .LVU235
	.loc 1 144 10 discriminator 3 view .LVU236
	.loc 1 143 12 is_stmt 0 discriminator 3 view .LVU237
	movl	(%rdx), %eax
	.loc 1 142 21 discriminator 3 view .LVU238
	addq	$4, %rdx
.LVL55:
	.loc 1 143 12 discriminator 3 view .LVU239
	movl	%eax, %ecx
	.loc 1 144 21 discriminator 3 view .LVU240
	shrl	$31, %eax
.LVL56:
	.loc 1 143 12 discriminator 3 view .LVU241
	sarl	$8, %ecx
.LVL57:
	.loc 1 144 21 discriminator 3 view .LVU242
	addl	%ecx, %eax
	sarl	%eax
.LVL58:
	.loc 1 145 10 is_stmt 1 discriminator 3 view .LVU243
	.loc 1 144 12 is_stmt 0 discriminator 3 view .LVU244
	addl	$1, %eax
.LVL59:
	.loc 1 145 24 discriminator 3 view .LVU245
	sall	$8, %eax
.LVL60:
	.loc 1 145 24 discriminator 3 view .LVU246
	movl	%eax, -4(%rdx)
.LVL61:
	.loc 1 142 36 is_stmt 1 discriminator 3 view .LVU247
	.loc 1 142 21 discriminator 3 view .LVU248
	cmpq	%rdx, %rsi
	jne	.L27
	jmp	.L3
.LVL62:
	.p2align 4,,10
	.p2align 3
.L68:
	.loc 1 118 27 is_stmt 0 view .LVU249
	xorl	%eax, %eax
	jmp	.L26
.LVL63:
	.p2align 4,,10
	.p2align 3
.L41:
	.loc 1 101 52 view .LVU250
	movl	%r8d, %edx
	jmp	.L18
.LVL64:
	.p2align 4,,10
	.p2align 3
.L31:
	.loc 1 93 15 view .LVU251
	movl	%r8d, %edx
	jmp	.L8
.LVL65:
.L33:
.LBB16:
	.loc 1 101 80 view .LVU252
	movl	$1, %ecx
	jmp	.L12
.LVL66:
.L37:
	.loc 1 101 80 view .LVU253
.LBE16:
.LBB17:
	.loc 1 102 80 view .LVU254
	movl	$1, %ecx
	jmp	.L15
.LVL67:
.L1:
	.loc 1 102 80 view .LVU255
.LBE17:
	.loc 1 148 1 view .LVU256
	movq	5256(%rsp), %rax
	subq	%fs:40, %rax
	jne	.L69
	addq	$5272, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%rbp
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	ret
.L69:
	.cfi_restore_state
	call	__stack_chk_fail@PLT
.LVL68:
	.cfi_endproc
.LFE53:
	.size	BZ2_hbMakeCodeLengths, .-BZ2_hbMakeCodeLengths
	.p2align 4
	.globl	BZ2_hbAssignCodes
	.type	BZ2_hbAssignCodes, @function
BZ2_hbAssignCodes:
.LVL69:
.LFB54:
	.loc 1 157 1 is_stmt 1 view -0
	.cfi_startproc
	.loc 1 157 1 is_stmt 0 view .LVU258
	endbr64
	.loc 1 158 4 is_stmt 1 view .LVU259
	.loc 1 160 4 view .LVU260
.LVL70:
	.loc 1 161 4 view .LVU261
	.loc 1 161 23 view .LVU262
	.loc 1 157 1 is_stmt 0 view .LVU263
	movq	%rdi, %r10
	movq	%rsi, %rdi
.LVL71:
	.loc 1 161 23 view .LVU264
	cmpl	%ecx, %edx
	jg	.L70
	leal	1(%rcx), %r11d
	.loc 1 160 8 view .LVU265
	xorl	%esi, %esi
.LVL72:
	.loc 1 160 8 view .LVU266
	movslq	%r8d, %r9
.LVL73:
	.p2align 4,,10
	.p2align 3
.L72:
	.loc 1 162 21 is_stmt 1 view .LVU267
	xorl	%eax, %eax
	testl	%r8d, %r8d
	jle	.L76
.LVL74:
	.p2align 4,,10
	.p2align 3
.L74:
	.loc 1 163 10 view .LVU268
	.loc 1 163 20 is_stmt 0 view .LVU269
	movzbl	(%rdi,%rax), %ecx
	.loc 1 163 13 view .LVU270
	cmpl	%edx, %ecx
	jne	.L73
	.loc 1 163 32 is_stmt 1 discriminator 1 view .LVU271
	.loc 1 163 40 is_stmt 0 discriminator 1 view .LVU272
	movl	%esi, (%r10,%rax,4)
	.loc 1 163 47 is_stmt 1 discriminator 1 view .LVU273
	.loc 1 163 50 is_stmt 0 discriminator 1 view .LVU274
	addl	$1, %esi
.LVL75:
.L73:
	.loc 1 162 35 is_stmt 1 discriminator 2 view .LVU275
	.loc 1 162 21 discriminator 2 view .LVU276
	addq	$1, %rax
.LVL76:
	.loc 1 162 21 is_stmt 0 discriminator 2 view .LVU277
	cmpq	%rax, %r9
	jne	.L74
.L76:
	.loc 1 163 55 is_stmt 1 discriminator 2 view .LVU278
	.loc 1 164 7 discriminator 2 view .LVU279
	.loc 1 161 35 is_stmt 0 discriminator 2 view .LVU280
	addl	$1, %edx
.LVL77:
	.loc 1 164 11 discriminator 2 view .LVU281
	addl	%esi, %esi
.LVL78:
	.loc 1 161 35 is_stmt 1 discriminator 2 view .LVU282
	.loc 1 161 23 discriminator 2 view .LVU283
	cmpl	%r11d, %edx
	jne	.L72
.LVL79:
.L70:
	.loc 1 166 1 is_stmt 0 view .LVU284
	ret
	.cfi_endproc
.LFE54:
	.size	BZ2_hbAssignCodes, .-BZ2_hbAssignCodes
	.p2align 4
	.globl	BZ2_hbCreateDecodeTables
	.type	BZ2_hbCreateDecodeTables, @function
BZ2_hbCreateDecodeTables:
.LVL80:
.LFB55:
	.loc 1 177 1 is_stmt 1 view -0
	.cfi_startproc
	.loc 1 177 1 is_stmt 0 view .LVU286
	endbr64
	pushq	%r13
	.cfi_def_cfa_offset 16
	.cfi_offset 13, -16
	movq	%rsi, %r11
	.loc 1 178 4 is_stmt 1 view .LVU287
	.loc 1 180 4 view .LVU288
.LVL81:
	.loc 1 181 4 view .LVU289
	.loc 1 181 23 view .LVU290
	.loc 1 177 1 is_stmt 0 view .LVU291
	movq	%rdi, %r10
	movq	%rcx, %rsi
.LVL82:
	.loc 1 177 1 view .LVU292
	pushq	%r12
	.cfi_def_cfa_offset 24
	.cfi_offset 12, -24
	pushq	%rbp
	.cfi_def_cfa_offset 32
	.cfi_offset 6, -32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	.cfi_offset 3, -40
	.loc 1 177 1 view .LVU293
	movl	%r8d, %ebx
	movl	40(%rsp), %ebp
	.loc 1 181 23 view .LVU294
	cmpl	%r9d, %r8d
	jg	.L80
	movl	%r8d, %ecx
.LVL83:
	.loc 1 181 23 view .LVU295
	movq	%rdx, %r12
	leal	1(%r9), %r13d
	.loc 1 180 7 view .LVU296
	xorl	%edi, %edi
.LVL84:
	.loc 1 180 7 view .LVU297
	movslq	%ebp, %r8
.LVL85:
	.p2align 4,,10
	.p2align 3
.L81:
	.loc 1 182 21 is_stmt 1 view .LVU298
	xorl	%eax, %eax
	testl	%ebp, %ebp
	jle	.L85
.LVL86:
	.p2align 4,,10
	.p2align 3
.L83:
	.loc 1 183 10 view .LVU299
	.loc 1 183 20 is_stmt 0 view .LVU300
	movzbl	(%rsi,%rax), %edx
	.loc 1 183 13 view .LVU301
	cmpl	%ecx, %edx
	jne	.L82
	.loc 1 183 32 is_stmt 1 discriminator 1 view .LVU302
	.loc 1 183 36 is_stmt 0 discriminator 1 view .LVU303
	movslq	%edi, %rdx
	.loc 1 183 48 discriminator 1 view .LVU304
	addl	$1, %edi
.LVL87:
	.loc 1 183 41 discriminator 1 view .LVU305
	movl	%eax, (%r12,%rdx,4)
	.loc 1 183 46 is_stmt 1 discriminator 1 view .LVU306
.LVL88:
.L82:
	.loc 1 182 35 discriminator 2 view .LVU307
	.loc 1 182 21 discriminator 2 view .LVU308
	addq	$1, %rax
.LVL89:
	.loc 1 182 21 is_stmt 0 discriminator 2 view .LVU309
	cmpq	%rax, %r8
	jne	.L83
.L85:
	.loc 1 181 35 is_stmt 1 discriminator 2 view .LVU310
	addl	$1, %ecx
.LVL90:
	.loc 1 181 23 discriminator 2 view .LVU311
	cmpl	%r13d, %ecx
	jne	.L81
.LVL91:
.L80:
	.loc 1 185 37 is_stmt 0 view .LVU312
	leaq	8(%r11), %rdi
	movq	%r11, %rcx
	xorl	%eax, %eax
	movq	$0, (%r11)
	andq	$-8, %rdi
	movq	$0, 84(%r11)
	subq	%rdi, %rcx
	addl	$92, %ecx
	shrl	$3, %ecx
	rep stosq
.LVL92:
	.loc 1 186 18 is_stmt 1 view .LVU313
	testl	%ebp, %ebp
	jle	.L86
	.loc 1 186 18 is_stmt 0 view .LVU314
	leal	-1(%rbp), %eax
	leaq	1(%rsi,%rax), %rdx
.LVL93:
	.p2align 4,,10
	.p2align 3
.L87:
	.loc 1 186 36 is_stmt 1 discriminator 3 view .LVU315
	.loc 1 186 40 is_stmt 0 discriminator 3 view .LVU316
	movzbl	(%rsi), %eax
	.loc 1 186 18 discriminator 3 view .LVU317
	addq	$1, %rsi
.LVL94:
	.loc 1 186 53 discriminator 3 view .LVU318
	addl	$1, 4(%r11,%rax,4)
	.loc 1 186 32 is_stmt 1 discriminator 3 view .LVU319
	.loc 1 186 18 discriminator 3 view .LVU320
	cmpq	%rsi, %rdx
	jne	.L87
.L86:
	.loc 1 186 18 is_stmt 0 discriminator 3 view .LVU321
	leaq	4(%r11), %rax
	leaq	92(%r11), %rcx
	.p2align 4,,10
	.p2align 3
.L88:
.LVL95:
	.loc 1 188 29 is_stmt 1 discriminator 3 view .LVU322
	.loc 1 188 37 is_stmt 0 discriminator 3 view .LVU323
	movl	-4(%rax), %edx
	addl	%edx, (%rax)
	.loc 1 188 25 is_stmt 1 discriminator 3 view .LVU324
.LVL96:
	.loc 1 188 18 discriminator 3 view .LVU325
	addq	$4, %rax
.LVL97:
	.loc 1 188 18 is_stmt 0 discriminator 3 view .LVU326
	cmpq	%rax, %rcx
	jne	.L88
	.loc 1 190 38 view .LVU327
	leaq	8(%r10), %rdi
	movq	%r10, %rcx
	xorl	%eax, %eax
.LVL98:
	.loc 1 190 38 view .LVU328
	movq	$0, (%r10)
	andq	$-8, %rdi
	movq	$0, 84(%r10)
	subq	%rdi, %rcx
	addl	$92, %ecx
	shrl	$3, %ecx
	rep stosq
.LVL99:
	.loc 1 193 23 is_stmt 1 view .LVU329
	cmpl	%r9d, %ebx
	jg	.L89
	movslq	%ebx, %rdx
.LVL100:
	.p2align 4,,10
	.p2align 3
.L90:
	.loc 1 194 7 discriminator 3 view .LVU330
	.loc 1 194 25 is_stmt 0 discriminator 3 view .LVU331
	addl	4(%r11,%rdx,4), %eax
.LVL101:
	.loc 1 194 11 discriminator 3 view .LVU332
	subl	(%r11,%rdx,4), %eax
.LVL102:
	.loc 1 195 7 is_stmt 1 discriminator 3 view .LVU333
	.loc 1 195 21 is_stmt 0 discriminator 3 view .LVU334
	leal	-1(%rax), %ecx
	.loc 1 196 11 discriminator 3 view .LVU335
	addl	%eax, %eax
.LVL103:
	.loc 1 195 21 discriminator 3 view .LVU336
	movl	%ecx, (%r10,%rdx,4)
	.loc 1 196 7 is_stmt 1 discriminator 3 view .LVU337
.LVL104:
	.loc 1 193 35 discriminator 3 view .LVU338
	.loc 1 193 23 discriminator 3 view .LVU339
	addq	$1, %rdx
.LVL105:
	.loc 1 193 23 is_stmt 0 discriminator 3 view .LVU340
	cmpl	%edx, %r9d
	jge	.L90
.LVL106:
.L89:
	.loc 1 198 4 is_stmt 1 view .LVU341
	.loc 1 198 11 is_stmt 0 view .LVU342
	leal	1(%rbx), %eax
.LVL107:
	.loc 1 198 27 is_stmt 1 view .LVU343
	cmpl	%eax, %r9d
	jl	.L79
	cltq
.LVL108:
	.p2align 4,,10
	.p2align 3
.L92:
	.loc 1 199 7 discriminator 3 view .LVU344
	.loc 1 199 30 is_stmt 0 discriminator 3 view .LVU345
	movl	-4(%r10,%rax,4), %edx
	.loc 1 199 35 discriminator 3 view .LVU346
	leal	2(%rdx,%rdx), %edx
	.loc 1 199 41 discriminator 3 view .LVU347
	subl	(%r11,%rax,4), %edx
	.loc 1 199 15 discriminator 3 view .LVU348
	movl	%edx, (%r11,%rax,4)
	.loc 1 198 39 is_stmt 1 discriminator 3 view .LVU349
.LVL109:
	.loc 1 198 27 discriminator 3 view .LVU350
	addq	$1, %rax
.LVL110:
	.loc 1 198 27 is_stmt 0 discriminator 3 view .LVU351
	cmpl	%eax, %r9d
	jge	.L92
.L79:
	.loc 1 200 1 view .LVU352
	popq	%rbx
	.cfi_def_cfa_offset 32
.LVL111:
	.loc 1 200 1 view .LVU353
	popq	%rbp
	.cfi_def_cfa_offset 24
.LVL112:
	.loc 1 200 1 view .LVU354
	popq	%r12
	.cfi_def_cfa_offset 16
	popq	%r13
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE55:
	.size	BZ2_hbCreateDecodeTables, .-BZ2_hbCreateDecodeTables
.Letext0:
	.file 2 "bzlib_private.h"
	.section	.debug_info,"",@progbits
.Ldebug_info0:
	.long	0x498
	.value	0x5
	.byte	0x1
	.byte	0x8
	.long	.Ldebug_abbrev0
	.uleb128 0xd
	.long	.LASF33
	.byte	0x1d
	.long	.LASF0
	.long	.LASF1
	.quad	.Ltext0
	.quad	.Letext0-.Ltext0
	.long	.Ldebug_line0
	.uleb128 0x3
	.byte	0x8
	.byte	0x7
	.long	.LASF2
	.uleb128 0xe
	.byte	0x4
	.byte	0x5
	.string	"int"
	.uleb128 0x3
	.byte	0x8
	.byte	0x5
	.long	.LASF3
	.uleb128 0x3
	.byte	0x8
	.byte	0x5
	.long	.LASF4
	.uleb128 0x3
	.byte	0x1
	.byte	0x8
	.long	.LASF5
	.uleb128 0x3
	.byte	0x2
	.byte	0x7
	.long	.LASF6
	.uleb128 0x3
	.byte	0x4
	.byte	0x7
	.long	.LASF7
	.uleb128 0x3
	.byte	0x1
	.byte	0x6
	.long	.LASF8
	.uleb128 0x3
	.byte	0x2
	.byte	0x5
	.long	.LASF9
	.uleb128 0x3
	.byte	0x1
	.byte	0x6
	.long	.LASF10
	.uleb128 0x3
	.byte	0x8
	.byte	0x7
	.long	.LASF11
	.uleb128 0x5
	.long	.LASF12
	.byte	0x2a
	.byte	0x17
	.long	0x4a
	.uleb128 0x5
	.long	.LASF13
	.byte	0x2b
	.byte	0x17
	.long	0x4a
	.uleb128 0x5
	.long	.LASF14
	.byte	0x2c
	.byte	0xd
	.long	0x35
	.uleb128 0x9
	.long	0x86
	.uleb128 0x9
	.long	0x91
	.uleb128 0xf
	.long	.LASF34
	.byte	0x2
	.byte	0x3a
	.byte	0xd
	.long	0xb8
	.uleb128 0x10
	.long	0x35
	.byte	0
	.uleb128 0x6
	.long	.LASF21
	.byte	0xaa
	.quad	.LFB55
	.quad	.LFE55-.LFB55
	.uleb128 0x1
	.byte	0x9c
	.long	0x19b
	.uleb128 0x2
	.long	.LASF15
	.byte	0xaa
	.byte	0x28
	.long	0xa1
	.long	.LLST33
	.long	.LVUS33
	.uleb128 0x2
	.long	.LASF16
	.byte	0xab
	.byte	0x28
	.long	0xa1
	.long	.LLST34
	.long	.LVUS34
	.uleb128 0x2
	.long	.LASF17
	.byte	0xac
	.byte	0x28
	.long	0xa1
	.long	.LLST35
	.long	.LVUS35
	.uleb128 0x2
	.long	.LASF18
	.byte	0xad
	.byte	0x28
	.long	0x9c
	.long	.LLST36
	.long	.LVUS36
	.uleb128 0x2
	.long	.LASF19
	.byte	0xae
	.byte	0x27
	.long	0x91
	.long	.LLST37
	.long	.LVUS37
	.uleb128 0xa
	.long	.LASF24
	.byte	0xaf
	.byte	0x27
	.long	0x91
	.uleb128 0x1
	.byte	0x59
	.uleb128 0x2
	.long	.LASF20
	.byte	0xb0
	.byte	0x27
	.long	0x91
	.long	.LLST38
	.long	.LVUS38
	.uleb128 0x1
	.string	"pp"
	.byte	0xb2
	.byte	0xa
	.long	0x91
	.long	.LLST39
	.long	.LVUS39
	.uleb128 0x1
	.string	"i"
	.byte	0xb2
	.byte	0xe
	.long	0x91
	.long	.LLST40
	.long	.LVUS40
	.uleb128 0x1
	.string	"j"
	.byte	0xb2
	.byte	0x11
	.long	0x91
	.long	.LLST41
	.long	.LVUS41
	.uleb128 0x1
	.string	"vec"
	.byte	0xb2
	.byte	0x14
	.long	0x91
	.long	.LLST42
	.long	.LVUS42
	.byte	0
	.uleb128 0x6
	.long	.LASF22
	.byte	0x98
	.quad	.LFB54
	.quad	.LFE54-.LFB54
	.uleb128 0x1
	.byte	0x9c
	.long	0x246
	.uleb128 0x2
	.long	.LASF23
	.byte	0x98
	.byte	0x21
	.long	0xa1
	.long	.LLST26
	.long	.LVUS26
	.uleb128 0x2
	.long	.LASF18
	.byte	0x99
	.byte	0x21
	.long	0x9c
	.long	.LLST27
	.long	.LVUS27
	.uleb128 0x2
	.long	.LASF19
	.byte	0x9a
	.byte	0x20
	.long	0x91
	.long	.LLST28
	.long	.LVUS28
	.uleb128 0x2
	.long	.LASF24
	.byte	0x9b
	.byte	0x20
	.long	0x91
	.long	.LLST29
	.long	.LVUS29
	.uleb128 0xa
	.long	.LASF20
	.byte	0x9c
	.byte	0x20
	.long	0x91
	.uleb128 0x1
	.byte	0x58
	.uleb128 0x1
	.string	"n"
	.byte	0x9e
	.byte	0xa
	.long	0x91
	.long	.LLST30
	.long	.LVUS30
	.uleb128 0x1
	.string	"vec"
	.byte	0x9e
	.byte	0xd
	.long	0x91
	.long	.LLST31
	.long	.LVUS31
	.uleb128 0x1
	.string	"i"
	.byte	0x9e
	.byte	0x12
	.long	0x91
	.long	.LLST32
	.long	.LVUS32
	.byte	0
	.uleb128 0x6
	.long	.LASF25
	.byte	0x3f
	.quad	.LFB53
	.quad	.LFE53-.LFB53
	.uleb128 0x1
	.byte	0x9c
	.long	0x470
	.uleb128 0x11
	.string	"len"
	.byte	0x1
	.byte	0x3f
	.byte	0x25
	.long	0x9c
	.long	.LLST0
	.long	.LVUS0
	.uleb128 0x2
	.long	.LASF26
	.byte	0x40
	.byte	0x25
	.long	0xa1
	.long	.LLST1
	.long	.LVUS1
	.uleb128 0x2
	.long	.LASF20
	.byte	0x41
	.byte	0x24
	.long	0x91
	.long	.LLST2
	.long	.LVUS2
	.uleb128 0x2
	.long	.LASF24
	.byte	0x42
	.byte	0x24
	.long	0x91
	.long	.LLST3
	.long	.LVUS3
	.uleb128 0x7
	.long	.LASF27
	.byte	0x48
	.byte	0xa
	.long	0x91
	.long	.LLST4
	.long	.LVUS4
	.uleb128 0x7
	.long	.LASF28
	.byte	0x48
	.byte	0x12
	.long	0x91
	.long	.LLST5
	.long	.LVUS5
	.uleb128 0x1
	.string	"n1"
	.byte	0x48
	.byte	0x19
	.long	0x91
	.long	.LLST6
	.long	.LVUS6
	.uleb128 0x1
	.string	"n2"
	.byte	0x48
	.byte	0x1d
	.long	0x91
	.long	.LLST7
	.long	.LVUS7
	.uleb128 0x1
	.string	"i"
	.byte	0x48
	.byte	0x21
	.long	0x91
	.long	.LLST8
	.long	.LVUS8
	.uleb128 0x1
	.string	"j"
	.byte	0x48
	.byte	0x24
	.long	0x91
	.long	.LLST9
	.long	.LVUS9
	.uleb128 0x1
	.string	"k"
	.byte	0x48
	.byte	0x27
	.long	0x91
	.long	.LLST10
	.long	.LVUS10
	.uleb128 0x7
	.long	.LASF29
	.byte	0x49
	.byte	0x9
	.long	0x7b
	.long	.LLST11
	.long	.LVUS11
	.uleb128 0x8
	.long	.LASF30
	.byte	0x4b
	.long	0x470
	.uleb128 0x3
	.byte	0x91
	.sleb128 -5248
	.uleb128 0x8
	.long	.LASF31
	.byte	0x4c
	.long	0x481
	.uleb128 0x3
	.byte	0x91
	.sleb128 -4208
	.uleb128 0x8
	.long	.LASF32
	.byte	0x4d
	.long	0x481
	.uleb128 0x3
	.byte	0x91
	.sleb128 -2144
	.uleb128 0x4
	.long	.LLRL12
	.long	0x398
	.uleb128 0x1
	.string	"zz"
	.byte	0x5f
	.byte	0x12
	.long	0x91
	.long	.LLST13
	.long	.LVUS13
	.uleb128 0x1
	.string	"tmp"
	.byte	0x5f
	.byte	0x16
	.long	0x91
	.long	.LLST14
	.long	.LVUS14
	.byte	0
	.uleb128 0x4
	.long	.LLRL15
	.long	0x3d9
	.uleb128 0x1
	.string	"zz"
	.byte	0x65
	.byte	0x40
	.long	0x91
	.long	.LLST16
	.long	.LVUS16
	.uleb128 0x1
	.string	"yy"
	.byte	0x65
	.byte	0x44
	.long	0x91
	.long	.LLST17
	.long	.LVUS17
	.uleb128 0x1
	.string	"tmp"
	.byte	0x65
	.byte	0x48
	.long	0x91
	.long	.LLST18
	.long	.LVUS18
	.byte	0
	.uleb128 0x4
	.long	.LLRL19
	.long	0x41a
	.uleb128 0x1
	.string	"zz"
	.byte	0x66
	.byte	0x40
	.long	0x91
	.long	.LLST20
	.long	.LVUS20
	.uleb128 0x1
	.string	"yy"
	.byte	0x66
	.byte	0x44
	.long	0x91
	.long	.LLST21
	.long	.LVUS21
	.uleb128 0x1
	.string	"tmp"
	.byte	0x66
	.byte	0x48
	.long	0x91
	.long	.LLST22
	.long	.LVUS22
	.byte	0
	.uleb128 0x4
	.long	.LLRL23
	.long	0x449
	.uleb128 0x1
	.string	"zz"
	.byte	0x6d
	.byte	0x12
	.long	0x91
	.long	.LLST24
	.long	.LVUS24
	.uleb128 0x1
	.string	"tmp"
	.byte	0x6d
	.byte	0x16
	.long	0x91
	.long	.LLST25
	.long	.LVUS25
	.byte	0
	.uleb128 0x12
	.quad	.LVL44
	.long	0xa6
	.long	0x462
	.uleb128 0x13
	.uleb128 0x1
	.byte	0x55
	.uleb128 0x3
	.byte	0xa
	.value	0x7d2
	.byte	0
	.uleb128 0x14
	.quad	.LVL68
	.long	0x492
	.byte	0
	.uleb128 0xb
	.long	0x91
	.long	0x481
	.uleb128 0xc
	.long	0x2e
	.value	0x103
	.byte	0
	.uleb128 0xb
	.long	0x91
	.long	0x492
	.uleb128 0xc
	.long	0x2e
	.value	0x203
	.byte	0
	.uleb128 0x15
	.long	.LASF35
	.long	.LASF35
	.byte	0
	.section	.debug_abbrev,"",@progbits
.Ldebug_abbrev0:
	.uleb128 0x1
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x2
	.uleb128 0x5
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x3
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0xe
	.byte	0
	.byte	0
	.uleb128 0x4
	.uleb128 0xb
	.byte	0x1
	.uleb128 0x55
	.uleb128 0x17
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x5
	.uleb128 0x16
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 2
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x6
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0x21
	.sleb128 6
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x7
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x7a
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x7
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x8
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0x21
	.sleb128 10
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0x9
	.uleb128 0xf
	.byte	0
	.uleb128 0xb
	.uleb128 0x21
	.sleb128 8
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0xa
	.uleb128 0x5
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0x21
	.sleb128 1
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0xb
	.uleb128 0x1
	.byte	0x1
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0xc
	.uleb128 0x21
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2f
	.uleb128 0x5
	.byte	0
	.byte	0
	.uleb128 0xd
	.uleb128 0x11
	.byte	0x1
	.uleb128 0x25
	.uleb128 0xe
	.uleb128 0x13
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x1f
	.uleb128 0x1b
	.uleb128 0x1f
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x7
	.uleb128 0x10
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0xe
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x8
	.byte	0
	.byte	0
	.uleb128 0xf
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x3c
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x10
	.uleb128 0x5
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x11
	.uleb128 0x5
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x17
	.uleb128 0x2137
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x12
	.uleb128 0x48
	.byte	0x1
	.uleb128 0x7d
	.uleb128 0x1
	.uleb128 0x7f
	.uleb128 0x13
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x13
	.uleb128 0x49
	.byte	0
	.uleb128 0x2
	.uleb128 0x18
	.uleb128 0x7e
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0x14
	.uleb128 0x48
	.byte	0
	.uleb128 0x7d
	.uleb128 0x1
	.uleb128 0x7f
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x15
	.uleb128 0x2e
	.byte	0
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3c
	.uleb128 0x19
	.uleb128 0x6e
	.uleb128 0xe
	.uleb128 0x3
	.uleb128 0xe
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_loclists,"",@progbits
	.long	.Ldebug_loc3-.Ldebug_loc2
.Ldebug_loc2:
	.value	0x5
	.byte	0x8
	.byte	0
	.long	0
.Ldebug_loc0:
.LVUS33:
	.uleb128 0
	.uleb128 .LVU297
	.uleb128 .LVU297
	.uleb128 0
.LLST33:
	.byte	0x4
	.uleb128 .LVL80-.Ltext0
	.uleb128 .LVL84-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL84-.Ltext0
	.uleb128 .LFE55-.Ltext0
	.uleb128 0x1
	.byte	0x5a
	.byte	0
.LVUS34:
	.uleb128 0
	.uleb128 .LVU292
	.uleb128 .LVU292
	.uleb128 0
.LLST34:
	.byte	0x4
	.uleb128 .LVL80-.Ltext0
	.uleb128 .LVL82-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0x4
	.uleb128 .LVL82-.Ltext0
	.uleb128 .LFE55-.Ltext0
	.uleb128 0x1
	.byte	0x5b
	.byte	0
.LVUS35:
	.uleb128 0
	.uleb128 .LVU298
	.uleb128 .LVU298
	.uleb128 .LVU312
	.uleb128 .LVU312
	.uleb128 0
.LLST35:
	.byte	0x4
	.uleb128 .LVL80-.Ltext0
	.uleb128 .LVL85-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL85-.Ltext0
	.uleb128 .LVL91-.Ltext0
	.uleb128 0x1
	.byte	0x5c
	.byte	0x4
	.uleb128 .LVL91-.Ltext0
	.uleb128 .LFE55-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x51
	.byte	0x9f
	.byte	0
.LVUS36:
	.uleb128 0
	.uleb128 .LVU295
	.uleb128 .LVU295
	.uleb128 .LVU315
	.uleb128 .LVU315
	.uleb128 0
.LLST36:
	.byte	0x4
	.uleb128 .LVL80-.Ltext0
	.uleb128 .LVL83-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL83-.Ltext0
	.uleb128 .LVL93-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0x4
	.uleb128 .LVL93-.Ltext0
	.uleb128 .LFE55-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x52
	.byte	0x9f
	.byte	0
.LVUS37:
	.uleb128 0
	.uleb128 .LVU298
	.uleb128 .LVU298
	.uleb128 .LVU353
	.uleb128 .LVU353
	.uleb128 0
.LLST37:
	.byte	0x4
	.uleb128 .LVL80-.Ltext0
	.uleb128 .LVL85-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL85-.Ltext0
	.uleb128 .LVL111-.Ltext0
	.uleb128 0x1
	.byte	0x53
	.byte	0x4
	.uleb128 .LVL111-.Ltext0
	.uleb128 .LFE55-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x58
	.byte	0x9f
	.byte	0
.LVUS38:
	.uleb128 0
	.uleb128 .LVU312
	.uleb128 .LVU312
	.uleb128 .LVU354
.LLST38:
	.byte	0x4
	.uleb128 .LVL80-.Ltext0
	.uleb128 .LVL91-.Ltext0
	.uleb128 0x2
	.byte	0x91
	.sleb128 0
	.byte	0x4
	.uleb128 .LVL91-.Ltext0
	.uleb128 .LVL112-.Ltext0
	.uleb128 0x1
	.byte	0x56
	.byte	0
.LVUS39:
	.uleb128 .LVU289
	.uleb128 .LVU298
	.uleb128 .LVU298
	.uleb128 .LVU305
	.uleb128 .LVU305
	.uleb128 .LVU307
	.uleb128 .LVU307
	.uleb128 .LVU312
.LLST39:
	.byte	0x4
	.uleb128 .LVL81-.Ltext0
	.uleb128 .LVL85-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL85-.Ltext0
	.uleb128 .LVL87-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL87-.Ltext0
	.uleb128 .LVL88-.Ltext0
	.uleb128 0x3
	.byte	0x75
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL88-.Ltext0
	.uleb128 .LVL91-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0
.LVUS40:
	.uleb128 .LVU290
	.uleb128 .LVU298
	.uleb128 .LVU298
	.uleb128 .LVU312
	.uleb128 .LVU313
	.uleb128 .LVU315
	.uleb128 .LVU315
	.uleb128 .LVU318
	.uleb128 .LVU322
	.uleb128 .LVU325
	.uleb128 .LVU325
	.uleb128 .LVU326
	.uleb128 .LVU326
	.uleb128 .LVU328
	.uleb128 .LVU329
	.uleb128 .LVU330
	.uleb128 .LVU330
	.uleb128 .LVU339
	.uleb128 .LVU339
	.uleb128 .LVU340
	.uleb128 .LVU343
	.uleb128 .LVU350
	.uleb128 .LVU350
	.uleb128 .LVU351
.LLST40:
	.byte	0x4
	.uleb128 .LVL81-.Ltext0
	.uleb128 .LVL85-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL85-.Ltext0
	.uleb128 .LVL91-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL92-.Ltext0
	.uleb128 .LVL93-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL93-.Ltext0
	.uleb128 .LVL94-.Ltext0
	.uleb128 0x7
	.byte	0x74
	.sleb128 0
	.byte	0xa3
	.uleb128 0x1
	.byte	0x52
	.byte	0x1c
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL95-.Ltext0
	.uleb128 .LVL96-.Ltext0
	.uleb128 0xc
	.byte	0x70
	.sleb128 0
	.byte	0x7b
	.sleb128 0
	.byte	0x1c
	.byte	0x34
	.byte	0x1c
	.byte	0x32
	.byte	0x25
	.byte	0x23
	.uleb128 0x1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL96-.Ltext0
	.uleb128 .LVL97-.Ltext0
	.uleb128 0xc
	.byte	0x70
	.sleb128 0
	.byte	0x7b
	.sleb128 0
	.byte	0x1c
	.byte	0x34
	.byte	0x1c
	.byte	0x32
	.byte	0x25
	.byte	0x23
	.uleb128 0x2
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL97-.Ltext0
	.uleb128 .LVL98-.Ltext0
	.uleb128 0xc
	.byte	0x70
	.sleb128 0
	.byte	0x7b
	.sleb128 0
	.byte	0x1c
	.byte	0x38
	.byte	0x1c
	.byte	0x32
	.byte	0x25
	.byte	0x23
	.uleb128 0x2
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL99-.Ltext0
	.uleb128 .LVL100-.Ltext0
	.uleb128 0x1
	.byte	0x53
	.byte	0x4
	.uleb128 .LVL100-.Ltext0
	.uleb128 .LVL104-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL104-.Ltext0
	.uleb128 .LVL105-.Ltext0
	.uleb128 0x3
	.byte	0x71
	.sleb128 1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL107-.Ltext0
	.uleb128 .LVL109-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL109-.Ltext0
	.uleb128 .LVL110-.Ltext0
	.uleb128 0x3
	.byte	0x70
	.sleb128 1
	.byte	0x9f
	.byte	0
.LVUS41:
	.uleb128 .LVU298
	.uleb128 .LVU299
	.uleb128 .LVU299
	.uleb128 .LVU308
	.uleb128 .LVU308
	.uleb128 .LVU309
.LLST41:
	.byte	0x4
	.uleb128 .LVL85-.Ltext0
	.uleb128 .LVL86-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL86-.Ltext0
	.uleb128 .LVL88-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL88-.Ltext0
	.uleb128 .LVL89-.Ltext0
	.uleb128 0x3
	.byte	0x70
	.sleb128 1
	.byte	0x9f
	.byte	0
.LVUS42:
	.uleb128 .LVU329
	.uleb128 .LVU330
	.uleb128 .LVU330
	.uleb128 .LVU332
	.uleb128 .LVU333
	.uleb128 .LVU336
	.uleb128 .LVU336
	.uleb128 .LVU338
	.uleb128 .LVU338
	.uleb128 .LVU341
.LLST42:
	.byte	0x4
	.uleb128 .LVL99-.Ltext0
	.uleb128 .LVL100-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL100-.Ltext0
	.uleb128 .LVL101-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL102-.Ltext0
	.uleb128 .LVL103-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL103-.Ltext0
	.uleb128 .LVL104-.Ltext0
	.uleb128 0x3
	.byte	0x72
	.sleb128 1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL104-.Ltext0
	.uleb128 .LVL106-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0
.LVUS26:
	.uleb128 0
	.uleb128 .LVU264
	.uleb128 .LVU264
	.uleb128 0
.LLST26:
	.byte	0x4
	.uleb128 .LVL69-.Ltext0
	.uleb128 .LVL71-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL71-.Ltext0
	.uleb128 .LFE54-.Ltext0
	.uleb128 0x1
	.byte	0x5a
	.byte	0
.LVUS27:
	.uleb128 0
	.uleb128 .LVU266
	.uleb128 .LVU266
	.uleb128 0
.LLST27:
	.byte	0x4
	.uleb128 .LVL69-.Ltext0
	.uleb128 .LVL72-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0x4
	.uleb128 .LVL72-.Ltext0
	.uleb128 .LFE54-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0
.LVUS28:
	.uleb128 0
	.uleb128 .LVU267
	.uleb128 .LVU267
	.uleb128 0
.LLST28:
	.byte	0x4
	.uleb128 .LVL69-.Ltext0
	.uleb128 .LVL73-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL73-.Ltext0
	.uleb128 .LFE54-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x51
	.byte	0x9f
	.byte	0
.LVUS29:
	.uleb128 0
	.uleb128 .LVU267
	.uleb128 .LVU267
	.uleb128 .LVU284
	.uleb128 .LVU284
	.uleb128 0
.LLST29:
	.byte	0x4
	.uleb128 .LVL69-.Ltext0
	.uleb128 .LVL73-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL73-.Ltext0
	.uleb128 .LVL79-.Ltext0
	.uleb128 0x3
	.byte	0x7b
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL79-.Ltext0
	.uleb128 .LFE54-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x52
	.byte	0x9f
	.byte	0
.LVUS30:
	.uleb128 .LVU262
	.uleb128 .LVU281
	.uleb128 .LVU281
	.uleb128 .LVU283
	.uleb128 .LVU283
	.uleb128 0
.LLST30:
	.byte	0x4
	.uleb128 .LVL70-.Ltext0
	.uleb128 .LVL77-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL77-.Ltext0
	.uleb128 .LVL78-.Ltext0
	.uleb128 0x3
	.byte	0x71
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL78-.Ltext0
	.uleb128 .LFE54-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0
.LVUS31:
	.uleb128 .LVU261
	.uleb128 .LVU267
	.uleb128 .LVU267
	.uleb128 .LVU284
.LLST31:
	.byte	0x4
	.uleb128 .LVL70-.Ltext0
	.uleb128 .LVL73-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL73-.Ltext0
	.uleb128 .LVL79-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0
.LVUS32:
	.uleb128 .LVU267
	.uleb128 .LVU268
	.uleb128 .LVU268
	.uleb128 .LVU276
	.uleb128 .LVU276
	.uleb128 .LVU277
.LLST32:
	.byte	0x4
	.uleb128 .LVL73-.Ltext0
	.uleb128 .LVL74-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL74-.Ltext0
	.uleb128 .LVL75-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL75-.Ltext0
	.uleb128 .LVL76-.Ltext0
	.uleb128 0x3
	.byte	0x70
	.sleb128 1
	.byte	0x9f
	.byte	0
.LVUS0:
	.uleb128 0
	.uleb128 .LVU3
	.uleb128 .LVU3
	.uleb128 0
.LLST0:
	.byte	0x4
	.uleb128 .LVL0-.Ltext0
	.uleb128 .LVL1-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL1-.Ltext0
	.uleb128 .LFE53-.Ltext0
	.uleb128 0x3
	.byte	0x91
	.sleb128 -5296
	.byte	0
.LVUS1:
	.uleb128 0
	.uleb128 .LVU21
	.uleb128 .LVU21
	.uleb128 0
.LLST1:
	.byte	0x4
	.uleb128 .LVL0-.Ltext0
	.uleb128 .LVL7-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0x4
	.uleb128 .LVL7-.Ltext0
	.uleb128 .LFE53-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x54
	.byte	0x9f
	.byte	0
.LVUS2:
	.uleb128 0
	.uleb128 .LVU11
	.uleb128 .LVU11
	.uleb128 .LVU19
	.uleb128 .LVU19
	.uleb128 0
.LLST2:
	.byte	0x4
	.uleb128 .LVL0-.Ltext0
	.uleb128 .LVL3-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL3-.Ltext0
	.uleb128 .LVL6-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL6-.Ltext0
	.uleb128 .LFE53-.Ltext0
	.uleb128 0x4
	.byte	0xa3
	.uleb128 0x1
	.byte	0x51
	.byte	0x9f
	.byte	0
.LVUS3:
	.uleb128 0
	.uleb128 .LVU11
	.uleb128 .LVU11
	.uleb128 0
.LLST3:
	.byte	0x4
	.uleb128 .LVL0-.Ltext0
	.uleb128 .LVL3-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL3-.Ltext0
	.uleb128 .LFE53-.Ltext0
	.uleb128 0x3
	.byte	0x91
	.sleb128 -5284
	.byte	0
.LVUS4:
	.uleb128 .LVU23
	.uleb128 .LVU69
	.uleb128 .LVU69
	.uleb128 .LVU159
	.uleb128 .LVU159
	.uleb128 .LVU209
	.uleb128 .LVU209
	.uleb128 .LVU210
	.uleb128 .LVU250
	.uleb128 .LVU251
	.uleb128 .LVU251
	.uleb128 .LVU252
	.uleb128 .LVU252
	.uleb128 .LVU255
.LLST4:
	.byte	0x4
	.uleb128 .LVL7-.Ltext0
	.uleb128 .LVL20-.Ltext0
	.uleb128 0x3
	.byte	0x91
	.sleb128 -5288
	.byte	0x4
	.uleb128 .LVL20-.Ltext0
	.uleb128 .LVL38-.Ltext0
	.uleb128 0x3
	.byte	0x76
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL38-.Ltext0
	.uleb128 .LVL43-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL43-.Ltext0
	.uleb128 .LVL44-1-.Ltext0
	.uleb128 0xb
	.byte	0x71
	.sleb128 0
	.byte	0x32
	.byte	0x24
	.byte	0x91
	.sleb128 0
	.byte	0x22
	.byte	0xa
	.value	0x1480
	.byte	0x1c
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL64-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL64-.Ltext0
	.uleb128 .LVL65-.Ltext0
	.uleb128 0x3
	.byte	0x91
	.sleb128 -5288
	.byte	0x4
	.uleb128 .LVL65-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x3
	.byte	0x76
	.sleb128 -1
	.byte	0x9f
	.byte	0
.LVUS5:
	.uleb128 .LVU24
	.uleb128 .LVU33
	.uleb128 .LVU36
	.uleb128 .LVU39
	.uleb128 .LVU39
	.uleb128 .LVU42
	.uleb128 .LVU42
	.uleb128 .LVU63
	.uleb128 .LVU63
	.uleb128 .LVU67
	.uleb128 .LVU67
	.uleb128 .LVU69
	.uleb128 .LVU69
	.uleb128 .LVU72
	.uleb128 .LVU72
	.uleb128 .LVU76
	.uleb128 .LVU76
	.uleb128 .LVU119
	.uleb128 .LVU119
	.uleb128 .LVU179
	.uleb128 .LVU179
	.uleb128 .LVU210
	.uleb128 .LVU250
	.uleb128 .LVU253
	.uleb128 .LVU253
	.uleb128 .LVU255
.LLST5:
	.byte	0x4
	.uleb128 .LVL7-.Ltext0
	.uleb128 .LVL9-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL10-.Ltext0
	.uleb128 .LVL11-.Ltext0
	.uleb128 0x3
	.byte	0x71
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL11-.Ltext0
	.uleb128 .LVL12-.Ltext0
	.uleb128 0x3
	.byte	0x78
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL12-.Ltext0
	.uleb128 .LVL17-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL17-.Ltext0
	.uleb128 .LVL19-.Ltext0
	.uleb128 0x1
	.byte	0x5a
	.byte	0x4
	.uleb128 .LVL19-.Ltext0
	.uleb128 .LVL20-.Ltext0
	.uleb128 0x3
	.byte	0x91
	.sleb128 -5288
	.byte	0x4
	.uleb128 .LVL20-.Ltext0
	.uleb128 .LVL21-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL21-.Ltext0
	.uleb128 .LVL23-.Ltext0
	.uleb128 0x3
	.byte	0x78
	.sleb128 1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL23-.Ltext0
	.uleb128 .LVL31-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL31-.Ltext0
	.uleb128 .LVL39-.Ltext0
	.uleb128 0x3
	.byte	0x78
	.sleb128 -1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL39-.Ltext0
	.uleb128 .LVL44-1-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL66-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL66-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x3
	.byte	0x78
	.sleb128 -1
	.byte	0x9f
	.byte	0
.LVUS6:
	.uleb128 .LVU73
	.uleb128 .LVU211
	.uleb128 .LVU250
	.uleb128 .LVU251
	.uleb128 .LVU252
	.uleb128 .LVU255
.LLST6:
	.byte	0x4
	.uleb128 .LVL22-.Ltext0
	.uleb128 .LVL44-.Ltext0
	.uleb128 0x1
	.byte	0x5e
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL64-.Ltext0
	.uleb128 0x1
	.byte	0x5e
	.byte	0x4
	.uleb128 .LVL65-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x1
	.byte	0x5e
	.byte	0
.LVUS7:
	.uleb128 .LVU116
	.uleb128 .LVU211
	.uleb128 .LVU250
	.uleb128 .LVU251
	.uleb128 .LVU253
	.uleb128 .LVU255
.LLST7:
	.byte	0x4
	.uleb128 .LVL30-.Ltext0
	.uleb128 .LVL44-.Ltext0
	.uleb128 0x1
	.byte	0x5d
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL64-.Ltext0
	.uleb128 0x1
	.byte	0x5d
	.byte	0x4
	.uleb128 .LVL66-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x1
	.byte	0x5d
	.byte	0
.LVUS8:
	.uleb128 .LVU10
	.uleb128 .LVU11
	.uleb128 .LVU11
	.uleb128 .LVU16
	.uleb128 .LVU16
	.uleb128 .LVU17
	.uleb128 .LVU32
	.uleb128 .LVU33
	.uleb128 .LVU36
	.uleb128 .LVU39
	.uleb128 .LVU39
	.uleb128 .LVU63
	.uleb128 .LVU63
	.uleb128 .LVU66
	.uleb128 .LVU210
	.uleb128 .LVU211
	.uleb128 .LVU213
	.uleb128 .LVU229
	.uleb128 .LVU229
	.uleb128 .LVU230
	.uleb128 .LVU233
	.uleb128 .LVU235
	.uleb128 .LVU235
	.uleb128 .LVU239
	.uleb128 .LVU239
	.uleb128 .LVU248
	.uleb128 .LVU249
	.uleb128 .LVU250
	.uleb128 .LVU251
	.uleb128 .LVU252
.LLST8:
	.byte	0x4
	.uleb128 .LVL2-.Ltext0
	.uleb128 .LVL3-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL3-.Ltext0
	.uleb128 .LVL4-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL4-.Ltext0
	.uleb128 .LVL5-.Ltext0
	.uleb128 0x3
	.byte	0x70
	.sleb128 1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL8-.Ltext0
	.uleb128 .LVL9-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL10-.Ltext0
	.uleb128 .LVL11-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL11-.Ltext0
	.uleb128 .LVL17-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL17-.Ltext0
	.uleb128 .LVL18-.Ltext0
	.uleb128 0x1
	.byte	0x5a
	.byte	0x4
	.uleb128 .LVL44-.Ltext0
	.uleb128 .LVL44-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL45-.Ltext0
	.uleb128 .LVL50-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL50-.Ltext0
	.uleb128 .LVL51-.Ltext0
	.uleb128 0x3
	.byte	0x72
	.sleb128 1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL52-.Ltext0
	.uleb128 .LVL54-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL54-.Ltext0
	.uleb128 .LVL55-.Ltext0
	.uleb128 0xd
	.byte	0x71
	.sleb128 0
	.byte	0x91
	.sleb128 0
	.byte	0x1c
	.byte	0x23
	.uleb128 0x106c
	.byte	0x32
	.byte	0x25
	.byte	0x23
	.uleb128 0x1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL55-.Ltext0
	.uleb128 .LVL61-.Ltext0
	.uleb128 0xd
	.byte	0x71
	.sleb128 0
	.byte	0x91
	.sleb128 0
	.byte	0x1c
	.byte	0x23
	.uleb128 0x1068
	.byte	0x32
	.byte	0x25
	.byte	0x23
	.uleb128 0x1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL62-.Ltext0
	.uleb128 .LVL63-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL64-.Ltext0
	.uleb128 .LVL65-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0
.LVUS9:
	.uleb128 .LVU213
	.uleb128 .LVU217
	.uleb128 .LVU217
	.uleb128 .LVU223
	.uleb128 .LVU236
	.uleb128 .LVU239
	.uleb128 .LVU239
	.uleb128 .LVU241
	.uleb128 .LVU241
	.uleb128 .LVU242
	.uleb128 .LVU242
	.uleb128 .LVU243
	.uleb128 .LVU243
	.uleb128 .LVU245
	.uleb128 .LVU245
	.uleb128 .LVU246
	.uleb128 .LVU246
	.uleb128 .LVU247
	.uleb128 .LVU249
	.uleb128 .LVU250
.LLST9:
	.byte	0x4
	.uleb128 .LVL45-.Ltext0
	.uleb128 .LVL46-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL46-.Ltext0
	.uleb128 .LVL49-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL54-.Ltext0
	.uleb128 .LVL55-.Ltext0
	.uleb128 0x7
	.byte	0x71
	.sleb128 0
	.byte	0x94
	.byte	0x4
	.byte	0x38
	.byte	0x26
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL55-.Ltext0
	.uleb128 .LVL56-.Ltext0
	.uleb128 0x5
	.byte	0x70
	.sleb128 0
	.byte	0x38
	.byte	0x26
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL56-.Ltext0
	.uleb128 .LVL57-.Ltext0
	.uleb128 0x5
	.byte	0x72
	.sleb128 0
	.byte	0x38
	.byte	0x26
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL57-.Ltext0
	.uleb128 .LVL58-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL58-.Ltext0
	.uleb128 .LVL59-.Ltext0
	.uleb128 0x3
	.byte	0x70
	.sleb128 1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL59-.Ltext0
	.uleb128 .LVL60-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL60-.Ltext0
	.uleb128 .LVL61-.Ltext0
	.uleb128 0xe
	.byte	0x71
	.sleb128 -4
	.byte	0x94
	.byte	0x4
	.byte	0x4f
	.byte	0x25
	.byte	0x72
	.sleb128 0
	.byte	0x22
	.byte	0x31
	.byte	0x26
	.byte	0x23
	.uleb128 0x1
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL62-.Ltext0
	.uleb128 .LVL63-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0
.LVUS10:
	.uleb128 .LVU213
	.uleb128 .LVU217
	.uleb128 .LVU218
	.uleb128 .LVU220
	.uleb128 .LVU249
	.uleb128 .LVU250
.LLST10:
	.byte	0x4
	.uleb128 .LVL45-.Ltext0
	.uleb128 .LVL46-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL46-.Ltext0
	.uleb128 .LVL47-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL62-.Ltext0
	.uleb128 .LVL63-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0
.LVUS11:
	.uleb128 .LVU210
	.uleb128 .LVU211
	.uleb128 .LVU213
	.uleb128 .LVU234
	.uleb128 .LVU249
	.uleb128 .LVU250
.LLST11:
	.byte	0x4
	.uleb128 .LVL44-.Ltext0
	.uleb128 .LVL44-.Ltext0
	.uleb128 0x2
	.byte	0x30
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL45-.Ltext0
	.uleb128 .LVL53-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0x4
	.uleb128 .LVL62-.Ltext0
	.uleb128 .LVL63-.Ltext0
	.uleb128 0x1
	.byte	0x54
	.byte	0
.LVUS13:
	.uleb128 .LVU45
	.uleb128 .LVU51
	.uleb128 .LVU51
	.uleb128 .LVU52
	.uleb128 .LVU52
	.uleb128 .LVU56
	.uleb128 .LVU56
	.uleb128 .LVU60
	.uleb128 .LVU251
	.uleb128 .LVU252
.LLST13:
	.byte	0x4
	.uleb128 .LVL12-.Ltext0
	.uleb128 .LVL13-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL13-.Ltext0
	.uleb128 .LVL14-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL14-.Ltext0
	.uleb128 .LVL15-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL15-.Ltext0
	.uleb128 .LVL16-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL64-.Ltext0
	.uleb128 .LVL65-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0
.LVUS14:
	.uleb128 .LVU46
	.uleb128 .LVU63
	.uleb128 .LVU63
	.uleb128 .LVU69
	.uleb128 .LVU251
	.uleb128 .LVU252
.LLST14:
	.byte	0x4
	.uleb128 .LVL12-.Ltext0
	.uleb128 .LVL17-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL17-.Ltext0
	.uleb128 .LVL20-.Ltext0
	.uleb128 0x1
	.byte	0x5a
	.byte	0x4
	.uleb128 .LVL64-.Ltext0
	.uleb128 .LVL65-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0
.LVUS16:
	.uleb128 .LVU78
	.uleb128 .LVU87
	.uleb128 .LVU87
	.uleb128 .LVU94
	.uleb128 .LVU94
	.uleb128 .LVU99
	.uleb128 .LVU99
	.uleb128 .LVU110
	.uleb128 .LVU252
	.uleb128 .LVU253
.LLST16:
	.byte	0x4
	.uleb128 .LVL23-.Ltext0
	.uleb128 .LVL24-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL24-.Ltext0
	.uleb128 .LVL26-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL26-.Ltext0
	.uleb128 .LVL27-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL27-.Ltext0
	.uleb128 .LVL29-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL65-.Ltext0
	.uleb128 .LVL66-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0
.LVUS17:
	.uleb128 .LVU81
	.uleb128 .LVU87
	.uleb128 .LVU87
	.uleb128 .LVU88
	.uleb128 .LVU88
	.uleb128 .LVU97
	.uleb128 .LVU97
	.uleb128 .LVU108
	.uleb128 .LVU108
	.uleb128 .LVU110
	.uleb128 .LVU252
	.uleb128 .LVU253
.LLST17:
	.byte	0x4
	.uleb128 .LVL23-.Ltext0
	.uleb128 .LVL24-.Ltext0
	.uleb128 0x2
	.byte	0x32
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL24-.Ltext0
	.uleb128 .LVL25-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL25-.Ltext0
	.uleb128 .LVL26-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL26-.Ltext0
	.uleb128 .LVL28-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL28-.Ltext0
	.uleb128 .LVL29-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL65-.Ltext0
	.uleb128 .LVL66-.Ltext0
	.uleb128 0x2
	.byte	0x32
	.byte	0x9f
	.byte	0
.LVUS18:
	.uleb128 .LVU79
	.uleb128 .LVU116
	.uleb128 .LVU116
	.uleb128 .LVU118
	.uleb128 .LVU252
	.uleb128 .LVU253
.LLST18:
	.byte	0x4
	.uleb128 .LVL23-.Ltext0
	.uleb128 .LVL30-.Ltext0
	.uleb128 0x1
	.byte	0x5d
	.byte	0x4
	.uleb128 .LVL30-.Ltext0
	.uleb128 .LVL31-.Ltext0
	.uleb128 0xb
	.byte	0x72
	.sleb128 0
	.byte	0x32
	.byte	0x24
	.byte	0x91
	.sleb128 0
	.byte	0x22
	.byte	0xa
	.value	0x1480
	.byte	0x1c
	.byte	0x4
	.uleb128 .LVL65-.Ltext0
	.uleb128 .LVL66-.Ltext0
	.uleb128 0x1
	.byte	0x5d
	.byte	0
.LVUS20:
	.uleb128 .LVU121
	.uleb128 .LVU130
	.uleb128 .LVU130
	.uleb128 .LVU137
	.uleb128 .LVU137
	.uleb128 .LVU142
	.uleb128 .LVU142
	.uleb128 .LVU153
	.uleb128 .LVU253
	.uleb128 .LVU255
.LLST20:
	.byte	0x4
	.uleb128 .LVL31-.Ltext0
	.uleb128 .LVL32-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL32-.Ltext0
	.uleb128 .LVL34-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL34-.Ltext0
	.uleb128 .LVL35-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL35-.Ltext0
	.uleb128 .LVL37-.Ltext0
	.uleb128 0x1
	.byte	0x52
	.byte	0x4
	.uleb128 .LVL66-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x2
	.byte	0x31
	.byte	0x9f
	.byte	0
.LVUS21:
	.uleb128 .LVU124
	.uleb128 .LVU130
	.uleb128 .LVU130
	.uleb128 .LVU131
	.uleb128 .LVU131
	.uleb128 .LVU140
	.uleb128 .LVU140
	.uleb128 .LVU151
	.uleb128 .LVU151
	.uleb128 .LVU153
	.uleb128 .LVU253
	.uleb128 .LVU255
.LLST21:
	.byte	0x4
	.uleb128 .LVL31-.Ltext0
	.uleb128 .LVL32-.Ltext0
	.uleb128 0x2
	.byte	0x32
	.byte	0x9f
	.byte	0x4
	.uleb128 .LVL32-.Ltext0
	.uleb128 .LVL33-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL33-.Ltext0
	.uleb128 .LVL34-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL34-.Ltext0
	.uleb128 .LVL36-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL36-.Ltext0
	.uleb128 .LVL37-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL66-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x2
	.byte	0x32
	.byte	0x9f
	.byte	0
.LVUS22:
	.uleb128 .LVU122
	.uleb128 .LVU211
	.uleb128 .LVU250
	.uleb128 .LVU251
	.uleb128 .LVU253
	.uleb128 .LVU255
.LLST22:
	.byte	0x4
	.uleb128 .LVL31-.Ltext0
	.uleb128 .LVL44-.Ltext0
	.uleb128 0x1
	.byte	0x5f
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL64-.Ltext0
	.uleb128 0x1
	.byte	0x5f
	.byte	0x4
	.uleb128 .LVL66-.Ltext0
	.uleb128 .LVL67-.Ltext0
	.uleb128 0x1
	.byte	0x5f
	.byte	0
.LVUS24:
	.uleb128 .LVU182
	.uleb128 .LVU189
	.uleb128 .LVU189
	.uleb128 .LVU190
	.uleb128 .LVU190
	.uleb128 .LVU194
	.uleb128 .LVU194
	.uleb128 .LVU210
	.uleb128 .LVU250
	.uleb128 .LVU251
.LLST24:
	.byte	0x4
	.uleb128 .LVL39-.Ltext0
	.uleb128 .LVL40-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0x4
	.uleb128 .LVL40-.Ltext0
	.uleb128 .LVL41-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL41-.Ltext0
	.uleb128 .LVL42-.Ltext0
	.uleb128 0x1
	.byte	0x50
	.byte	0x4
	.uleb128 .LVL42-.Ltext0
	.uleb128 .LVL44-1-.Ltext0
	.uleb128 0x1
	.byte	0x51
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL64-.Ltext0
	.uleb128 0x1
	.byte	0x58
	.byte	0
.LVUS25:
	.uleb128 .LVU183
	.uleb128 .LVU209
	.uleb128 .LVU209
	.uleb128 .LVU210
	.uleb128 .LVU250
	.uleb128 .LVU251
.LLST25:
	.byte	0x4
	.uleb128 .LVL39-.Ltext0
	.uleb128 .LVL43-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0x4
	.uleb128 .LVL43-.Ltext0
	.uleb128 .LVL44-1-.Ltext0
	.uleb128 0xb
	.byte	0x71
	.sleb128 0
	.byte	0x32
	.byte	0x24
	.byte	0x91
	.sleb128 0
	.byte	0x22
	.byte	0xa
	.value	0x1480
	.byte	0x1c
	.byte	0x4
	.uleb128 .LVL63-.Ltext0
	.uleb128 .LVL64-.Ltext0
	.uleb128 0x1
	.byte	0x55
	.byte	0
.Ldebug_loc3:
	.section	.debug_aranges,"",@progbits
	.long	0x2c
	.value	0x2
	.long	.Ldebug_info0
	.byte	0x8
	.byte	0
	.value	0
	.value	0
	.quad	.Ltext0
	.quad	.Letext0-.Ltext0
	.quad	0
	.quad	0
	.section	.debug_rnglists,"",@progbits
.Ldebug_ranges0:
	.long	.Ldebug_ranges3-.Ldebug_ranges2
.Ldebug_ranges2:
	.value	0x5
	.byte	0x8
	.byte	0
	.long	0
.LLRL12:
	.byte	0x4
	.uleb128 .LBB2-.Ltext0
	.uleb128 .LBE2-.Ltext0
	.byte	0x4
	.uleb128 .LBB3-.Ltext0
	.uleb128 .LBE3-.Ltext0
	.byte	0x4
	.uleb128 .LBB4-.Ltext0
	.uleb128 .LBE4-.Ltext0
	.byte	0x4
	.uleb128 .LBB5-.Ltext0
	.uleb128 .LBE5-.Ltext0
	.byte	0x4
	.uleb128 .LBB6-.Ltext0
	.uleb128 .LBE6-.Ltext0
	.byte	0x4
	.uleb128 .LBB7-.Ltext0
	.uleb128 .LBE7-.Ltext0
	.byte	0
.LLRL15:
	.byte	0x4
	.uleb128 .LBB8-.Ltext0
	.uleb128 .LBE8-.Ltext0
	.byte	0x4
	.uleb128 .LBB16-.Ltext0
	.uleb128 .LBE16-.Ltext0
	.byte	0
.LLRL19:
	.byte	0x4
	.uleb128 .LBB9-.Ltext0
	.uleb128 .LBE9-.Ltext0
	.byte	0x4
	.uleb128 .LBB10-.Ltext0
	.uleb128 .LBE10-.Ltext0
	.byte	0x4
	.uleb128 .LBB17-.Ltext0
	.uleb128 .LBE17-.Ltext0
	.byte	0
.LLRL23:
	.byte	0x4
	.uleb128 .LBB11-.Ltext0
	.uleb128 .LBE11-.Ltext0
	.byte	0x4
	.uleb128 .LBB12-.Ltext0
	.uleb128 .LBE12-.Ltext0
	.byte	0x4
	.uleb128 .LBB13-.Ltext0
	.uleb128 .LBE13-.Ltext0
	.byte	0x4
	.uleb128 .LBB14-.Ltext0
	.uleb128 .LBE14-.Ltext0
	.byte	0x4
	.uleb128 .LBB15-.Ltext0
	.uleb128 .LBE15-.Ltext0
	.byte	0
.Ldebug_ranges3:
	.section	.debug_line,"",@progbits
.Ldebug_line0:
	.section	.debug_str,"MS",@progbits,1
.LASF33:
	.string	"GNU C17 11.4.0 -mtune=generic -march=x86-64 -g -O2 -fasynchronous-unwind-tables -fstack-protector-strong -fstack-clash-protection -fcf-protection"
.LASF19:
	.string	"minLen"
.LASF31:
	.string	"weight"
.LASF21:
	.string	"BZ2_hbCreateDecodeTables"
.LASF24:
	.string	"maxLen"
.LASF17:
	.string	"perm"
.LASF25:
	.string	"BZ2_hbMakeCodeLengths"
.LASF5:
	.string	"unsigned char"
.LASF12:
	.string	"Bool"
.LASF16:
	.string	"base"
.LASF32:
	.string	"parent"
.LASF6:
	.string	"short unsigned int"
.LASF27:
	.string	"nNodes"
.LASF28:
	.string	"nHeap"
.LASF35:
	.string	"__stack_chk_fail"
.LASF2:
	.string	"long unsigned int"
.LASF22:
	.string	"BZ2_hbAssignCodes"
.LASF9:
	.string	"short int"
.LASF23:
	.string	"code"
.LASF20:
	.string	"alphaSize"
.LASF14:
	.string	"Int32"
.LASF7:
	.string	"unsigned int"
.LASF11:
	.string	"long long unsigned int"
.LASF30:
	.string	"heap"
.LASF34:
	.string	"BZ2_bz__AssertH__fail"
.LASF15:
	.string	"limit"
.LASF13:
	.string	"UChar"
.LASF4:
	.string	"long long int"
.LASF10:
	.string	"char"
.LASF26:
	.string	"freq"
.LASF3:
	.string	"long int"
.LASF8:
	.string	"signed char"
.LASF18:
	.string	"length"
.LASF29:
	.string	"tooLong"
	.section	.debug_line_str,"MS",@progbits,1
.LASF0:
	.string	"huffman.c"
.LASF1:
	.string	"/root/rustmap/bzip2-real-test"
	.ident	"GCC: (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	1f - 0f
	.long	4f - 1f
	.long	5
0:
	.string	"GNU"
1:
	.align 8
	.long	0xc0000002
	.long	3f - 2f
2:
	.long	0x3
3:
	.align 8
4:
