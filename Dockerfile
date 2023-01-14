FROM alpine:latest
WORKDIR /app

ARG execfile=target/release
ADD $execfile/DeconzMqttBridge DeconzMqttBridge
RUN chmod +x DeconzMqttBridge
ENTRYPOINT ["./DeconzMqttBridge"]