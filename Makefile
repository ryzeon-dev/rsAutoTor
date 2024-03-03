make:
	cargo build -r 
	mkdir -p ./bin
	rm -rf ./bin/rsAutoTor
	mv ./target/release/rsAutoTor ./bin 
	rm -rf ./target 

install: 
	sudo cp ./bin/rsAutoTor /usr/local/bin

clean:
	rm -rf ./bin 
