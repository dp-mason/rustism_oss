#pragma once

#include <stdlib.h>
#include <string.hpp>
#include "../../extism/runtime/extism.h"

// TODO: pass the number of samples to compute as an arg
#pragma pack(1)
struct ExtismArgs
{
	float sample_time;
    int num_voices;
    float freq_hz_0;
    float freq_hz_1;
    float freq_hz_2;
    float freq_hz_3;
	float input_0;
	float input_1;
    float input_2;
	float input_3;
    float input_4;
	float input_5;
};

ExtismPlugin* LoadExtismPlugin(std::string, bool);

float* ComputeAudioSamples(
    ExtismPlugin *plugin,
    float sample_time,
    int num_voices,
    float freq_hz[4],
    float inputs[6]=nullptr,
    size_t num_samples=256);

// void ComputeAudioSamplesPolyphonic(
//     ExtismPlugin* plugin,
//     float sample_time,
//     float freq_hz[4],
//     float poly_output[4][256], // TODO: only 256 is supported due to coordination with wasm modules
//     float inputs[6]);