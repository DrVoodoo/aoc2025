# set $env:AOC_SESSION="your_session_cookie_here"
# Usage: .\fetch_input.ps1 1
param (
    [int]$Day
)

$Year = 2025
$Session = $env:AOC_SESSION
$OutputFile = ("day{0:D2}\input.txt" -f $Day)

Invoke-WebRequest -Uri "https://adventofcode.com/$Year/day/$Day/input" -Headers @{ "Cookie" = "session=$Session" } -OutFile $OutputFile

Write-Host "Downloaded input for day $Day → $OutputFile"
