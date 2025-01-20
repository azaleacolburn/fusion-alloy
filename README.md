# fusion-alloy

Plugin package manager for Autodesk Fusion (formerly Fusion 360)

Development will start when I get annoyed enough with Fusion package management again.

## Structure

- An alloy is a plugin, it is a list of compositions which are differences between versions. This is just a git repo. Commits can be assigned names and / or versions by the creator or user
- The Compositions dir in every device contains all the compositions, theses can be used to download and verify alloys
- Once an alloy is installed, it's stored in the Furnace
