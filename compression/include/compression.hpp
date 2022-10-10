class Compressor {
public:
    unsigned int field_0[8192];
    unsigned char gap8000[9220];
    int some_buf5[8192];
    int some_buf6[8192];
    int some_buf7[8192];
    int some_buf[8192];
    int some_buf2[8192];
    unsigned int gap32404[100];
    int field_32594;
    unsigned char gap32598[32364];
    int initially_256;
    int some_buf_index_initially_0x2000;
    int initially_0x2000_2;
    int shift_subtraction_initially_1;
    int maybe_some_bit_initially_2;
    unsigned int most_recently_acquired_src_byte;
    int amount_to_shift_by;
    int initially_8;
    int initially_0;
    unsigned int src_size;
    unsigned int available_dest_space;
    unsigned int src_index;
    unsigned int filled_dest_space;
    int src_word_size;
    unsigned int some_size_always_1;
    unsigned char *src;
    unsigned char *dest;
    int field_3A448;
    int field_3A44C;
    int field_3A450;
    int field_3A454;
    int field_3A458;
    int field_3A45C;
    int field_3A460;
    int field_3A464;
    int field_3A468;
    int field_3A46C;
    int field_3A470;

    int sub_10001006(unsigned int *a2, int a3, signed int a4, int a5);
    signed int InitState();
    signed int Unpack(unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size, unsigned int dw1);
    signed int GetBit();
    signed int Pack(unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size, unsigned int dw1, int bool1);
    int sub_10001BBB(signed int a2);
};

extern "C" {
    int Unpack(const unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size);
    int Pack(const unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size);
    void PackFree(unsigned char *mem);
}
