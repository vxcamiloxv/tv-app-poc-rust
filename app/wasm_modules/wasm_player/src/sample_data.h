#ifndef SAMPLE_DATA_H_
#define SAMPLE_DATA_H_

#include <array>

#include "samsung/wasm/elementary_media_packet.h"
#include "samsung/wasm/elementary_video_track_config.h"

namespace sample_data {

// Duration of a stream.
//
// Usually it wouldn't be hardcoded but read from a stream source. Please note
// duration is applicable only in some ElementaryMediaStreamSource modes (see
// samsung::wasm::ElementaryMediaStreamSource::Mode).
extern const samsung::wasm::Seconds kStreamDuration;

// Contains video track configuration.
//
// Usually it wouldn't be hardcoded but read from a stream source.
extern const samsung::wasm::ElementaryVideoTrackConfig kVideoTrackConfig;

// Contains  Elementary Media Packets of the sample video track.
//
// Usually those wouldn't be hardcoded but either demuxed from a container or
// received from network (depending on the use case and protocol used by an
// application).
extern const std::array<samsung::wasm::ElementaryMediaPacket, 413>
    kVideoPackets;

}  // namespace sample_data

#endif  // SAMPLE_DATA_H_
