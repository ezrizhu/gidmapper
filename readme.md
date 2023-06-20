Usage: `gidmapper <pid to change the mapping of> <uid to map the user to> <amount of groups to map>`

Example: `gidmapper 1234 1 65535` to map the process 1's UID to 0 (root), and
map all groups starting from caller's group.
