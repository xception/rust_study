# Minimal assembly example (Linux x86-64)

Below is a tiny program that exits with status `0`. Each line is one instruction or directive.

```asm
section .text
global _start
_start:
    mov eax, 60       ; syscall number: exit (Linux x86-64)
    xor edi, edi      ; exit status = 0
    syscall
```

Assemble and link (example):

```bash
nasm -f elf64 exit0.asm -o exit0.o && ld exit0.o -o exit0
```

Run `./exit0`; `echo $?` should print `0`.

---

## Registers in this example (`eax`, `edi`, …)

On **x86-64**, `eax` and `edi` are the **low 32 bits** of two **64-bit** registers. Writing to `eax` or `edi` **zero-extends** into the full 64-bit register (`rax` / `rdi`). That is why `mov eax, 60` and `xor edi, edi` correctly set up the 64-bit syscall convention.

### What `eax` and `edi` are

| Name | Part of | Typical role |
|------|---------|----------------|
| `eax` | `rax` (low 32 bits) | Historically the **accumulator**; here it supplies the **syscall number** in `rax` (`mov eax, 60` → `rax = 60`). |
| `edi` | `rdi` (low 32 bits) | Historically **destination index** for string/memory ops; on **Linux x86-64 syscalls**, `rdi` is the **first syscall argument** (here: exit status). |

So in the program above: **`rax = 60`** (exit), **`rdi = 0`** (exit code 0).

### General-purpose registers (64-bit): the core eight

| 64-bit | Low 32 | Low 16 | Low 8 high 8 | Notes |
|--------|--------|--------|----------------|--------|
| `rax` | `eax` | `ax` | `al`, `ah`* | Accumulator; syscall number / return values |
| `rbx` | `ebx` | `bx` | `bl`, `bh`* | Often callee-saved in calling conventions |
| `rcx` | `ecx` | `cx` | `cl`, `ch`* | Counter; 4th integer argument (SysV AMD64 ABI) |
| `rdx` | `edx` | `dx` | `dl`, `dh`* | 3rd integer argument |
| `rsi` | `esi` | `si` | `sil`† | Source index; 2nd integer argument |
| `rdi` | `edi` | `di` | `dil`† | Destination index; 1st integer argument |
| `rbp` | `ebp` | `bp` | `bpl`† | Frame pointer (optional) |
| `rsp` | `esp` | `sp` | `spl`† | Stack pointer |

\* `ah` / `bh` / `ch` / `dh` are the legacy high bytes of `ax`–`dx` only.  
† In 64-bit mode, the low 8 bits of `rsi`, `rdi`, etc. use names like `sil`, `dil`, …

**Extra registers (64-bit mode):** `r8`–`r15`, each with `rNd` (low 32), `rNw` (low 16), `rNb` (low 8), e.g. `r8`, `r8d`, `r8w`, `r8b`.

### Registers that are not general-purpose like `rax`

- **`rip`** — instruction pointer.
- **`rflags` / `eflags`** — flags (zero, carry, sign, …).
- **Segment registers** (`cs`, `ds`, `es`, `ss`, and often `fs` / `gs` on 64-bit) — mostly special or fixed in flat 64-bit user code.

### Why this syscall uses `eax` / `edi`

Linux x86-64 `syscall` uses **`rax`** = syscall number, and **`rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`** for arguments (similar order to function arguments). **`exit`** is syscall **60**; its first argument is the status → **`rax = 60`**, **`rdi = 0`**.

### 8086-era names (optional contrast)

16-bit programming uses **`ax`, `bx`, `cx`, `dx`, `si`, `di`, `bp`, `sp`** plus **segment registers** (`cs`, `ds`, `es`, `ss`, …). The same family grew into the 32-bit `eax`… names and then 64-bit `rax`… and `r8`–`r15`.
