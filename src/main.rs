// heavily inspired by: https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, Device, SampleFormat
};
use ringbuf::HeapRb;
use std::io::stdin;


const LATENCY: f32 = 150.;


fn main() {
    println!("{:?}", cpal::available_hosts());

    let host = cpal::default_host();
    println!("Host: {:?}", host.id());

    println!("");

    let devices = host. devices().unwrap();
    println!("All devices:");
    println!("{:?}", devices.map(|x| x.name().unwrap()).collect::<Vec<String>>() );

    println!("");

    println!("Default output: {:?}", host.default_output_device().unwrap().name().unwrap());
    println!("Default input: {:?}", host.default_input_device().unwrap().name().unwrap());

    println!("");

    println!("All Input devices:");
    println!("{:?}", host.input_devices().unwrap().map(|x| x.name().unwrap()).collect::<Vec<String>>() );

    println!("All Output devices:");
    println!("{:?}", host.output_devices().unwrap().map(|x| x.name().unwrap()).collect::<Vec<String>>() );

    let odevice = host.default_output_device().unwrap();
    let oconfig = odevice.default_output_config().unwrap();
    println!("Default output config: {:?}", oconfig);

    let idevice = host.default_input_device().unwrap();
    let iconfig = idevice.default_input_config().unwrap();
    println!("Default input config: {:?}", iconfig);

    let config : cpal::StreamConfig = idevice.default_input_config().unwrap().into();


     // Create a delay in case the input and output devices aren't synced.
     let latency_frames = (LATENCY / 1_000.0) * config.sample_rate.0 as f32;
     let latency_samples = latency_frames as usize * config.channels as usize;
 
     // The buffer to share samples
     let ring = HeapRb::<f32>::new(latency_samples * 2);
     let (mut producer, mut consumer) = ring.split();
    
     // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }



    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {

            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }

    // Build streams.
    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = idevice.build_input_stream(&config, input_data_fn, err_fn).unwrap();
    let output_stream = odevice.build_output_stream(&config, output_data_fn, err_fn).unwrap();
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        LATENCY
    );
    input_stream.play().unwrap();
    output_stream.play().unwrap();


    // handle keyboard interrupt gracefully by closing cpal streams without using ctrlc package
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    println!("You typed: {}", s);
    drop(input_stream);
    drop(output_stream);
    println!("Streams stopped.");
    println!("Goodbye.");



}
