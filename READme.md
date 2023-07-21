# torrent client with rust

inspired by
[tutorial](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)


<div align='center' styl>
    <img height='200' src='./logos/logo.svg'>
</div>


## notes
- copying the contents of a .torrent file doesn't work (idk why)
- copying the entire file then changing it's name is ok


## features
- bencode encoder
- bencode decoder













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
- [ ] the doubling of the timeout duration doesn't effect the actual timeout