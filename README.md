<div align="center">
  <h1><code>Game for programmers</code></h1>

  <p>
    <strong>Game for programmers in which you collide your programs in battle
  </p>
</div>

## Start

Download the project, install the [Rustup](https://rustup.rs/), start the server:

```sh
cargo run
```

## Action example JSON

Communication with the server takes place using JSON, you connect to the server using a websocket and as soon as the server signals the start of the game, you send messages in the form:

```json
{
    "actions": [
        {
            "action": "Move",
            "direction": {
                "direction": "Right"
            },
            "range": 2
        },
        {
            "action": "Reload"
        }
    ]
}
```

After receiving a message from the client (or without waiting for it if it timed out), the server will update the game state and return the current game state. All repeats.
