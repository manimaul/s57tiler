#pragma once

#include <ogrsf_frmts.h>

class S57Layer {

private:
    std::shared_ptr<GDALDataset> parentDs;
    OGRLayer *layer; //non-allocated ptr owned by parentDs

public:
    S57Layer(std::shared_ptr<GDALDataset> parent, OGRLayer *layer);
    std::string name();
};
