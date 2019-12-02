NAME:=hadlock
INSTALL_PATH:=/usr/local/bin/
CONFIG:=./config/hadlok.json
CONFIG_PATH:=~/.config/hadlock/

build:
	cargo build --release

install: ./target/release/$(NAME)
	install -m 644 ./target/release/$(NAME) $(INSTALL_PATH)
	install -d $(CONFIG_PATH)
	install -m $(CONFIG) $(CONFIG_PATH)

clean:
	rm $(INSTALL_PATH)$(NAME)
