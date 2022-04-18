# LocalTerra xLOCAL Staking

This staking contract allows LOCAL holders to stake their tokens in exchange for xLOCAL. The amount of LOCAL they can claim later increases as accrued fees in the Maker contract get swapped to LOCAL which is then sent to stakers.

---

## InstantiateMsg

Initializes the contract with the token code ID used by LOCAL and the LOCAL token address.

```json
{
  "token_code_id": 123,
  "deposit_token_addr": "terra..."
}
```

## ExecuteMsg

### `receive`

CW20 receive msg.

```json
{
  "receive": {
    "sender": "terra...",
    "amount": "123",
    "msg": "<base64_encoded_json_string>"
  }
}
```

#### `Enter`

Deposits LOCAL in the xLOCAL staking contract.

Execute this message by calling the LOCAL token contract and use a message like this:

```json
{
  "send": {
    "contract": <StakingContractAddress>,
    "amount": "999",
    "msg": "base64-encodedStringOfWithdrawMsg"
  }
}
```

In `send.msg`, you may encode this JSON string into base64 encoding:

```json
{
  "enter": {}
}
```

#### `leave`

Burns xLOCAL and unstakes underlying LOCAL (initial staked amount + accrued LOCAL since staking).

Execute this message by calling the xLOCAL token contract and use a message like this:

```json
{
  "send": {
    "contract": <StakingContractAddress>,
    "amount": "999",
    "msg": "base64-encodedStringOfWithdrawMsg"
  }
}
```

In `send.msg` you may encode this JSON string into base64 encoding:

```json
{
  "leave": {}
}
```

## QueryMsg

All query messages are described below. A custom struct is defined for each query response.

### `config`

Returns the LOCAL and xLOCAL addresses.

```json
{
  "config": {}
}
```

### `get_total_shares`

Returns the total amount of xLOCAL tokens.

```json
{
  "get_total_shares": {}
}
```

### `get_total_deposit`

Returns the total amount of LOCAL deposits in the staking contract.

```json
{
  "get_total_deposit": {}
}
```
