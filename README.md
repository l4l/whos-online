## Whos-online

[![Build Status](https://travis-ci.org/l4l/whos-online.svg?branch=master)](https://travis-ci.org/l4l/whos-online)

Tool that is answer the question who is online of your colleagues.
It's consist of the three main parts.

### whosd

__whosd__ is a daemon that periodically send the status from your toggl account. That's why you need to give it your token (you may find it at the [profile settings](https://toggl.com/app/profile)). Besides you may set the __host__ where submit your statuses and a __period__ or how often should it be done.

The defaults are:
- host: http://127.0.0.1:8080
- period: 30s

### whosc

__whosc__ is stands for collector. Its aim is grub all of the statuses.

### whosb

__whosb__ is a bot that answer the main question.
