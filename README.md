Ollama Discord bot
==================

This project allows you to run your **custom** Ollama model **locally** as a Discord bot. It can:
- Mimick a virtual character (see Mavin, Kirby or Pastafari)
- Provide mental and relationship help (see Samantha), `This doesn't replace professional help`
- Python code teacher
- Dungeon Master assitant to generate encounters on the flight 

Quickstart
=============

```
# Use WSL on Windows, skip on Linux/MacOS
wsl --install Ubuntu-22.04
wsl

# Install Ollama and pre-pull mistral
curl -fsSL https://ollama.com/install.sh | sh
ollama pull mistral

# Start your bot
git clone https://github.com/AlexisTM/ollama-discord-god-rs
cd ollama-discord-god-rs
ollama create marvin -f modelfiles/marvin.modelfile
DISCORD_BOT_TOKEN=[YOUR_DISCORD_BOT_TOKEN] cargo run --release gods/marvin.json
```

Commands
=============

- `Direct message`: The god replies to the message
- `/botname prompt`: Slash command to contact a specific god
- `/clear`: Slash command to remove the god memory

Modelfiles
===============

Modelfiles can be used to preseed your bot directly within ollama to avoid having to send your whole prompt each time. This also simplifies handling message history.

Create your [Marvin modelfile](modelfiles/marvin.modelfile), seed your network and start the bot.

```bash
ollama create marvin -f modelfiles/marvin.modelfile
ollama create kirby -f modelfiles/kirby.modelfile
ollama create pastafari -f modelfiles/pastafari.modelfile
ollama create samantha -f modelfiles/samantha.modelfile

DISCORD_BOT_TOKEN=DISCORD_BOT_TOKEN cargo run --release modelfiles/marvin.json
```

FAQ
========

Missing pkg-config on WSL
--------------

```bash
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
```

Technical help on how to make a Discord bot:
==================

Create a bot application: https://discordpy.readthedocs.io/en/stable/discord.html

Configure intents for your bot: https://discordpy.readthedocs.io/en/stable/intents.html

In the oauth section of discord dev portal, make a link to allow your bot to join your server such as:

https://discord.com/api/oauth2/authorize?client_id=APPID&permissions=2215115840&scope=bot

In this case, we only need the bot scope and READ/WRITE messages permissions
