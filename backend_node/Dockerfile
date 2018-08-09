FROM node:latest

#create app dir
WORKDIR /api

COPY package*.json .

RUN npm install

# RUN npm install --only=production

#bundle app source
COPY . .

EXPOSE 3000

CMD ["npm", "start"]