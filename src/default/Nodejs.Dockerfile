FROM node:20-alpine3.17
LABEL authors="nkisselev"

#ENV http_proxy "http://mwgapp.test.tsb:3128"
#ENV https_proxy "http://mwgapp.test.tsb:3128"
RUN apk add curl
RUN apk add linux-headers
RUN apk add build-base