OpenAI Discord bot
==================

This is a refactoring of https://github.com/AlexisTM/gpt3-discord-bot in rust for fun purpose.

Environment keys:
- AI21_API_KEY
- DISCORD_BOT_TOKEN
- REDIS_URI: redis://user:pass@127.0.0.1:6379/

Come and test on Discord!: https://discord.gg/Y8XPcj2Q

Commands
=============

- `God are you there?`: Replies yes if the server runs
- `God: `: Answers as a Kirby god.
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
