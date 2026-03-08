A real-time audio visualizer that maps frequency content onto a three-dimensional Tonnetz spiral. Harmonic and combination-tone relationships are rendered as additive Gaussian splats; the result is a fluid, perceptually grounded representation of spectral structure without post-processing.

The explanation for musicians is that it visualizes harmonic relationship and movement between frequencies, rather than the regular frequency map. The main goal of this is bringing to a visualizer not just the shape of music, but it's meaning, movement and form.

## Architecture

- **Audio thread:** Lock-free capture (cpal) into a ring buffer; no allocations, no mutexes. Support for system loopback (WASAPI, Pulse/PipeWire monitor) and microphone.
- **DSP thread:** Sliding-window FFT (Hanning, 4096 bins), magnitude spectrum, peak detection, and Tartini (sum/difference) tone derivation. Frames are sent by value over a bounded channel.
- **Main thread:** EMA-smoothed magnitudes and Tartini data are uploaded to the GPU; WGSL vertex shaders place billboard quads on a logarithmic spiral (pitch class vs. log frequency) and apply Gaussian falloff in the fragment stage. Additive blending yields a soft, glowing field.

## Stack

Rust; cpal, rustfft, ringbuf, crossbeam, nannou (wgpu). Single-pass, instanced rendering; no bloom pipeline.

## Build and run

```bash
cargo run
```

Default window: 1280×720. Requires an audio input (loopback or microphone).
