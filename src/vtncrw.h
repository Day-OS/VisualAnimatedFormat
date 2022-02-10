#pragma once
#include "vector"

typedef unsigned char u8c;
typedef unsigned char u8p;
typedef unsigned short int u16c;
#define U8Max 0xFF
#define U16Max 0xFFFF
#define U24Max 0xFFFFFF

class Resolution{
    public:
    u16c x = 1;
    u16c y = 1;
};
class RGBA{
    public:
    u8c R;
    u8c G;
    u8c B;
    u8c A;
};
class Frame {
    public:
    u8c Pixels [U8Max];
    u16c msDuration;
};
class Layer{
    public:
    u8c layerKey;
    Frame framesArray [U8Max];
};

class VTNCFile
{
public:
    bool isFile = false;
    u8c layersQuantity;
    Resolution layersResolution[U8Max];
    u8c colorsQuantity;
    RGBA Colors [U8Max];
    u8c framesQuantity;
    Layer *Layers;
};

class VTNCRW
{
private:
    u8c _TAGNeeded [5] = "VTNC";
public:
    VTNCRW();
    VTNCFile read(std::vector<unsigned char> file);
    std::vector<unsigned char> write(VTNCFile file);
};
