use extism_pdk::*;

// TODO: this is bad I think, will not translate well to other templates
static mut PHASE:f32 = 0.0;
static mut PHASES:[f32;4] = [0.0; 4];

const OUTBUF_SAMPLES:usize = 256;
const FLOAT32_BYTES:usize = 4;

// #[derive(serde::Deserialize)]
// #[derive(serde::Deserialize, FromBytes)]
// #[encoding(Json)]
// struct VcvInput {
//     sample_rate: f32,
//     sample_time: f32,
//     frame: i64,
//     pitch: f32,
//     phase: f32
// }

#[plugin_fn]
pub fn batch_compute_wf(input: Vec<u8>) -> FnResult<Vec<u8>> {
    let sample_time: f32 = f32::from_le_bytes(input[0..4].try_into().unwrap());
    let num:         i32 = i32::from_le_bytes(input[4..8].try_into().unwrap());
    let _freq_hz_0:   f32 = f32::from_le_bytes(input[8..12].try_into().unwrap());
    let _freq_hz_1:   f32 = f32::from_le_bytes(input[12..16].try_into().unwrap());
    let freq_hz_2:   f32 = f32::from_le_bytes(input[16..20].try_into().unwrap());
    let freq_hz_3:   f32 = f32::from_le_bytes(input[20..24].try_into().unwrap());
    let _input_0:     f32 = f32::from_le_bytes(input[24..28].try_into().unwrap());
    let _input_1:     f32 = f32::from_le_bytes(input[28..32].try_into().unwrap());
    let _input_2:     f32 = f32::from_le_bytes(input[32..36].try_into().unwrap());
    let _input_3:     f32 = f32::from_le_bytes(input[36..40].try_into().unwrap());
    let _input_4:     f32 = f32::from_le_bytes(input[40..44].try_into().unwrap());
    let _input_5:     f32 = f32::from_le_bytes(input[44..48].try_into().unwrap());

    let mut outbuf: Vec<u8> = vec![0; OUTBUF_SAMPLES * FLOAT32_BYTES];
    let num_voices:usize = num as usize;
    let freqs:Vec<f32> = vec![0.0, 1.0, freq_hz_2,freq_hz_3];

    for index in 0..OUTBUF_SAMPLES {
        // Computes the value of this individual sample and puts it where it belongs in the output buffer
        outbuf[index * FLOAT32_BYTES..index * FLOAT32_BYTES + 4].copy_from_slice(
            &pitched_sine_polyphonic(
                sample_time, 
                usize::from(num_voices),
                &freqs, 
                ).unwrap().to_le_bytes(),
        );
    }

    Ok(outbuf)
}

pub fn pitched_sine(
    time_elapsed:f32,
    freq_hz:f32,
) -> FnResult<f32> {
   
    // Accumulate the phase
    unsafe {
        // TODO: using this mutable static feels bad, is there another way?
        PHASE = (PHASE + (freq_hz * time_elapsed)).fract();
    }
    
    unsafe {
        // TODO: using this mutable static feels bad, is there another way?
        // Compute the sine output
        let sine_amplitude:f32 = f32::sin(2.0 * 3.14159 * PHASE);
        Ok(sine_amplitude)
    }
}

pub fn pitched_sine_polyphonic(
    time_elapsed:f32,
    num_voices:usize,
    freq_hz: &Vec<f32>,
    // input:   &Vec<f32>
) -> FnResult<f32> {

    let mut total_sine_amplitude:f32 = 0.0;
    // Accumulate the phase
    for voice in 0..num_voices{
        unsafe {
            // TODO: using this mutable static feels bad, is there another way?
            PHASES[voice] = (PHASES[voice] + (freq_hz[voice] * time_elapsed)).fract();
        }
        unsafe {
            // TODO: using this mutable static feels bad, is there another way?
            // Compute and aggregate the sine output
            total_sine_amplitude += f32::sin(2.0 * 3.14159 * PHASES[voice]);
        }
    }
    Ok(total_sine_amplitude / num_voices as f32)
    
}