# MDDF
A simple manga downloader, it downloads mangas and turns the chapters in pdf

Currently only supports mangakakalot links

## Env Configurations file example
```
REFERER_TO_HEADER=<The manga host url>
CHAPTERS_FOLDER=final-chapters
DEFAULT_SCRAPS_FOLDER_NAME=scraps
DEFAULT_SEARCH_LINK=<The search link for the manga host url>
MODE=debug | normal

```
# Another flippin' update
Now I've managed to make it work in a absolute move, I used a simple header paramether to 
bypass the 403 http error. And it's working now, I'll work in some new bugs due to the code
being made while I'm eepy