# discord-gif-saver

Downloads all your saved gifs from Discord.

## How to use

1. Open Discord, and then open the inspector pane. Go to the network tab.
2. Save/unsave a GIF -> this will result in a request called `2` to appear in the network tab. Click on the request, and then click payload. You will be able to copy the actual string (it is base64 encoded).
3. Store this base64 text in `sources/response.txt` (relative to where you are running cargo/the rust program). **Make sure you remove the surrounding quotes.**
4. Create a new directory called `output`.
5. Run the program with `cargo r -r`.
