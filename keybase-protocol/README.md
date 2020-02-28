# keybase-protocol
Just the protocol files of keybase/client

For the source, see: https://github.com/keybase/client/tree/master/protocol/avdl

# HOWTO
FYI how to update:
```
~/code/keybase-client master
❮ git remote add protocol-only git@github.com:MarcoPolo/keybase-protocol.git

~/code/keybase-client master
❯ ls protocol/avdl 
chat1		gregor1		kbgitkbfs1	keybase1	stellar1

~/code/keybase-client master
❯ git subtree push --prefix=protocol/avdl protocol-only update
```
