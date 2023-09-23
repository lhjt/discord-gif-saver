# gif-saver

Downloads all your saved gifs from Discord.

## How to use

1. open discord
2. then open the inspect element pane
3. then go to network in inspect element
4. then save or unsave a gif on discord
5. there should be a request with the name 2
6. you will then want to click on that and then check the payload that is sent
7. there should be a base64 string
8. you want to copy the base64 string and put it in a file called sources/response.txt (make sure you remove the quotes)
9. then create a folder called output
10. then run the software with cargo r --release

> I will do a proper writeup later
