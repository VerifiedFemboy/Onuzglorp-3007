# How to use the bot?
First you need to get token from [discord developers](https://discord.com/developers/applications)

# Putting the token into .env
```
DISCORD_TOKEN={discord bot token} // For release bot 
DISCORD_TEST_TOKEN={discord bot token} // For testing bot
MONGO_URI={mongodb uri connection} // For database connection
```

# Running the bot
```
cargo run --release // to run the release bot with the main token
cargo run dev // to run the test bot
```

# Todos - checked means finished
- [x] Make cache manager to limit those requests to Tuforums API
- [ ] Connect cache manager with leaderboard, level and clear info
- [ ] Make calcscore_j command that calcs score by judgements (if it is going to be useful)
- [x] Log saver