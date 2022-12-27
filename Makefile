build:
	@cargo build
	@rm -rf "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/libBootstrap.so"
	@mv target/debug/libBootstrap.so "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/"
	