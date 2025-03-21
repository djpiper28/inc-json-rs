# `inc-json-rs` An Incremental JSON Parser That Uses Tokio

CURRENTLY A WIP.

## Why Do I Need This Library?

You have a big JSON file (a few hundred megabytes) and reading the file then parsing it seems awfully slow.

This library lets JSON be parsed incrementally from a stream. This means that for big streams you can be processing the data whilst it is 
still reading it form the network/disk. For example when reading from a hard drive you can be processing whilst the disk is working.

## How It Works

You define the shape of a JSON object and what to do for each object / primitive you care about. As the stream is read from the parse
object you have created user defined code is called (in the order of JSON input) allowing for the parsing of the data incrementally.
