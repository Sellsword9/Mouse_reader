run: 
	cargo build
	cp ./target/debug/raton_battery_detector ./
	sudo ./raton_battery_detector
