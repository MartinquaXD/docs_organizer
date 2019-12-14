#Purpose
## Basic Idea
This tool will someday organize all your documents in a convenient manner.

To accomplish this you can take a photo of your document, optionally tag it and send that to a server. The server will then store the image
in an organized file structure und will recognize the text in it. This text will then be fed into a database for later
lookups.

To find some stored document, there will be a simple website on which you get simple filter functions. Full text search 
along with filters by date and tags will find your document quickly.

## Possible Features
After the text recognition is accurate enough there should be many ways to analyze that data.

This tool could analyze receipts and help with bookkeeping by creating simple graphs and tables.

#Tech
##Server
The server will be written in rust and use tokio to handle many connected clients concurrently. 

To allow a responsive, low overhead connection clients will communicate via websockets and a binary protocol like CBOR 
with the server. It is planned to allow live updates of the clients but only after the basic features are done.

Images will be preprocessed with opencv to boost the accuracy of the character recognition with tesseract.

The text search will be provided by an Elasticsearch database.

##Frontend (Website)
The web frontend will be written in Typescript and uses the usual React/Redux stack. Webpack will probably used as the 
bundler. 

I am very interested to explore use cases for WebAssembly compiled from Rust.

##Future
If this project is popular enough there could be a Android and iOS app.