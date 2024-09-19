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

