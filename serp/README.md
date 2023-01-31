# serp (scorched earth relay protocol)

This is a protocol I just made up as an easy way to get two players of scorched earth connected to each other based on a room ID that the game host generated

Maybe there's a better way to do this, but I don't care, I couldn't find anything I liked, and I wanted to make up a protocol.

## quick overview of the protocol

Everything is a line of ASCII text sent over TCP ending in a single `\n`

There are two main commands:

### Host

Hosts a room with a certain ID. An ID is assumed to be a hash of a public key that only the two players know, so the server can connect those two players together, but it can be anything that doesn't contain a newline.

Protocol:

Client sends `host {id}\n`, where `{id}` is the ID (can be any text)

Server responds with `ok\n` if the request was received, `invalid\n` if the command was invalid, or `fail\n` if the room already exists

### Conn

Connects to a room created with `host` using the same ID

Protocol:

Client sends `conn {id}\n`

Server responds with `ok\n` if the request was received, `invalid\n` if the request was invalid, or `fail\n` if the room doesn't exist

## Connection

After either of these, the server will send `connected\n` to both players (the one who sent `host` and the one who sent `conn`)

After this, everything sent by one player will be relayed directly to the other player, basically creating a direct connection between the players and the server no longer does anything
