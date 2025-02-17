# torrent client with rust

sources

[tutorial in js](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)

[tutorial in go](https://blog.jse.li/posts/torrent/)

[torrent specification](https://wiki.theory.org/BitTorrentSpecification#piece:_.3Clen.3D0009.2BX.3E.3Cid.3D7.3E.3Cindex.3E.3Cbegin.3E.3Cblock.3E)

<div align='center' style>
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
- [x] parse torrent file
- [x] calculate info hash
- [x] get peers with tracker
- [ ] download
    - [x] tcp connection
    - [x] handshakes
    - [x] messages
    - [x] pieces
    - [x] managing connections and pieces
    - [x] downloading pieces
    - [ ] writing pieces to files
    - [ ] make it work with both single file and multiple files torrents


## extra features to implement
- [ ] recovery from all types of errors and disconnections (no unwraps in this app should be left)
- [ ] no reliance on announce (implementing DHT)
- [ ] magnet links
- [ ] when download is interrupted pick off where it started 


## Errors:
- [ ] restart clients (clients keep breaking with): 
  - [ ] Resource temporarily unavailable (os error 11) ---> (message.rs line:107)
  - [ ] failed to fill whole buffer ---> (message.rs line:127)
  - [ ] Broken pipe (os error 32)
- [ ] receiving pieces in the wrong order "expected index: 44, got: 9"
- [x] insane CPU usage almost 100% (i5 8600k):
```rust
// this one was caused because of this code block which was ran everytime we received a PIECE message from a peer
// the CPU consumption is still a bit high and can be lowered but thats not a priority right now
// NOTE: this was discovered thanks to flamegraph profiler (https://github.com/flamegraph-rs/flamegraph)
progress.buf.splice((buf_begin as usize).., res_buf);
let mut file = std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open("buffers.txt")
    .unwrap();

writeln!(file, "{:?}", res_buf).unwrap();
```

## to fix
- [ ] some Peers responses come out as binary instead of text of Ip addresses (maybe change it to udp (https://www.bittorrent.org/beps/bep_0015.html))
- [ ] refactor and improve naming 
- [ ] divide code to more functions for better testing
- [ ] add connection resets in case of connection errors (using MSCP channels maybe)
- [x] send piece result to a writer thread with MSCP channels
- [ ] search for more clients regularly
- [ ] add documentation
- [ ] add all errors handling
- [ ] add all components tests