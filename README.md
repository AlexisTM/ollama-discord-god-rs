OpenAI Discord bot
==================

This is a refactoring of https://github.com/AlexisTM/gpt3-discord-bot in rust for fun purpose.

Environment keys:
- AI21_API_KEY: AI21 Token, free to get from https://www.ai21.com/studio
- DISCORD_BOT_TOKEN: Discord token to allow the bot to connect, get it from https://discord.com/developers/applications
- GODS_PATH: The paths to the god configurations, to be selected with `god set`
- REDIS_URI: redis://user:pass@127.0.0.1:6379/ by default, this allows for permanent changes accross reboots of the bot.

Come and test on Discord!: https://discord.gg/Y8XPcj2Q

Commands
=============

- `God are you there?`: Replies yes if the server runs
- `God: `: Answers as a god. The `God:` part is not sent to the god.
- `God get` Gets the current setup of the god
- `God set` Open the menu to modify the god: Change name, context, interactions and saves it within redis.
- `God clean`: Cleans the god memory
- Any sentence with `godname` will be taken in account (Not yet there)

Technical help on how to make a Discord bot:
==================

Create a bot application: https://discordpy.readthedocs.io/en/stable/discord.html

Configure intents for your bot: https://discordpy.readthedocs.io/en/stable/intents.html

In the oauth section of discord dev portal, make a link to allow your bot to join your server such as:

https://discord.com/api/oauth2/authorize?client_id=APPID&permissions=2215115840&scope=bot

In this case, we only need the bot scope and READ/WRITE messages permissions/

Example
===========


`god get` spawns the bot menu, to change the config

![Menu showing: Change name, change context, add interaction, clear interactions, save the god](/doc/menu.png)

Clicking on the buttons creates a modal for easy configuration

![Modal showing asking to change the name](/doc/god_name_change.png)

`god get` shows the current god configuration

![The output of the god get command, showing the bot name, context, available interactions and memory used for generation](/doc/god_name_change.png)
