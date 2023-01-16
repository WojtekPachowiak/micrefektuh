import pyaudio
import rich
from rich.text import Text
from rich.live import Live
import time
import numpy as np
import pedalboard
import dearpygui.dearpygui as dpg
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
CHUNK = 1024
FORMAT = pyaudio.paInt16
CHANNELS = 1
RATE = 44100
RECORD_SECONDS = 5
WAVE_OUTPUT_FILENAME = "output.wav"

p = pyaudio.PyAudio()

in_stream = p.open(format=FORMAT,
                   channels=CHANNELS,
                   rate=RATE,
                   input=True,
                   frames_per_buffer=CHUNK)

out_stream = p.open(format=FORMAT,
                    channels=CHANNELS,
                    rate=RATE,
                    output=True)


########################################################


history =  np.zeros((CHUNK*10,CHANNELS))
fig,ax = plt.subplots(figsize=(8,4))
lines = ax.plot(history,color = (0,1,0.29))

def update_plot(frame):
    global history
    data = in_stream.read(CHUNK)
    shift = len(data)
    history = np.roll(history, -shift, axis = 0)
    # Elements that roll beyond the last position are 
    # re-introduced 
    history[-shift:,:] = data
    out_stream.write(data)
    for col, line in enumerate(lines):
        line.set_ydata(data[:,col])
    return lines    

ax.set_facecolor((0,0,0))
# Lets add the grid
ax.set_yticks([0])
ax.yaxis.grid(True)

ani  = FuncAnimation(fig, update_plot, interval=20,blit=True)
with True:
	plt.show()








# try: 
#     while True:
#         history = in_stream.read(CHUNK)
#         np_data = np.frombuffer(history, dtype=np.int16)

#         out_stream.write(history)
# except KeyboardInterrupt:
#     pass





# with Live(Text("start"), refresh_per_second=4) as live:  # update 4 times a second to feel fluid
#     while True:
#         history = in_stream.read(CHUNK)
#         max_amp = np.amax(np.frombuffer(history, dtype=np.int16))
#         live.update(Text(
#             f"[{ int(min(1,(max_amp/5000))*50)* '|' }"
#             ))
#         live.update(Text(
#             f"[{max_amp}"
#             ))
#         out_stream.write(history)

# print("* stopped recording")


# in_stream.close()
# out_stream.close()
# p.terminate()
