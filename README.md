# fixed-audio

Are you annoyed that the microphone of your mac automatically switches to your Airpods whenever you wear them even though you have another microphone
you rather want to use? Why? Cause using Airpods for in AND output will destroy the quality cause it switches to headset mode.

## What does this binary do?

This is a deamon. On startup it checks what your current microphone is and makes sure that macOS will never change it. It listens for audio events and
switches back to this audio device whenever it changes. No worries. It doesn't poll and eat your CPU.
