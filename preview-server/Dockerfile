FROM node:24-alpine

WORKDIR /app

# * Use apk instead of apt-get for Alpine
RUN apk update && \
    apk add --no-cache curl

RUN ln -s /usr/local/bin/npm /usr/bin/npm

COPY package.json .

RUN npm install

COPY . .

EXPOSE 8000
EXPOSE 10000-10100

CMD [ "node", "server.js" ]