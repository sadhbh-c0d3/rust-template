FROM ubuntu:latest

RUN apt update

RUN apt install -y build-essential curl

RUN mkdir -p /home/app

ADD "./init.sh" "/home/app"

WORKDIR "/home/app"

RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf > ./rusetup-download.sh

RUN sh ./rusetup-download.sh -y

CMD ["./init.sh"]
