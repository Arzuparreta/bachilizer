use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::traits::{Observer, Producer, Split};
use ringbuf::{HeapCons, HeapRb};

const RING_BUFFER_CAPACITY: usize = 8192;

pub struct AudioCapture {
    pub stream: cpal::Stream,
    pub consumer: HeapCons<f32>,
    pub sample_rate: u32,
}

pub fn start_capture() -> AudioCapture {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no input device available");

    eprintln!("Audio device: {}", device.name().unwrap_or_default());

    let supported = device
        .default_input_config()
        .expect("no default input config");

    let sample_rate = supported.sample_rate().0;
    let channels = supported.channels() as usize;
    eprintln!("Audio config: {sample_rate}Hz, {channels} ch");

    let config: cpal::StreamConfig = supported.into();
    let (mut producer, consumer) = HeapRb::<f32>::new(RING_BUFFER_CAPACITY).split();

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if channels == 1 {
                    // Mono: push the entire slice if there is room.
                    // If the ring is full, drop the ENTIRE callback to avoid
                    // partial-slice phase misalignment in the DSP accumulator.
                    if producer.vacant_len() >= data.len() {
                        producer.push_slice(data);
                    }
                } else {
                    // Multi-channel: extract first channel only (zero-alloc).
                    for chunk in data.chunks_exact(channels) {
                        if producer.vacant_len() < 1 {
                            break;
                        }
                        let _ = producer.try_push(chunk[0]);
                    }
                }
            },
            |err| eprintln!("audio error: {err}"),
            None,
        )
        .expect("failed to build input stream");

    stream.play().expect("failed to start audio stream");

    AudioCapture {
        stream,
        consumer,
        sample_rate,
    }
}
