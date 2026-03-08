mod audio;

use nannou::prelude::*;
use ringbuf::traits::Consumer;

struct Model {
    _stream: cpal::Stream,
    consumer: ringbuf::HeapCons<f32>,
    sample_rate: u32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Synesthetic Visualizer")
        .size(1280, 720)
        .view(view)
        .build()
        .unwrap();

    let capture = audio::start_capture();

    Model {
        _stream: capture.stream,
        consumer: capture.consumer,
        sample_rate: capture.sample_rate,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Drain the ring buffer so it doesn't fill up and stall the audio callback.
    // Phase 2 replaces this with the DSP thread consuming via ringbuf::Consumer.
    let mut scratch = [0.0f32; 1024];
    let mut total = 0usize;
    loop {
        let n = model.consumer.pop_slice(&mut scratch);
        if n == 0 {
            break;
        }
        total += n;
    }
    if total > 0 {
        let ms = total as f64 / model.sample_rate as f64 * 1000.0;
        eprintln!("audio: drained {total} samples ({ms:.1} ms @ {}Hz)", model.sample_rate);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
