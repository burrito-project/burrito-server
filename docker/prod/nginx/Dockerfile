FROM nginx:1.27.1-alpine

ARG DOMAIN_NAME
ENV DOMAIN_NAME=${DOMAIN_NAME:?}

ARG WEB_DOMAIN_NAME
ENV WEB_DOMAIN_NAME=${WEB_DOMAIN_NAME:?}

ENV TZ=America/Lima

RUN rm /etc/nginx/conf.d/default.conf
COPY ./docker/prod/nginx/nginx.conf /tmp/nginx.conf.tpl

RUN : \
    && envsubst \
        '${DOMAIN_NAME} ${WEB_DOMAIN_NAME}' \
        < /tmp/nginx.conf.tpl \
        > /etc/nginx/conf.d/nginx.conf \
    && :
