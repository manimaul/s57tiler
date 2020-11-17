//
// Created by William Kamp on 11/15/20.
//

#include <iostream>
#include "gdal.h"
#include "cpl_conv.h"
#include "S57.h"

using namespace std;

int main(int argc, char *argv[]) {
    if (argc != 3) {
        cout << "Please supply 2 arguments:" << endl
             << "1 - The S57 Chart file path." << endl
             << "2 - The output directory for the chart's geojson layers." << endl;
        return 1;
    }
    string chartPath = argv[1];
    string outDir = argv[2];

    GDALAllRegister();
    CPLSetConfigOption("OGR_S57_OPTIONS",
                       "LNAM_REFS:ON,UPDATES:ON,SPLIT_MULTIPOINT:ON,PRESERVE_EMPTY_NUMBERS:ON,RETURN_LINKAGES:ON");
    auto s57 = S57{chartPath};
    s57.renderLayersGeojson("foo/bar");
    return 0;
}
