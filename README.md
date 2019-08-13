Q: Is it possible for a secret contract to call a smart contract function that will mutate the smart contract's state?

1. user calls secret contract's `toogle_bool`
2. secret contract calls smart contract's `toggleBool`
3. smart contract should update it's state

See test