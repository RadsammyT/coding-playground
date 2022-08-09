mode COM3 BAUD=9600 PARITY=n DATA=8
@echo off
for /F "usebackq tokens=1,2 delims==" %%i in (`wmic os get LocalDateTime /VALUE 2^>NUL`) do if '.%%i.'=='.LocalDateTime.' set ldt=%%j
set hours=%ldt:~8,2%
set minutes=%ldt:~10,2%
set /A seconds=%ldt:~12,2%+2
set miliSeconds=%ldt:~15,3%
:loop
if %TIME% LSS %hours%:%minutes%:%seconds%.00 goto loop
for /F "usebackq tokens=1,2 delims==" %%i in (`wmic os get LocalDateTime /VALUE 2^>NUL`) do if '.%%i.'=='.LocalDateTime.' set ldt=%%j
echo S%ldt:~12,2%, >COM3
echo D%ldt:~10,2%, >COM3
echo H%ldt:~8,2%, >COM3

echo S%ldt:~12,2%, 
echo D%ldt:~10,2%,
echo H%ldt:~8,2%,
set /A seconds=%ldt:~12,2%+4
:loop2
if %TIME% LSS %hours%:%minutes%:%seconds%.00 goto loop2

echo T%ldt:~6,2%, >COM3
echo M%ldt:~4,2%, >COM3
echo J%ldt:~0,4%, >COM3

echo T%ldt:~6,2%,
echo M%ldt:~4,2%,
echo J%ldt:~0,4%,