A telegram chatbot made in rust

Dependencies:
-serde_json
  for handling jsons
-minreq
  for sending requests to the telegram api
-tiny_http
  for recieving requests sent to the server
-dotenv
  for accessing the API keys stored in the .env file

For local deployment
-run the code
-using ngrok or alternative, expose port 8080 of the localhost to the web
-setup webhook connection done automatically by telegram by
  running: curl -X GET "https://api.telegram.org/bot<API_KEY>/setWebhook?url=<ngrokUrl>" or
  getting get request on postman to the "https://api.telegram.org/bot<API_KEY>/setWebhook" with url set to the ngrok Url
