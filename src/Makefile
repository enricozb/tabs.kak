all: tabs.c
	gcc -g -Os -static -nostdlib -nostdinc -fno-pie -no-pie -mno-red-zone \
		-fno-omit-frame-pointer -pg -mnop-mcount \
		-o ../dist/tabs.com.dbg tabs.c -fuse-ld=bfd -Wl,-T,cosmopolitan/ape.lds \
		-include cosmopolitan/cosmopolitan.h cosmopolitan/crt.o cosmopolitan/ape.o cosmopolitan/cosmopolitan.a

	objcopy -S -O binary ../dist/tabs.com.dbg ../dist/tabs.com
	cp ../dist/tabs.com ../bin/tabs.com
