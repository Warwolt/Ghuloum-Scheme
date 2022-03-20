driver: src/driver.c build/program.s
	@mkdir build -p
	@gcc $^ -o build/$@
