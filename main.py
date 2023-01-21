import pyaudio
import rich
from rich.text import Text
from rich.live import Live
import time
import numpy as np
from pedalboard import Pedalboard, Chorus, Reverb, Delay
import dearpygui.dearpygui as dpg
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import matplotlib


CHUNK = 1024
FORMAT = pyaudio.paFloat32
CHANNELS = 1
RATE = 44100
RECORD_SECONDS = 5
WAVE_OUTPUT_FILENAME = "output.wav"

matplotlib.rcParams['toolbar'] = 'None' 


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

board = Pedalboard([Delay(), Reverb(room_size=0.25)])

#######################################################
# zoom = 10

# history =  np.zeros((CHUNK*5,CHANNELS))
# fig,ax = plt.subplots(figsize=(8,4))
# lines = ax.plot(history, color = (0,1,0.29))

# 


# def update_plot(frame):
#     global history
#     data = in_stream.read(CHUNK)
#     data = np.frombuffer(data, dtype=np.float32).reshape(-1,1)

#     shift = len(data)
#     history = np.roll(history, -shift, axis = 0) 
#     history[-shift:,:] = data

#     data = board(data, RATE, reset=False)
#     data = data.tobytes()
#     out_stream.write(data)


#     for col, line in enumerate(lines):
#         line.set_ydata(history[:,col])
#     return lines  


# # def update_plot_mean(frame):
# #     global history
# #     data = in_stream.read(CHUNK)
# #     data = np.frombuffer(data, dtype=np.int16).reshape(-1,1)
    
# #     history = np.roll(history, -1, axis = 0) 
# #     history[-1:,:] = np.mean(data)
# #     # out_stream.write(data)
# #     for col, line in enumerate(lines):
# #         line.set_ydata(history[:,col])
# #     return lines      



# def on_scroll(event):
#     'scale the plot on scroll'

#     print(event.button, event.step)
    
#     global zoom
    
#     zoom += event.step 
#     zoom = np.clip(zoom, 7,15)
#     val =  np.power(2, zoom)
#     ax.set_ylim(-val, val)
#     ax.set_yticks([-val,0,val])
#     fig.canvas.draw_idle()    

# ax.set_facecolor((0,0,0))
# # Lets add the grid
# ax.set_xticks([])
# ax.set_ylim(-1000, 1000)

# fig.canvas.mpl_connect('scroll_event', on_scroll)

# ani  = FuncAnimation(fig, update_plot, interval=30, blit=True)
# # while True:
# # 	plt.show()
# plt.show()


########################################################




try: 
    with Live(Text("start"), refresh_per_second=4) as live:  # update 4 times a second to feel fluid
        while True:
            now = time.time()
            data = in_stream.read(CHUNK)
            data = np.frombuffer(data, dtype=np.float32)
            data = board(data, RATE, reset=False)
            data = data.tobytes()
            out_stream.write(data)
            # live.update(Text(f"{str((time.time()-now)*1000)} / {1/RATE*1000}"))
except KeyboardInterrupt:
    pass





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
