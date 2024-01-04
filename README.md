# machos

Machos is a very simple tool for dumping sections of MachO files.
With Machos, you can choose specific sections to be dumped into an output file.
This tool was developed to fulfill the need for a method to extract shellcodes written in Rust for macOS.

## Usage

```text
Usage: machos [COMMAND]

Commands:
  list  List MachO file sections
  dump  Dump selected MachO file sections
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Listing sections

```text
$ machos list -i shellcode
Avalible sections:
__text
__const
```

### Dumping sections

For the following file:

```text
$ objdump -D shellcode

shellcode:      file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100003fbc <__start>:
100003fbc: 10000141     adr     x1, #40
100003fc0: d503201f     nop
100003fc4: 52800090     mov     w16, #4
100003fc8: 52800020     mov     w0, #1
100003fcc: 52800342     mov     w2, #26
100003fd0: d4001001     svc     #0x80
100003fd4: 52800030     mov     w16, #1
100003fd8: d2800000     mov     x0, #0
100003fdc: d4001001     svc     #0x80
100003fe0: d65f03c0     ret

Disassembly of section __TEXT,__const:

0000000100003fe4 <l_anon.199b25154eef2430d3d8813648c6db47.0>:
100003fe4: 6c6c6548     ldnp    d8, d25, [x10, #-320]
100003fe8: 7266206f     <unknown>
100003fec: 73206d6f     <unknown>
100003ff0: 6c6c6568     ldnp    d8, d25, [x11, #-320]
100003ff4: 65646f63     fnmls   z3.h, p3/m, z27.h, z4.h
100003ff8: a69ff020     <unknown>
100003ffc: 86 0a        <unknown>
```

The following dump was created:

```text
$ machos dump -i shellcode -s __text,__const -o shellcode.bin
$ xxd shellcode.bin
00000000: 4101 0010 1f20 03d5 9000 8052 2000 8052  A.... .....R ..R
00000010: 4203 8052 0110 00d4 3000 8052 0000 80d2  B..R....0..R....
00000020: 0110 00d4 c003 5fd6 4865 6c6c 6f20 6672  ......_.Hello fr
00000030: 6f6d 2073 6865 6c6c 636f 6465 20f0 9fa6  om shellcode ...
00000040: 860a                                     ..
```
