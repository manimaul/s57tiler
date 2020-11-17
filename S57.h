#pragma once

#include <string>
#include <gdal_priv.h>
#include "S57Layer.h"

class S57 {
private:
    std::shared_ptr<GDALDataset> dataset;
    std::vector<S57Layer> layers {};
    static GDALDataset *initialize(std::string &path) ;
    const std::vector<S57Layer> &getLayers();

public:
    S57(std::string path);
    bool renderLayersGeojson(const std::string& outPath);
};
