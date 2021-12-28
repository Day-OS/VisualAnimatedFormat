#include "lib.h"
#include "iostream"
#include "vector"
#include "string.h"



VTNCRW::VTNCRW() 
{
    std::cout << GREEN << "VTNCRW Class CREATED." << RESET << std::endl;
}

void readChunk(std::vector<unsigned char> file, int &lastaddress, int bytesize, unsigned char *output, bool isString = false){
     
    for (size_t i = 0; i < bytesize; i++)
    {
        memset(output + i, file[lastaddress + i], 1);
    }
    if (isString)
    {
        lastaddress -= 1;
        memset(output + bytesize-1, 0, 1);
    }
    lastaddress += bytesize;
    return;
}
void readChunk16u(std::vector<unsigned char> file, int &lastaddress, u16c *output){
    unsigned char buf[2];
    readChunk(file, lastaddress, 2, buf);
    *output = (buf[1] << 8) + buf[0];
}
VTNCFile VTNCRW::read(std::vector<unsigned char> file)
{
    VTNCFile output;
    u8c _TAGNeeded [5] = "VTNC";
    u8c TAG [5] = {0};
    output.layersQuantity;
    u8c layerKeys[U8Max];
    output.layersResolution[U8Max];
    output.colorsQuantity;
    output.Colors [U8Max];
    output.framesQuantity;

    int blockOffset = 0;

    
    readChunk(file, blockOffset, 5, TAG, true); if (memcmp(TAG, _TAGNeeded, 5) == 0) output.isFile = true;
    readChunk(file, blockOffset, 1, &output.layersQuantity);
    for (size_t i = 0; i < output.layersQuantity; i++){readChunk(file, blockOffset, 1, &layerKeys[i]);}
    for (size_t i = 0; i < output.layersQuantity; i++){
        Resolution res;
        readChunk16u(file, blockOffset, &res.x);
        readChunk16u(file, blockOffset, &res.y);
        output.layersResolution[i] = res;
    }
    readChunk(file, blockOffset, 1, &output.colorsQuantity);
    for (size_t i = 0; i < output.colorsQuantity; i++){
        u8c address = 0;
        RGB temp;
        readChunk(file, blockOffset, 1, &address);
        readChunk(file, blockOffset, 1, &temp.R);
        readChunk(file, blockOffset, 1, &temp.G);
        readChunk(file, blockOffset, 1, &temp.B);
        output.Colors[address] = temp;
    }
    readChunk(file, blockOffset, 1, &output.framesQuantity);
    
    Layer layers[output.layersQuantity];

   
    
    for (size_t layerindex = 0; layerindex < output.layersQuantity; layerindex++)
    {
        Layer currentLayer;
        for (size_t frameindex = 0; layerindex < output.framesQuantity; layerindex++)
        {
            Frame currentFrame;
            for (size_t i = 0; i < output.layersResolution[layerindex].x * output.layersResolution[layerindex].y; i++)
            {
                
                readChunk(file, blockOffset, 1, &currentFrame.Pixels[i]);
                std::cout << "| addr: " << int(currentFrame.Pixels[i]);
            }
            
            //readChunk16u(file,blockOffset,&currentFrame.msDuration);
            currentLayer.layerKey = layerKeys[layerindex];
            layers[layerindex] = currentLayer;
            //THIS STUFF CRASHES AFTER THIS POINT!!
        }
    }
    output.Layers = layers;
    
    return output;
}
