# How to update git subtree:

~/code/rusty-keybase-protocol master
❮ git remote add -f keybase-protocol git@github.com:MarcoPolo/keybase-protocol.git

~/code/rusty-keybase-protocol master
❮ git subtree pull --prefix=keybase-protocol/ keybase-protocol master --squash

More info: https://www.atlassian.com/git/tutorials/git-subtree
