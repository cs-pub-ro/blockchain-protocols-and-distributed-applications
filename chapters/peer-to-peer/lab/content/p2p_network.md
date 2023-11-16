# Peer to peer network

For this part we will use libp2p was used by the guys that build IPFS and Filecoin.

First we need to spawn a p2p node

```
func RunSourceNode() {
 // start a libp2p node that listens on a random local TCP port,
 // but without running the built-in ping protocol
 node := CreateNode("/ip4/127.0.0.1/tcp/0")

 // configure our own ping protocol
 pingService := &ping.PingService{Host: node}
 node.SetStreamHandler(ping.ID, pingService.PingHandler)

 // print the node's PeerInfo in multiaddr format
 peerInfo := peerstore.AddrInfo{
  ID:    node.ID(),
  Addrs: node.Addrs(),
 }
 addrs, err := peerstore.AddrInfoToP2pAddrs(&peerInfo)
 if err != nil {
  panic(err)
 }
 fmt.Println("libp2p node address:", addrs[0])
//we need paralel interaction for this 
 go LogicNodeInteraction(node)

 for {
//here will be our local terminal to interact with the blockchain
  terminalview.TerminalView(node)
 }

}

```

Here we spawn a basic peer to peer Node now we need a way to interact with the node and some logic to be able to communicate with the other blockchains.

## Syncronise peers inside the network

So we have LogicNodeInteraction which will run in parallel with the terminal.

```
func LogicNodeInteraction(node host.Host) {
 //Set stream handler for the "/hello/1.0.0" protocol
 go node.SetStreamHandler("/transaction/1.0.0", func(s network.Stream) {
  log.Printf("/transaction/1.0.0 stream created")
  err := ReadTransactionProtocol(s)
  if err != nil {
   s.Reset()
  } else {
   s.Close()
  }
 })

 go node.SetStreamHandler("/mine/1.0.0", func(s network.Stream) {
  log.Printf("/mine/1.0.0 stream created")
  err := ReadMineProtocol(s)
  if err != nil {
   log.Printf("Error: %s", err)
   s.Reset()
  } else {
   log.Printf("Closing stream")
   s.Close()
  }
 })

 go node.SetStreamHandler("/messages/1.0.0", func(s network.Stream) {
  log.Printf("/messages/1.0.0 stream created")
  err := ReadMessagesProtocol(s)
  if err != nil {
   log.Printf("Error: %s", err)
   s.Reset()
  } else {
   log.Printf("Closing stream")
   s.Close()
  }
 })

}

```

Here we spawn 3 channels to listen for other peers(they need to send messages on one of this channels)

## So what are they doing?

This will be out channel of communication between our nodes.
```
"/transaction/1.0.0" - is responsible for transaction, here is the way our transaction will be send

”/mine/1.0.0" — is responsible for the blocks that are mined, they will come from other nodes to this one ( here will come blocks that are valid and invalid form the other peers)

”/messages/1.0.0" — here will come diffrent messages about the network(transaction that are not valid…etc) (we will not enter into this)```


