FROM node:latest

WORKDIR /client

COPY package*.json .

RUN npm install

COPY . .

#opening port for app + websocket as well.
EXPOSE 3001 38373

CMD ["npm", "start"]