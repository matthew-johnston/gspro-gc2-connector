# Overview

Disclaimer - I wrote this for myself, so it's a little rough around the edges. Sharing it wider in case it helps someone else.

This is solving a very specific issue with connecting GSPro to a Foresight GC2 launch monitor across rooms.

-  Gaming PC in the house running GSPro and Sunshine game streaming software - https://github.com/LizardByte/Sunshine
-  Sim PC in the garage (connected to the projector), running Moonlight - https://github.com/moonlight-stream

I needed to send the GC2 launch monitor data from the garage to the Gaming PC (out of bluetooth range).

What this does:

-  Runs on the Sim PC in the garage - connects to GC2 launch monitor over bluetooth.
-  Connects to GSPro (on the gaming pc) using their open api (enter the ip address of your gaming pc) - https://gsprogolf.com/GSProConnectV1.html
-  Sends the ball events through

# Quickstart

Start GSPro (on your gaming PC) and your GC2 launch monitor.

_Run this on your Sim PC_ Replace 127.0.0.1 with the ip address of your Gaming PC. Find it by typing `ipconfig` into powershell (or a command window)
The GC2 will be sending data on `com3` or `com4`
`.\gspro-gc2-connector.exe --com-port com4 --gs-pro-ip 127.0.0.1`

A better approach - after trying the above and making sure it connects is to create a shortcut to the exe. You can add params to the shortcut.
Right click on the shortcut - `Properties` add ` --com-port com4 --gs-pro-ip 127.0.0.1` (Your settings) to the end of the target line.

# For developers

`cargo run -- --com-port com4 --gs-pro-ip 127.0.0.1`

### Example event from the GC2 launch monitor

`CT=1259299,SN=2638,HW=3,SW=4.0.0,ID=2,TM=1259299,SP=8.39,AZ=-6.08,EL=18.88,TS=800.00,SS=-125.00,BS=790.00,CY=0.95,TL=0.95,SM=0.00,HMT=0`
