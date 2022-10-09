#!/bin/sh

rm message* transcribed_* tmp.wav* 

env DISCORD_TOKEN=$( cat ../token.env ) GUILD_ID=$( cat ../guild_id.env ) APP_ID=$( cat ../application_id.env ) cargo run ./tr-tr-trevor
