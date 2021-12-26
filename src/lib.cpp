#include "lib.h"
#include "iostream"
#include "vector"
#include "string.h"



VTNCRW::VTNCRW(/* args */) 
{
    this->a = 's';
    std::cout << "I'M LOADED!" << std::endl;
}

 void readChunk(std::vector<unsigned char> file, int &lastaddress, int bytesize, unsigned char *output, bool isString = false){
     
    for (size_t i = 0; i < bytesize; i++)
    { 
        std::cout << std::hex << i  << " (" << int(*(output + i)) << ") |";
        memset(output + i, file[lastaddress + i], 1);
        std::cout << std::hex << i  << " (" << int(*(output + i)) << ") |" << std::endl;
    }
    if (isString)
    {
        lastaddress -= 1;
        memset(output + bytesize-1, 0, 1);
    }
    std::cout << std::endl << int(output) << std::endl ;
    lastaddress += bytesize;
    return;
}

VTNCFile VTNCRW::read(std::vector<unsigned char> file)
{
    VTNCFile output;
    u8c _TAGNeeded [5] = "VTNC";
    u8c TAG [5] = {0};
    output.layersQuantity;
    output.layerKeys[U8Max];
    output.layersResolution[U8Max];
    output.colorsQuantity;
    output.Colors [U8Max];
    output.framesQuantity;
    output.framesArray [U8Max];

    int size = 4;
    int blockOffset = 0;

    
    readChunk(file, blockOffset, 5, TAG, true);
    readChunk(file, blockOffset, 1, &output.layersQuantity);

    std::cout << std::endl << int(output.layersQuantity) << std::endl ;
    //readChunk(file, blockOffset, 1, &output.layersQuantity, blockOffset);
    /*
    for (size_t i = 0; i < 4; i++)  
    { 
        TAG[i] = file[i];
        //std::cout << std::hex << int(file[i]) << "|";
    }
    */
    if (memcmp(TAG, _TAGNeeded, 5) == 0)
    {
        output.isFile = true;
    }
    
   
    /*
    
    */
    
    return output;
}
