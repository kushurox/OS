INPUT(
    os/boot.o
    os/libordinal_system.a
)

MEMORY {
    os : ORIGIN = 0x7c00, LENGTH = 2M
}

SECTIONS {
    . = 0x7c00;

    .text :
    {   
        os/boot.o(.bootsector)
        . = 0x7c00+510;
        SHORT(0xaa55)
        . = 0x7e00;

    } > os

}
