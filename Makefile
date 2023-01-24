linux:
	@cargo build
	@rm -rf "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/libBootstrap.so"
	@mv target/debug/libBootstrap.so "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/"

windows:
	@cargo xwin build --target x86_64-pc-windows-msvc
	@rm -rf "/home/sarah/.steam/steam/steamapps/common/VRChat/Ferrex/Bootstrap.dll"
	@mv target/x86_64-pc-windows-msvc/debug/Bootstrap.dll "/home/sarah/.steam/steam/steamapps/common/VRChat/Ferrex/"