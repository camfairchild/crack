# Crack
This tool helps you crack a mnemonic.  
Obviously mnemonics have *many* combinations so this is only helpful as a tool for recovery. 
If you have written down your mnemonic slightly wrong, or missing a word, this tool can assist.

This tool comes with no warranties. By using it you accept the license.

# Instructions
## Install deps
```
pip install -r requirements/dev.txt
pip install -r requirements/prod.txt
```
## Build
```
maturin develop
```

## Edit main.py
Edit the mnemonic string into the `first` variable and modify the `START` variable for batch coordination.
Possibly modify `MAX_WORKERS` and `BATCH_SIZE` for CPU saturation.

## Run
```
python3 main.py
```

