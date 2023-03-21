linux:
	@cargo build
	@rm -rf "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/libBootstrap.so"
	@mv target/debug/libBootstrap.so "/home/sarah/.steam/steam/steamapps/common/The Stanley Parable Ultra Deluxe/Ferrex/"

windows:
	@cargo build --target x86_64-pc-windows-msvc
	@rm -rf "C:\\Users\\sarah\\scoop\\apps\\steam\\nightly-20230211\\steamapps\\common\\ChilloutVR\\Ferrex\\Bootstrap.dll"
	@mv target/x86_64-pc-windows-msvc/debug/Bootstrap.dll "C:\\Users\\sarah\\scoop\\apps\\steam\\nightly-20230211\\steamapps\\common\\ChilloutVR\\Ferrex\\"