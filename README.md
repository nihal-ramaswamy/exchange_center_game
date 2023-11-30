# Exchange Center Game 
Build bots to trade on the exchange and compete with other players. A simulation of the stock market.

This stock exchange simulator has the following constraints:
1. Only continuous phase implemented
2. Only limit orders and integer price values

---

## Installation
- Ensure rust is installed
- Next clone and `cd` into the repository and run `cargo update`
- To run the program, use `cargo run`

---

## Usage 
The program exposes a few APIs to help interact with it.

### Healthcheck 
```HTTP 
GET /healthcheck
```
A simple API to test out whether the server is up and running.

### Order Management
#### New Order
```HTTP
POST /order 
```
This is an endpoint to add new orders. It takes in the following `json` as the request body.
```json 
{
    "order_core": {
        "username": "nihal_ramaswamy",
        "security_id": "test"
    },
    "price": 6,
    "quantity": 6,
    "is_buy_side": false
}
```
It outputs the following response.
```json
{
    "order_core": {
        "username": "nihal_ramaswamy",
        "order_id": "random_order_id",
        "security_id": "test"
    },
    "status": Accepted
}
```
If there is any error, it can show the error in the status field.

#### Modify Order 
```HTTP 
PUT /order 
```
This is an endpoint to modify an existing order. It takes the following `json` as the request body. 
```json 
{
    "order_core": {
        "username": "nihal_ramaswamy",
        "order_id": "order_id",
        "security_id": "test"
    },
    "price": 7,
    "quantity": 6,
    "is_buy_side": true
}
```
It outputs the following response.
```json 
[
    {
        "order_core": {
            "username": "nihal_ramaswamy",
            "order_id": "order_id",
            "security_id": "test"
        },
        "status": Accepted
    },
    {
        "order_core": {
            "username": "nihal_ramaswamy",
            "order_id": "order_id",
            "security_id": "test"
        },
        "status": Accepted
    }
]
```
#### Cancel Order 
```HTTP 
DELETE /order
```
This is an endpoint to delete existing orders. It takes the following `json` as the request body. 
```json 
{
    "order_core": {
        "username": "nihal_ramaswamy",
        "order_id": "order_id",
        "security_id": "test"
    },
    "is_buy_side": true
}
```
It outputs the following response. 
```json 
{
    "order_core": {
        "username": "nihal_ramaswamy",
        "order_id": "order_id",
        "security_id": "test"
    },
    "status": Accepted
}
```

### Market Data Management 
#### Spread 
```HTTP 
GET /order/spread/<security_id>
```
It outputs the spread with the following response.
```json 
{
    "Ok": 3
}
```
#### Existing Ask Orders and Bid Orders
```HTTP 
GET /order/ask_orders/<security_id>
GET /order/bid_orders/<security_id>
```
It outputs the following response.
```json 
{
    "level": {
        "10": {
            "price": 10,
            "order_entries": [
                {
                    "order": {
                        "order_core": {
                            "username": "nihal_ramaswamy",
                            "order_id": "order_id",
                            "security_id": "test"
                        },
                        "price": 10,
                        "initial_quantity": 6,
                        "current_quantity": 6,
                        "side": "Ask"
                    },
                    "creation_time": 1701233450545550000
                }
            ]
        }
    }
}
```

### Trades 
```HTTP 
GET /order/trades 
```
This is an endpoint which continuously checks for trades which are done and it outputs it in an event stream.
This runs every single nanosecond and shows a response when a trade happens.
This also sends a heartbeat every second to ensure the connection is still alive.
When a trade happens, the output can look like the response shown below.
```json 
[
    {
        "buy_trade": {
            "username": "test",
            "order_id": "test1",
            "security_id": "test"
        },
        "sell_trade": {
            "username": "test1",
            "order_id": "test2",
            "security_id": "test"
        },
        "price": 6,
        "quantity": 5,
        "trade_id": "trade_id",
        "trade_time": "2023-11-29T09:58:09.779691+05:30"
    }
]
```

---

## License
[License](./LICENSE)


### TODO
- [ ] Add logging and writing to DB



