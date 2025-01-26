# torrent client with rust

sources

[tutorial in js](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)

[tutorial in go](https://blog.jse.li/posts/torrent/)

[torrent specification](https://wiki.theory.org/BitTorrentSpecification#piece:_.3Clen.3D0009.2BX.3E.3Cid.3D7.3E.3Cindex.3E.3Cbegin.3E.3Cblock.3E)

<div align='center' styl>
    <img height='200' src='./logos/logo.svg'>
</div>


## notes
- copying the contents of a .torrent file doesn't work (idk why)
- copying the entire file then changing it's name is ok


## features
- NOTICE: this client only works with torrent files that have an announce url in them
- bencode encoder














## TODO
- [x] bencode decoder
- [x] get peers with tracker
- [ ] download
    - [x] tcp connection
    - [x] handshakes
    - [x] messages
    - [x] pieces
    - [x] managing connections and pieces



## to fix
- [ ] sometimes clients work in sequence instead of in paralllel (needs to be fixed) 
- [ ] add documentation
- [ ] add all errors handling
- [ ] add all componenets tests