# accounts-engine
An accounting engine programming test

by Andrew Buck, 3/24/2022

## Design
The accounts engine streams transactions from a CSV file as specified by the challenge to avoid memory exhaustion or high usage. The transactions are then processed by an account engine class, which collects any serialization or execution errors and displays them to stderr upon completion before the output CSV.

## Tests
Tests for both deserialization and processing were done. Coverage is not massive, but the logic is rather simple and in a production environment tests for serialization and more invalid inputs could be done.

## Assumptions
A few assumptions were made. 
- Disputes can be done to both deposits and withdrawals. This was not explicitly stated, and only the deposit-dispute instructions were given
- Nothing can be done with a transaction after a dispute is resolved with either a direct resolution or chargeback
- "Chargeback" refers to a user charging back a deposit to an account from a third-party processor, not a purchase made from funds on the platform
- Errors in deserialization are not fatal
- Errors in processing including insufficient funds that are not provider faults should be presented in stderr
