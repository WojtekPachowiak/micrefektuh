# micrefektuh
super crazy library for applying real-time audio effects over microphone input

### TODO
- switching from Python to Rust for speed
- planning to implement guitar pedals and other audio effects. While modifying single input numbers is easy, I don't know how the effects requiring some buffer (ex. Delay) should be implemented. 
- Effects that I can implement by modyfing single input numbers: Distortion, Bitcrush, adding noise (white or some other)



### Sources of inspiration
- https://wiki.analog.com/resources/tools-software/sharc-audio-module/baremetal/using-both-cores
- (delay) https://wiki.analog.com/resources/tools-software/sharc-audio-module/baremetal/delay-effect-tutorial
- (mic input, speaker output; Rust cpal example)https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs
- (bit crush, sample rate reducer) http://10rem.net/blog/2013/01/13/a-simple-bitcrusher-and-sample-rate-reducer-in-cplusplus-for-a-windows-store-app
- (FFT in Rust) https://phip1611.de/blog/frequency-spectrum-analysis-with-fft-in-rust/
- add plots for debugging/visualization + freq spectrum analysis
