cargo build --release
REM if not exist "sc2pathlib" mkdir sc2pathlib
copy "target\release\sc2pathlib.dll" "sc2pathlibp\sc2pathlib.pyd"
pause