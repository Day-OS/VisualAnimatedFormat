#include "vtncrw.h"
#include "iostream"
#include "vector"
#include "string.h"

VTNCRW::VTNCRW(){}

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
        RGBA temp;
        readChunk(file, blockOffset, 1, &address);
        readChunk(file, blockOffset, 1, &temp.R);
        readChunk(file, blockOffset, 1, &temp.G);
        readChunk(file, blockOffset, 1, &temp.B);
        readChunk(file, blockOffset, 1, &temp.A);
        output.Colors[address] = temp;
    }
    readChunk(file, blockOffset, 1, &output.framesQuantity);
    
    Layer layers[output.layersQuantity];

   
    
    for (size_t layeri = 0; layeri < output.layersQuantity; layeri++)
    {
        Layer currentLayer;
        for (size_t framei = 0; framei < output.framesQuantity; framei++)
        {
            Frame currentFrame;
            for (size_t i = 0; i < output.layersResolution[layeri].x * output.layersResolution[layeri].y; i++)
            {
                
                readChunk(file, blockOffset, 1, &currentFrame.Pixels[i]);
            }
            
            readChunk16u(file,blockOffset,&currentFrame.msDuration);
            currentLayer.framesArray[framei] = currentFrame;
            currentLayer.layerKey = layerKeys[layeri];
            layers[layeri] = currentLayer;
        }
    }
    output.Layers = layers;
    return output;
}

std::vector<unsigned char> VTNCRW::write(VTNCFile file) 
{
    std::vector<unsigned char> buffer;
    VTNCFile buffergenerated;

    for (u8c i = 0; i < (sizeof(_TAGNeeded) - 1); i++)
    {
        buffer.push_back(_TAGNeeded[i]);
    }
    buffer.push_back(file.layersQuantity);
    for (u8c i = 0; i < file.layersQuantity; i++)
    {
        buffer.push_back(file.Layers[i].layerKey);
    }
    for (u8c i = 0; i < file.layersQuantity; i++)
    {
        buffer.push_back(file.layersResolution[i].x);
        buffer.push_back((file.layersResolution[i].x<<8));
        buffer.push_back(file.layersResolution[i].y);
        buffer.push_back((file.layersResolution[i].y<<8));
    }
    buffer.push_back(file.colorsQuantity);
    for (u8c i = 0; i < file.colorsQuantity; i++)
    {
        buffer.push_back(i);
        buffer.push_back(file.Colors[i].R);
        buffer.push_back(file.Colors[i].G);
        buffer.push_back(file.Colors[i].B);
        buffer.push_back(file.Colors[i].A);
    }
    buffer.push_back(file.framesQuantity);
    for (u8c i_layer = 0; i_layer < file.layersQuantity; i_layer++)
    {
        for (u8c i_frame = 0; i_frame < file.framesQuantity; i_frame++)
        {   
            Frame currentframe = file.Layers[i_layer].framesArray[i_frame];
            for (u8c i = 0; i < file.layersResolution[i_layer].x * file.layersResolution[i_layer].y; i++)
            {
                buffer.push_back(currentframe.Pixels[i]);   
            }
            buffer.push_back(currentframe.msDuration);
            buffer.push_back(currentframe.msDuration << 8);
        }
    }
    
    return buffer;
}
