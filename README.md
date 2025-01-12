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
          "error": "",
      },
      "on-click": "xdg-open https://pro.kraken.com/app/trade/eth-chf",
  },
}
```

## Styling
This module sets the following css classes: `.negative`, `.positive`, `.error`.

Example:
```css
#custom-kraken.positive {
  color: #81b57f;
}
#custom-kraken.negative {
  color: #b5867f;
}
#custom-kraken.error {
  color: #f00;
}
```

## Todo(nt?)'s
* [x] Actually make shown symbol configurable
* [ ] Multiple Symbols
* [ ] consolidate multiple instances (eg. multiple displays, multiple modules) to use only 1 websocket

These require authentication
* [ ] Open trade orders
