# 🐕‍🦺 Watchdog

An application to ensure that your interactive installation software (ahem, TouchDesigner application) is running 24/7.

Of course, your deployed application is perfectly bug free, performant, and definitely not ever never going to crash, and leave a giant LED wall empty or frozen. But... wouldn't you sleep better at night, knowing that even if the impossible happened, and some quantum bits were flipped, and it _did_ crash, that the crash would automatically be detected, the application restarted, an email alert sent to your team, all before your client could notice and call you out freaking out that "ITS BROKEN!"

### Why not pm2?
[PM2](https://pm2.keymetrics.io/) is an excellent piece of software to persist many types of applications, but I have been unable to integrate it in a clean, successful, and controlable way with TouchDesigner, and other miscellaneous visual-output applications--where the process itself might not fully be gone, and from pm2's perspective all is well, but in reality, the app is frozen. Watchdog maintains a more accurate health status by pinging for `heartbeats`, over OSC on localhost.


### Stay Tuned 
This is a work in progress, entirely un-ready for production, and is a rewrite of a minimally scoped but perfectly viable version I built in Node.js. I am also using this project as a means to learn Rust.

Further explanation and documentation to come.

# TO DO:

### Config
- [ ] Email client UI spruce up
- [ ] Email client password encrypt/decrypt
- [ ] Email custom types with validation
- [ ] etc

### Apps
- [ ] Launch apps lol
- [ ] Listen for osc
- [ ] Relaunch apps
- [ ] Kill apps
- [ ] Design ui
- [ ] etc

### Stats
- [ ] Design the ui
- [ ] Store to json file?
- [ ] etc...