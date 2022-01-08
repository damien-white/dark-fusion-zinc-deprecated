# Zinc

## Project Description

Zinc is a key-value store meant to be lightweight, easy-to-use and flexible.

It aims to provide a performant solution that runs efficiently on IoT devices and other systems such
as servers with resource-constraints.

## Roadmap

The following list of features is not set in stone. Things may change at any time, especially during
early development.

### Server

- [ ] Event loop to listen for client connections
  - [ ] Non-blocking, asynchronous
- [ ] Logging and telemetry
  - [ ] Formatting layer(s)
  - [X] Storage layer
- [ ] Logger implementation using `tracing` + `tracing-subscriber`, etc.
- [ ] Maintain list of active connections
- [ ] Parse commands from clients
  - [ ] `GET`: retrieve values from store
  - [ ] `SET`: set new key-value pairs from store
  - [ ] `DEL`: removes key-value pairs from store
- [ ] Watch channel for hot-reloading
- [ ] Optimize for single-core, resource constrained systems
  - [ ] Support parallel processing through the use of an optional feature flag

### Client

- [ ] Command line argument parser
- [ ] Logging and telemetry
  - `tracing` + `tracing-subscriber`

### Common / Shared (Agent)

- [ ] Codec for bytes off the wire
  - [ ] `encode` method
  - [ ] `decode` method
- [ ] Encryption with authentication (AEAD and HMAC) <sup>Note: independent of TLS</sup>
  - [ ] Asymmetric encryption
  - [ ] Symmetric encryption
  - [ ] Key exchange algorithm
  - [ ] HMAC to verify data has not been tampered with
- [ ] Task-handling queue
  - [ ] Buffered channels (`mpsc` + `spsc`)
  - [ ] FIFO scheme
  - [ ] Optimized through use of proper concurrent data structures

# Licensing

This software is licensed by the [MIT License](LICENSE).
