# Mini HTTP server

A simple single threaded server which can be used during development, or for light traffic applications.

The application will only serve content which is located in the same directory as own executable. 
This includes any content in subdirectories.

By default, the application starts listening on all available interfaces and associated addresses, on port 8080.

Procedure:
* Copy **mini-http** executable to target directory: `cp mini-http /path/to/target/directory/`
* Start **mini-http**: `/path/to/target/directory/mini-http`
* Open browser i.e. http://localhost:8080/index.html

The port and the address can be provided as command line arguments:
> mini-http 192.168.1.23 8090
> 
> mini-http 8090 192.168.1.23

Features:
* directory listing
* content type detection
