#pragma once
#include "vector"


typedef unsigned char u8c;
typedef unsigned char u8p;
typedef unsigned short int u16c;
#define U8Max 0xFF
#define U16Max 0xFFFF
#define U24Max 0xFFFFFF

#define LINEUP "\033[F"
#define RESET   "\033[0m"
#define BLACK   "\033[30m"      /* Black */
#define RED     "\033[31m"      /* Red */
#define GREEN   "\033[32m"      /* Green */
#define YELLOW  "\033[33m"      /* Yellow */
#define BLUE    "\033[34m"      /* Blue */
#define MAGENTA "\033[35m"      /* Magenta */
#define CYAN    "\033[36m"      /* Cyan */
#define WHITE   "\033[37m"      /* White */
#define BOLDBLACK   "\033[1m\033[30m"      /* Bold Black */
#define BOLDRED     "\033[1m\033[31m"      /* Bold Red */
#define BOLDGREEN   "\033[1m\033[32m"      /* Bold Green */
#define BOLDYELLOW  "\033[1m\033[33m"      /* Bold Yellow */
#define BOLDBLUE    "\033[1m\033[34m"      /* Bold Blue */
#define BOLDMAGENTA "\033[1m\033[35m"      /* Bold Magenta */
#define BOLDCYAN    "\033[1m\033[36m"      /* Bold Cyan */
#define BOLDWHITE   "\033[1m\033[37m"      /* Bold White */


class Resolution{
    public:
    u16c x = 1;
    u16c y = 1;
};
class RGB{
    public:
    u8c R;
    u8c G;
    u8c B;
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
    RGB Colors [U8Max];
    u8c framesQuantity;
    Layer *Layers;
    /*The first index being the layer and the other the frame itself*/
    

};

class VTNCRW
{
private:

public:
    VTNCRW();
    VTNCFile read(std::vector<unsigned char> file);
    int write(VTNCFile file);
};
