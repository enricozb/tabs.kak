all: tabs.c
	gcc -g -Os -static -nostdlib -nostdinc -fno-pie -no-pie -mno-red-zone \
		-fno-omit-frame-pointer -pg -mnop-mcount \
		-o tabs.com.dbg tabs.c -fuse-ld=bfd -Wl,-T,cosmopolitan/ape.lds \
		-include cosmopolitan/cosmopolitan.h cosmopolitan/crt.o cosmopolitan/ape.o cosmopolitan/cosmopolitan.a

	objcopy -S -O binary tabs.com.dbg tabs.com
	cp tabs.com ../dist/
