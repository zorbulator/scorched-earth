# serpd (scorched earth relay protocol daemon)

See `../serp` for information about the serp protocol.

This is a simple server using a protocol I made up that can connect two players based on a game id that the host would send to the person they want to play with.

Usage: just run `serpd`

`serpd 127.0.0.1:xxxx` to use a custom port / bind to a specific address

`RUST_LOG=none serpd` to disable logging
