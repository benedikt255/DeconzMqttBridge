FROM debian:stable-slim
WORKDIR /app

ARG execfile=target/release
ADD $execfile/DeconzMqttBridge DeconzMqttBridge
RUN chmod +x DeconzMqttBridge
CMD ["./DeconzMqttBridge"]