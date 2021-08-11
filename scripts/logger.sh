# This script generates jsons that contain
# logger url using Heroku API.

# Remove deprecated jsons
rm -rf *.json;

curl -o "log_request_details.json" -n -X POST https://api.heroku.com/apps/twitter-forward-tg-bot/log-sessions \
  -d "{\"lines\": 1500}" \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";
