# Parser Module

This module contains most of the parsing logic, the parent module abstracts 
parsing to provide a nice interface for use within other programs. Parsing
is done character by character, and the parser consumes "buffers" containing
data to parse. This is done because most data is buffered, i.e: data from an HTTP
connection has a buffer size of the chunk.
