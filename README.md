# Arma Webhooks

## How to use this addon?

### Config Examples

```yaml
discord:
  test:
    endpoint: https://discord.com/api/webhooks/.../...
    username: Test Logs
    content: A Warzone has spawned at ***! Get there quickly...

  hacks:
    endpoint: https://discord.com/api/webhooks/.../...
    username: Hack Logs
    avatar_url: https://cdn3.emoji.gg/emojis/3213-banhammer.png
```

### Usage Examples

This will return `"this is a test"` into the channel.

```
"arma_webhooks" callExtension ["discord", ["hacks", "this is a test"]];
```

To save yourself from writing the same logs/entries all the time you can use the shorthand feature that is built into the extension.
Providing that you have added a `content` option with a string with a match point defined using `***`

```
"arma_webhooks" callExtension ["discord", ["test", "Kavala"]];
```

The output of this would look like: `"A Warzone has spawned at Kavala Get there quickly..."`
