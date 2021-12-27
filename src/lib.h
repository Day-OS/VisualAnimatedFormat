#pragma once
#include "vector"


typedef unsigned char u8c;
typedef unsigned char u8p;
typedef unsigned short int u16c;
#define U8Max 0xFF
#define U16Max 0xFFFF
#define U24Max 0xFFFFFF


class Resolution{
    u16c x = 1;
    u16c y = 1;
};
class RGB{
    u8c R;
    u8c G;
    u8c B;
};
class Frame {
    u8c Pixels [U8Max];
    u16c msDuration;
};

class VTNCFile
{
public:
    bool isFile = false;
    u8c layersQuantity;
    u8c layerKeys[U8Max];
    Resolution layersResolution[U8Max];
    u8c colorsQuantity;
    RGB Colors [U8Max];
    u8c framesQuantity;
    Frame framesArray [U8Max];
};

class VTNCRW
{
private:
    /* data */
public:
    VTNCRW(/* args */);
    VTNCFile read(std::vector<unsigned char> file);
    char a;
};
