#!/usr/bin/env python

from websockets.sync.client import connect

def hello():
    with connect("ws://localhost:9001") as websocket:
        websocket.send("/imap/login\r\nemail=test1928346534@gmail.com\npassword=klxhmjkhojvzuphv\naddress=imap.gmail.com\nport=993")
        message = websocket.recv()
        print(f"Received: {message}")

        websocket.send("/imap/messages\r\nsession_id=0\nmailbox=INBOX\nnr_messages=5")
        message = websocket.recv()
        print(f"Received: {message}")

        websocket.send("/imap/mailboxes\r\nsession_id=0")
        message = websocket.recv()
        print(f"Received: {message}")

        websocket.send("/imap/message\r\nsession_id=0\nmailbox=INBOX\nmessage_uid=22")
        message = websocket.recv()
        print(f"Received: {message}")


        websocket.send("/imap/logout\r\nsession_id=0")
        message = websocket.recv()
        print(f"Received: {message}")

hello()
