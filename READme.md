# torrent client with rust

inspired by
[tutorial](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)


## features
- bencode encoder
- bencode decoder


<img height='200' src='./logos/logo4.svg'>











## TODO
- [x] bencode decoder
- [x] bencode encoder
- [x] get peers with tracker
    - [x] request udp connection
    - [x] recieve peers
- [ ] download
    - [x] tcp connection
    - [ ] grouping
    - [ ] handshakes
    - [ ] pieces
    - [ ] handling messages
    - [ ] managing connections and pieces



## to fix
- [ ] the doubling of the timeout duration doesnt effect the actual timeout