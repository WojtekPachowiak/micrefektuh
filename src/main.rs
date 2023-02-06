// heavily inspired by: https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, Device, SampleFormat
};
use ringbuf::{HeapRb, Consumer, Producer, SharedRb, producer};
use std::{sync::Arc, mem::MaybeUninit};
use console::Term;

mod effects;
use effects::{
    traits::AudioEffect,
    bitcrush::Bitcrush
};

const LATENCY: f32 = 50.;

#[allow(dead_code)]
fn print_devices_info(){
    println!("Available hosts: {:?}", cpal::available_hosts());

    let host = cpal::default_host();
    println!("Host: {:?}", host.id());

    println!("");

    let devices = host. devices().unwrap();
    println!("All devices:");
    println!("{:?}", devices.map(|x| x.name().unwrap()).collect::<Vec<String>>() );

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
    
    println!("");

    println!("Default output device: {:?}", host.default_output_device().unwrap().name().unwrap());
    println!("Default input device: {:?}", host.default_input_device().unwrap().name().unwrap());

}


fn setup_audio_devices() -> (Device, Device, cpal::StreamConfig) {
    let host = cpal::default_host();
    let odevice = host.default_output_device().unwrap();
    let idevice = host.default_input_device().unwrap();
    let config : cpal::StreamConfig = idevice.default_input_config().unwrap().into();
    (odevice, idevice, config)
}

// Create a delay in case the input and output devices aren't synced.
fn calc_latency_samples(config: &cpal::StreamConfig) -> usize {
    let latency_frames = (LATENCY / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;
    latency_samples
}

type ProducerT = Producer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;
type ConsumerT = Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;
fn setup_ring_buffer(latency_samples : usize) -> (ProducerT, ConsumerT) {
    let ring = HeapRb::<f32>::new(latency_samples * 2);
    let (mut producer, consumer) = ring.split();
    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }
    (producer, consumer)
}


fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}

// fn create_input_data_fn(producer: &mut ProducerT) -> impl FnMut(&[f32], &cpal::InputCallbackInfo) + Send{
//     let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
//         let mut output_fell_behind = false;
//         // println!("INPUT len data: {:?}", data.len());
//         for &sample in data {
//             if producer.push(sample).is_err() {
//                 output_fell_behind = true;
//             }
//         };
//         if output_fell_behind {
//             eprintln!("output stream fell behind: try increasing latency");
//         }
//     };
//     input_data_fn
// }

fn input_data_processing(producer: &mut ProducerT, data: &[f32]){
    let mut output_fell_behind = false;
    // println!("INPUT len data: {:?}", data.len());
    for &sample in data {
        if producer.push(sample).is_err() {
            output_fell_behind = true;
        }
    };
    if output_fell_behind {
        eprintln!("output stream fell behind: try increasing latency");
    }
}
fn create_input_stream(idevice: &Device, config: &cpal::StreamConfig, producer: ProducerT) -> cpal::Stream {
    let mut producer = producer;
    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        };
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let input_stream = idevice.build_input_stream(
        &config, 
        input_data_fn, 
        err_fn
    ).unwrap();
    input_stream
}

fn create_output_stream(odevice: &Device, config: &cpal::StreamConfig, consumer: ConsumerT) -> cpal::Stream {    
    let bitcrush = Bitcrush::new(32, 1);

    let mut consumer = consumer;
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        // println!("OUTPUT len data: {:?}", data.len());
        for sample in data {
            // println!("OUTPUT sample: {}", sample);
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
            // *sample = bitcrush.process_sample(*sample);
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };
    let output_stream = odevice.build_output_stream(
        &config, 
        output_data_fn, 
        err_fn
    ).unwrap();
    output_stream
}



fn main() {

    let (odevice, idevice, config) = setup_audio_devices();
    let latency_samples = calc_latency_samples(&config); 
    let (producer, consumer) = setup_ring_buffer(latency_samples);

    // Build streams.
    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = create_input_stream(&idevice, &config, producer);
    let output_stream = create_output_stream(&odevice, &config, consumer);
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        LATENCY
    );
    input_stream.play().unwrap();
    output_stream.play().unwrap();


    //wait for a keypress
    let stdout = Term::buffered_stdout();
    loop{
        if let Ok(char) = stdout.read_char(){
            println!("You typed: {}", char);
            drop(input_stream);
            drop(output_stream);
            println!("Streams stopped.");
            println!("Goodbye.");
            break;
        }
    }
}
