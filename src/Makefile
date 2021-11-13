all: tabs.c
	mkdir -p ../bin

	gcc -g -Os -static -nostdlib -nostdinc -fno-pie -no-pie -mno-red-zone \
		-fno-omit-frame-pointer -pg -mnop-mcount \
		-o ../bin/tabs.com.dbg tabs.c -fuse-ld=bfd -Wl,-T,cosmopolitan/ape.lds \
		-include cosmopolitan/cosmopolitan.h cosmopolitan/crt.o cosmopolitan/ape-no-modify-self.o cosmopolitan/cosmopolitan.a

	objcopy -S -O binary ../bin/tabs.com.dbg ../bin/tabs.com
