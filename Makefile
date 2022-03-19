main: src/main.c src/program.s
	gcc $^ -o build/$@
