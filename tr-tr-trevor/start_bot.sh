#!/bin/sh

env DISCORD_TOKEN=$( cat ../token.env ) GUILD_ID=$( cat ../guild_id.env ) cargo run ./tr-tr-trevor
