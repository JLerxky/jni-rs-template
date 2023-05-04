java_run: lib
	javac Api.java && java -Djava.library.path=target/release/ Api

lib:
	cargo build -r
