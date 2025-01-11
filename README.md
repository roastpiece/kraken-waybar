# Waybar config
```jsonc
{
  "custom/kraken": {
      "exec": "kraken-waybar ETH/CHF",
      "restart-interval": 30,
      "return-type": "json",
      "escape": true,
      "format": "{icon} {0}",
      "format-icons": {
          "negative": "󰔳",
          "positive": "󰔵",
      },
      "on-click": "xdg-open https://pro.kraken.com/app/trade/eth-chf",
  },
}
```

## Todo(nt?)'s
* [x] Actually make shown symbol configurable
* [ ] Multiple Symbols
* [ ] consolidate multiple instances (eg. multiple displays, multiple modules) to use only 1 websocket

These require authentication
* [ ] Open trade orders
