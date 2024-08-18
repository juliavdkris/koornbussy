FROM docker.io/nginx:1.27-alpine

COPY koornbussy.js /var/www/html/koornbussy.js
COPY nginx.conf /etc/nginx/conf.d/default.conf
