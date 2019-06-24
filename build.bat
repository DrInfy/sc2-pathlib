cargo build --release
if not exist "sc2pathlib" mkdir sc2pathlib
copy "target\release\sc2pathlib.dll" "sc2pathlib.pyd"