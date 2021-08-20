FROM rust:latest

# 初回のみ
#WORKDIR /workspace
#RUN cargo init socket-programing

RUN apt update
RUN apt-get install -y \
	curl \
	netcat 
#CMD ["cargo", "run"]

#EXPOSE 80/udp