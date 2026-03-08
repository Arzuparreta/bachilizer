# Bachilizer

A real-time audio visualizer that maps frequency content onto a three-dimensional Tonnetz spiral. Harmonic and combination-tone relationships are rendered as additive Gaussian splats; the result is a fluid, perceptually grounded representation of spectral structure without post-processing.

The explanation for musicians is that it visualizes harmonic relationship and movement between frequencies, rather than the regular frequency map. The main goal of this is bringing to a visualizer not just the shape of music, but its meaning, movement and form.

## Architecture

- **Audio thread:** Lock-free capture (cpal) into a ring buffer; no allocations, no mutexes. On Linux, the stream is routed via `pactl move-source-output` after connection, targeting either the PipeWire monitor (loopback) or the configured default source (microphone).
- **DSP thread:** Sliding-window FFT (Hanning, 4096 bins), magnitude spectrum, peak detection, and Tartini (sum/difference) tone derivation. Frames are sent by value over a bounded channel; no heap allocation in the hot loop.
- **Main thread:** EMA-smoothed magnitudes pass through a peak-tracking auto-gain stage before upload. WGSL vertex shaders place billboard quads on a logarithmic spiral (pitch class vs. log frequency) and apply Gaussian falloff in the fragment stage. Additive blending yields a soft, glowing field without a post-processing pass.

Camera control is orbital (left-drag to rotate, scroll to zoom); Tartini combination tones are drawn with a distinct palette and a time-based pulse.

## Stack

Rust; cpal, rustfft, ringbuf, crossbeam, nannou (wgpu). Single-pass, instanced rendering.

## Usage

```bash
cargo run              # system audio loopback (default)
cargo run -- --mic     # microphone input
```

Default window: 1280×720. On Linux, `pactl` must be available (PulseAudio or PipeWire with the PulseAudio compatibility layer). On Windows, WASAPI loopback is used automatically.
