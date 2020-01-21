#!/bin/bash

API_KEY="YOURAPIKEY"

mkdir -p ${HOME}/.config/weather_util
cat > ${HOME}/.config/weather_util/config.env <<EOL
API_KEY=$API_KEY
API_ENDPOINT=api.openweathermap.org
EOL
