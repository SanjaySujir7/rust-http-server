# Rust Web Server

A web server built from scratch in Rust to learn low-level networking, the HTTP protocol, and Rust's ownership system without using web frameworks.

## About

This project is a personal learning project focused on understanding how web servers work internally.

Instead of relying on existing web frameworks, the goal is to build the fundamental components manually and understand how they work under the hood.

## Learning Philosophy

This project emphasizes understanding concepts and writing the implementation manually. The focus is on learning Rust, networking, and HTTP by building each component step by step rather than relying on generated solutions or complete framework abstractions.

Every feature is implemented to understand the underlying concepts, trade-offs, and design decisions involved in building a web server.

## What I'm Learning

* TCP networking
* HTTP protocol
* Rust ownership and borrowing
* Structs and implementations
* Request parsing
* Routing
* Connection handling
* Systems programming concepts

## Current Features

* TCP listener
* Accept incoming connections
* Read raw bytes from clients
* Parse incoming HTTP requests
* Send HTTP responses
* Basic request logging

## Planned Features

* [x] TCP server
* [x] Read incoming requests
* [x] Send HTTP responses
* [ ] HTTP request parser
* [ ] Routing system
* [ ] Static file serving
* [ ] HTML page serving
* [ ] MIME type support
* [ ] Error pages
* [ ] Multiple routes
* [ ] Multithreading
* [ ] Middleware support

## Running

```bash
cargo run
```

Open:

```text
http://127.0.0.1:8080
```

Or test using:

```bash
curl http://127.0.0.1:8080
```

## Why This Project?

The purpose of this project is educational. Building a web server from scratch provides a deeper understanding of networking, HTTP, and Rust's memory and ownership model.


