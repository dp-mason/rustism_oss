#include "wasm_synth.hpp"

ExtismPlugin* LoadExtismPlugin(std::string path, bool is_url)
{
    std::string man_str;
    
    if(is_url){
        man_str = std::string("{\"wasm\": [{\"url\": \"" + path + "\"}]}");
    }
    else{
        man_str = std::string("{\"wasm\": [{\"path\":\"" + path + "\"}]}");
    }

    const char *manifest = man_str.c_str();
    char *errmsg = nullptr;

    ExtismPlugin *plugin = extism_plugin_new((const uint8_t *)manifest, strlen(manifest), NULL, 0, true, &errmsg);
    
    if(errmsg){
        DEBUG("%s", errmsg);
        return nullptr;
    }
    
    if(!plugin) return nullptr;

    return plugin;
}

float* ComputeAudioSamples(
    ExtismPlugin *plugin,
    float sample_time,
    int num_voices,
    float freq_hz[4],
    float inputs[6],
    size_t num_samples // TODO: only 256 is supported due to coordination with plugin modules
    )
{
    if (plugin == NULL){
        DEBUG("ERROR: NULL PLUGIN\n");
        return nullptr;
    }

    ExtismArgs args = ExtismArgs{
        sample_time,
        num_voices,
        freq_hz[0],
        freq_hz[1],
        freq_hz[2],
        freq_hz[3],
        inputs ? inputs[0] : 1.0f,
        inputs ? inputs[1] : 1.0f,
        inputs ? inputs[2] : 1.0f,
        inputs ? inputs[3] : 1.0f,
        inputs ? inputs[4] : 1.0f,
        inputs ? inputs[5] : 1.0f
    };

    int rc = extism_plugin_call(plugin, "batch_compute_wf", (const uint8_t*)&args, sizeof(ExtismArgs));
    
    if (rc != EXTISM_SUCCESS) {
        DEBUG("EXTISM PLUGIN CALL FAILURE: %s", extism_plugin_error(plugin));
        return nullptr;
    }

    return (float*)extism_plugin_output_data(plugin);
}

// // TODO: Compute Polyphonic output that either makes several calls to ComputeAudioSamplesMonophonic
// //  and copies memory out each time or does something more sophisticted on the wasm side
// //  Maybe the template programs could include boilerplate for returning a matrix representing polyphonic output?
// void ComputeAudioSamplesPolyphonic(
//     ExtismPlugin* plugin,
//     float sample_time,
//     float freq_hz[4],
//     float poly_output[4][256], // TODO: only 256 is supported due to coordination with wasm modules
//     float inputs[6]=nullptr
//     )
// {
//     for(int i=0; i<4; i++){
//         float *voice = ComputeAudioSamplesMonophonic(
//             plugin,
//             sample_time,
//             freq_hz[i],
//             inputs,
//             256 // TODO: only 256 is supported due to coordination with wasm modules
//         );
//         memcpy(voice, poly_output[i], sizeof(float) * 256);
//     }

//     return;
// }
