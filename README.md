# Watchdog

An application to ensure that your interactive installation software (ahem, TouchDesigner application) is running 24/7.

Of course, your deployed application isnot going to crash and leave a giant LED wall empty or frozen... but, wouldnt you sleep better knowing that even if it _did_, it would automatically be restarted, and you receive an alert, before your client calls to tell you its all broken...?

pm2 is a great software to persist many types of applications, but I have been unable to integrate it in a clean, successful, and controlable way with TouchDesigner.

This is a work in progress, is a rewrite of a quick and dirty version built in Node.js, and is a project for me to learn Rust.

Further explanation and documentation to come.

## TO DO:

### Config
[ ] Email client UI spruce up
[ ] Email client password encrypt/decrypt
[ ] Email custom types with validation
[ ] etc

### Apps
[ ] Launch apps lol
[ ] Listen for osc
[ ] Relaunch apps
[ ] Kill apps
[ ] Design ui
[ ] etc

### Stats
[ ] Design the ui
[ ] Store to json file?
[ ] etc...