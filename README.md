# Promochecker

This tool is not meant to be used in production and is far from being finished.

The goal is to have a functional windows tools that will help manage sales periods and peremption dates.

Nothing is actually compiling, this a draft.

# Add the automatic launch on windows 
schtasks /create /tn "PromoCheck" /tr "C:\PromoChecker\checker.exe" /sc daily /st 09:00