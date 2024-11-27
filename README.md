# HnyPt

A SSH HoneyPot built for research & educational purposes.

## Roadmap

- [x] Step One: Implement kube-rs

  - Have a pod running as the honeypot

- [x] Step Two: Setup MongoDB

  - Setup a pod with MongoDB
  - Setup a service for MongoDB
  - Have it connect to the application

- [x] Step Three: Implement RabbitMQ.
  - Setup RabbitMQ.
  - Setup P & C for logs

- [ ] Step Four: Implement Watcher.
  - Implement a Watcher for the honeypot.
  - Have it save logs in MongoDB via RabbitMQ.

- [ ] Step five: Fixes & Cleanups
  - Fix persistent volume for RabbitMQ & MongoDB.
  - Kill pods when exiting.
  - Multi-threaded message handling
   
