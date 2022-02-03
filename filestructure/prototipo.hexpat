struct layerkeysstruct{
	u8 extras[layerquantity];
};

struct vector2{
	u16 x [[color("00a0FF")]];
	u16 y [[color("00a0FF")]];
};

struct RGB{
	u8 ADDR [[color("FF00AF")]];
	u8 R [[color("FF0000")]];
	u8 G [[color("FF0000")]];
	u8 B [[color("FF0000")]];
};
struct layerrestruct{
	vector2 resolution[layerquantity];
};


#define CharQuantity 4
char HEADER[CharQuantity] @0x00 [[color("FFFFFF")]];;
u8 layerquantity @CharQuantity;

layerkeysstruct LayerKeys @$;
layerrestruct LayerResolution @$;
u8 PixelQuantity @$;
RGB PixelAddresses [PixelQuantity] @$;
u8 FramesQuantity @$;

struct frame{
	u8 RGBADR [LayerResolution.resolution[0].x * LayerResolution.resolution[0].y];
	u16 Miliseconds;
};

frame a[FramesQuantity] @$;

//std::print(LayerResolution.resolution[0].y);

//std::print(LayerResolution[0]]);
//frame a
//RGB c [while(std::mem::read_unsigned($, 1) != 0xFF)] @0x11;



