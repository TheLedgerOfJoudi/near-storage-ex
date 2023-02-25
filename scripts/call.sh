!#/bin/bash

near call $CONTRACT set_info '{"token_id": "first_token", "account_id" : "alhadi.testnet"}' --accountId "alhadi.testnet"
near view $CONTRACT get_owner '{"token_id": "first_token"}' 
near view $CONTRACT get_token '{"owner_id": "alhadi.testnet"}' 