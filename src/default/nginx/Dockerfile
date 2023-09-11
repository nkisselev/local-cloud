FROM nginx:1.21-alpine
LABEL authors="linitpro"

#ENV http_proxy "http://mwgapp.test.tsb:3128"
#ENV https_proxy "http://mwgapp.test.tsb:3128"
RUN apk add curl
RUN apk add linux-headers
RUN apk add build-base

# include nginx.conf
COPY nginx.conf /etc/nginx/nginx.conf


ENTRYPOINT ["top", "-b"]