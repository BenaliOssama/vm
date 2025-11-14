run:
	RUSTFLAGS="-Awarnings" cargo run playground/players_src/live.cor
#	cargo run playground/players_src/crab.cor
#	cargo run playground/players_src/pierino_add.cor
zjmp:
	RUSTFLAGS="-Awarnings" cargo run playground/players_src/pierino_test.cor
ldi:
	RUSTFLAGS="-Awarnings" cargo run playground/players_src/pierino_ldi_dir_dir.cor
python: 
	 python3 ./playground/reader.py ./playground/players_src/crab.cor
#	 python3 ./playground/reader.py ./playground/players_src/crab.cor
#	 python3 ./playground/reader.py ./playground/players_src/pierino_add.cor

