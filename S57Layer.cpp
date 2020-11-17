//
// Created by William Kamp on 11/15/20.
//

#include "S57Layer.h"
#include <utility>

S57Layer::S57Layer(std::shared_ptr<GDALDataset> parent, OGRLayer *layer) : parentDs(std::move(parent)), layer(layer) {}

std::string S57Layer::name() {
    return layer->GetName();
}
