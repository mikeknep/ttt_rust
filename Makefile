all: test

test:
	rustc --test main.rs
	./main
	make clean

play:
	rustc main.rs
	./main
	make clean

clean:
	rm main
