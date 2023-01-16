import dearpygui.dearpygui as dpg
from math import sin, cos
import numpy as np
import time
dpg.create_context()


with dpg.window(label="Tutorial", tag="win",no_collapse=True,no_move=True,no_resize=True, no_scrollbar=True,no_title_bar=True, no_close=True,pos=(0,0),autosize=True,no_background=True):
    # create plot
    dpg.add_simple_plot(label="Simple Plot", min_scale=-1.0, max_scale=1.0, width=800, height=600, tag="plot")


dpg.create_viewport(title='Custom Title', width=800, height=600)
dpg.setup_dearpygui()
dpg.show_viewport()

x = np.linspace(-3.14,3.14,1000)

while dpg.is_dearpygui_running():   
    # insert here any code you would like to run in the render loop
    # you can manually stop by using stop_dearpygui()
    dpg.set_value('plot', np.sin(x+time.time()))
    dpg.render_dearpygui_frame()


dpg.destroy_context()