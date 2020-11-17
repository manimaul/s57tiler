//
// Created by William Kamp on 11/15/20.
//

#include "S57.h"
#include <iostream>
#include <filesystem>
#include <cstdio>
#include <unistd.h>

using namespace std;
namespace fs = filesystem;

GDALDataset* S57::initialize(string &path) {
    cout << "opening s57 file: " << path << endl;
    return (GDALDataset *) GDALOpenEx(
            path.c_str(),
            GDAL_OF_VECTOR,
            nullptr,
            nullptr,
            nullptr
    );
}

S57::S57(string path) : dataset(initialize(path)) {}

bool S57::renderLayersGeojson(const string& outPath) {

    fs::path p = outPath;
    // use the cwd if the path is not absolute
    if (!p.is_absolute()) {
        char workDir[FILENAME_MAX];
        if (!getcwd(workDir, sizeof(workDir))) {
            return false;
        }
        string w = workDir;
        p = w.append("/").append(outPath);
    }

    if (!fs::is_directory(p) && fs::exists(p)) {
        cout << p << "exists and is not a directory!" << endl;
        return false;
    }

    if (!fs::exists(p)) {
        cout << "creating output directory:" << p << endl;
        fs::create_directories(p);
    }

    cout << "output directory is set to  " << p << endl;

    for (auto layer : getLayers()) {
        //todo: (WK) render layers here
        cout << layer.name() << endl;
    }
    return true;
}

const vector<S57Layer> &S57::getLayers() {
    if (nullptr != dataset && layers.empty()) {
        for (int i = 0; i < dataset->GetLayerCount(); ++i) {
            S57Layer layer = S57Layer{dataset, dataset->GetLayer(i)};
            layers.emplace_back(move(layer));
        }
    }
    return layers;
}
