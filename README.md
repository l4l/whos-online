## Whos-online

[![Build Status](https://travis-ci.org/l4l/whos-online.svg?branch=master)](https://travis-ci.org/l4l/whos-online)

### Usage
```
  whos-online (-b | --bot) [--token=<bot_token>] [--host=<host>]
  whos-online (-c | --collector)
  whos-online (-d | --daemon) <token> <user> [--host=<host>] [--period=<period>] [--workspace=<ws>]
  whos-online (-h | --help)
  whos-online --version

Arguments:
  <token>                 Toggl API token that you may find at toggl.com/app/profile
  <user>                  Username for data reporting (handy to use telegram nickname)

Options:
  -b --bot                 Launch bot
  -c --collector           Launch tracking data collector
  -d --daemon              Launch submitting daemon
  --token=<bot_token>  Telegram bot token
  --host=<host>            Host for data reporting [default: http://127.0.0.1:8080]
  --period=<period>        Period of data reports [default: 30]
  --workspace=<ws>         Toggl workspace for monitoring
  --version                Show version.
  -h --help                Show this help.
```

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
