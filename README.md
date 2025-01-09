# Waybar config
```jsonc
{
  "custom/kraken": {
      "exec": "kraken-waybar",
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
