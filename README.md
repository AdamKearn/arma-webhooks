# Arma Webhooks

## Instalation

You will need to first download the binary from the releases tab [here](https://github.com/AdamKearn/arma-webhooks/releases/latest).
You will then need to place the binary within a folder called `@arma_webhooks`.  After you can then create a `config.yaml` using one of the examles below.

The folder scructor should look like the following within your server files.

```
arma-server/
└── @arma_webhooks/
    ├── arma_webhooks_x64.dll
    ├── arma_webhooks_x64.so
    └── config.yaml
```

Once the files are in place, update your server’s launch parameters.  
Add or update the following flag in your startup script or config:  

```
... -serverMod=@arma_webhooks;@other_example_mod ...
```

## How to use this addon?

### Config Examples

```yaml
discord:
  warzone:
    endpoint: https://discord.com/api/webhooks/.../...
    username: Test Logs
    content: A Warzone has spawned at ***! Get there quickly...

  hacks:
    endpoint: https://discord.com/api/webhooks/.../...
    username: Hack Logs
    avatar_url: https://cdn3.emoji.gg/emojis/3213-banhammer.png

rest:
  external_api:
    endpoint: https://api.example.com/v1/...
```

### Usage Examples

#### Discord
This will return `"this is a test"` into the channel.

```
"arma_webhooks" callExtension ["discord", ["hacks", "this is a test"]];
```

To save yourself from writing the same logs/entries all the time you can use the shorthand feature that is built into the extension.
Providing that you have added a `content` option with a string with a match point defined using `***`

```
"arma_webhooks" callExtension ["discord", ["warzone", "Kavala"]];
```

The output of this would look like: `"A Warzone has spawned at Kavala Get there quickly..."`

#### REST
Here is an example of sending a GET request.  I'm planning on adding all other REST methords in the future.

```
"arma_webhooks" callExtension ["rest:get", ["external_api", <message_body>]];
```

## Adding support for other webhook handlers.
I've designed this package to be as generic as possable and deliberlly make it moduler.
This then allows to easily create webhook wrappers for other sources such as Teams/Slack etc in the future.
