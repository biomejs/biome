FROM node:slim
COPY target/debug/biome /usr/local/bin/biome
RUN useradd -m alice && \
    useradd -m bob && \
    su - alice -c 'whoami && biome start && biome __print_socket' && \
    su - bob -c 'whoami && biome start && biome __print_socket'
