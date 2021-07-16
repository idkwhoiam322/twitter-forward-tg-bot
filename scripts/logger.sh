# This script generates jsons that contain
# logger url using Heroku API.
rm -rf "worker_log_details.json";
curl -o "worker_log_details.json" -n -X POST https://api.heroku.com/apps/twitter-forward-tg-bot/log-sessions \
  -d '{
  "dyno": "worker",
  "lines": 1500,
  "source": "app",
  "tail": true
}' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";

rm -rf "api_log_details.json";
curl -o "api_log_details.json" -n -X POST https://api.heroku.com/apps/twitter-forward-tg-bot/log-sessions \
  -d '{
  "dyno": "api",
  "lines": 1500,
  "source": "app",
  "tail": true
}' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";

rm -rf "heroku_worker_log_details.json";
curl -o "heroku_worker_log_details.json" -n -X POST https://api.heroku.com/apps/twitter-forward-tg-bot/log-sessions \
  -d '{
  "dyno": "api",
  "lines": 1500,
  "source": "heroku",
  "tail": true
}' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3" \
  -H "Authorization: Bearer $HEROKU_API_KEY";
