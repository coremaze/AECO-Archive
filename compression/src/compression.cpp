#include "compression.hpp"
#include <stdlib.h>

int Compressor::sub_10001006(unsigned int *a2, int a3, signed int a4, int a5) {
    Compressor *v5;    // edi
    int result;        // eax
    signed int v7;     // ecx
    int i;             // edx
    signed int v9;     // ebp
    int v10;           // ebx
    unsigned int *v11; // esi
    unsigned int v12;  // ebx
    unsigned int v13;  // esi
    unsigned int v14;  // ebx
    int v15;           // eax
    int v16;           // esi
    int v17;           // eax
    unsigned int v18;  // ebx
    int v19;           // [esp+0h] [ebp-24h]
    unsigned int *v20; // [esp+Ch] [ebp-18h]
    int j;             // [esp+10h] [ebp-14h]

    v5 = this;
    result = a3;
    v7 = a4;
    v19 = a5;
    if (a4 != 0x2000) {
        for (i = 0; i < a3; ++i) {
            result = v19 + 1;
            v19 = result;
            if (result > 100)
                break;
            v9 = v5->some_buf5[v7];
            result = a2[i];
            for (j = v7; v9 != 0x2000; v9 = v5->some_buf6[v9]) {
                if (result == *((unsigned char *)v5->field_0 + v9))
                    break;
            }
            if (v9 == 0x2000) {
                v9 = v5->initially_256;
                if (v9 >= 0x2000) {
                    v9 = v5->initially_0x2000_2;
                    if (v7 == v9)
                        return result;
                    v10 = v5->some_buf[v9];
                    v5->initially_0x2000_2 = v10;
                    v5->some_buf2[v10] = 0x2000;
                    v11 = &v5->field_0[v9];
                    v12 = v11[26881];
                    v20 = &v5->field_0[v9];
                    v13 = v11[18689];
                    if (v12 == 0x2000)
                        v5->some_buf5[v20[2305]] = v13;
                    else
                        v5->some_buf6[v12] = v13;
                    if (v13 != 0x2000)
                        v5->some_buf7[v13] = v12;
                }
                else {
                    v5->initially_256 = v9 + 1;
                    v20 = &v5->field_0[v9];
                }
                *((unsigned char *)v5->field_0 + v9) = result;
                v20[2305] = v7;
                v5->some_buf5[v9] = 0x2000;
                v20[26881] = 0x2000;
                v14 = v5->some_buf5[j];
                v20[18689] = v14;
                if (v14 != 0x2000)
                    v5->some_buf7[v14] = v9;
                v5->some_buf5[j] = v9;
                if (v7 >= 256) {
                    v16 = v5->some_buf2[v7];
                    v17 = v5->some_buf_index_initially_0x2000;
                    if (v17 == 0x2000) {
                        v5->some_buf[v9] = 0x2000;
                        v5->some_buf2[v9] = 0x2000;
                        v5->initially_0x2000_2 = v9;
                        v5->some_buf_index_initially_0x2000 = v9;
                    }
                    else if (v16 == 0x2000) {
                        v5->some_buf2[v9] = 0x2000;
                        v5->some_buf[v9] = v5->initially_0x2000_2;
                        v5->some_buf2[v5->initially_0x2000_2] = v9;
                        v5->initially_0x2000_2 = v9;
                    }
                    else if (v16 == v17) {
                        v5->some_buf2[v9] = v17;
                        v5->some_buf[v9] = 0x2000;
                        v5->some_buf[v5->some_buf_index_initially_0x2000] = v9;
                        v5->some_buf_index_initially_0x2000 = v9;
                    }
                    else {
                        v5->some_buf2[v9] = v16;
                        v18 = v5->some_buf[v16];
                        v20[35073] = v18;
                        v5->some_buf2[v18] = v9;
                        v5->some_buf[v16] = v9;
                    }
                }
                else {
                    v15 = v5->some_buf_index_initially_0x2000;
                    if (v15 == 0x2000) {
                        v5->some_buf[v9] = 0x2000;
                        v5->some_buf2[v9] = 0x2000;
                        v5->initially_0x2000_2 = v9;
                        v5->some_buf_index_initially_0x2000 = v9;
                    }
                    else if (v15 == 0x2000) {
                        v5->some_buf2[v9] = 0x2000;
                        v5->some_buf[v9] = v5->initially_0x2000_2;
                        v5->some_buf2[v5->initially_0x2000_2] = v9;
                        v5->initially_0x2000_2 = v9;
                    }
                    else {
                        v5->some_buf2[v9] = v15;
                        v5->some_buf[v9] = 0x2000;
                        v5->some_buf[v5->some_buf_index_initially_0x2000] = v9;
                        v5->some_buf_index_initially_0x2000 = v9;
                    }
                }
            }
            result = a3;
            v7 = v9;
        }
    }
    return result;
}

signed int Compressor::InitState() {
    signed int result; // eax
    signed int v2;     // edx

    this->initially_8 = 8;
    this->most_recently_acquired_src_byte = 0;
    this->amount_to_shift_by = 0;
    this->initially_256 = 256;
    this->initially_0 = 0;
    this->shift_subtraction_initially_1 = 1;
    result = 1;
    this->maybe_some_bit_initially_2 = 2;
    this->initially_0x2000_2 = 0x2000;
    this->some_buf_index_initially_0x2000 = 0x2000;
    v2 = 0;
    do {
        *((unsigned char *)this->field_0 + v2) = v2;
        this->field_0[v2 + 2305] = 0x2000;
        this->some_buf6[v2] = 0x2000;
        this->some_buf7[v2] = 0x2000;
        this->some_buf5[v2] = 0x2000;
        *((unsigned char *)this->field_0 + v2 + 1) = result;
        this->some_buf6[v2 + 1] = 0x2000;
        this->some_buf7[v2 + 1] = 0x2000;
        this->some_buf5[v2 + 1] = 0x2000;
        this->field_0[v2 + 2306] = 0x2000;
        result += 2;
        v2 += 2;
    } while (v2 < 256);
    return result;
}

signed int Compressor::Unpack(unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size, unsigned int dw1) {
    int v5;                              // edx
    signed int bit;                      // eax
    int v7;                              // ecx
    signed int v8;                       // ebx
    int v9;                              // edx
    unsigned int v10;                    // eax
    unsigned int v11;                    // esi
    int v12;                             // edx
    int v13;                             // ecx
    int v14;                             // esi
    unsigned int v15;                    // eax
    unsigned int v16;                    // ebx
    int v17;                             // ecx
    int v18;                             // edx
    signed int v19;                      // esi
    int v20;                             // eax
    int v21;                             // ebx
    signed int offset_from_end_of_a_buf; // edi
    unsigned int *v23;                   // eax
    int v24;                             // edx
    unsigned int *v25;                   // ebx
    int v26;                             // ecx
    unsigned int *v27;                   // edi
    unsigned int v28;                    // edx
    int v30;                             // eax
    int v31;                             // esi
    int v32;                             // esi
    int v33;                             // ecx
    int v34;                             // esi
    int v35;                             // ecx
    int v36;                             // esi
    int v37;                             // [esp+4h] [ebp-20h]
    unsigned int v38;                    // [esp+8h] [ebp-1Ch]
    unsigned int v39;                    // [esp+8h] [ebp-1Ch]
    signed int v40;                      // [esp+8h] [ebp-1Ch]
    signed int a4;                       // [esp+Ch] [ebp-18h]
    int a3;                              // [esp+10h] [ebp-14h]

    this->src_size = src_size;
    this->available_dest_space = *dest_size;
    this->src = src;
    this->dest = *dest;
    this->src_word_size = 1;
    this->src_index = 0;
    this->filled_dest_space = 0;
    this->some_size_always_1 = dw1;
    this->InitState();
    if (src_size <= this->src_index)
        return 1;
    a3 = 0;
    a4 = 0x2000;
    while (1) {
        v5 = this->maybe_some_bit_initially_2;
        if (this->initially_256 - 256 >= v5) {
            ++this->shift_subtraction_initially_1;
            this->maybe_some_bit_initially_2 = 2 * v5;
        }
        bit = this->GetBit();
        if (bit == -1) {
            v18 = -1;
        }
        else if (bit) {
            v12 = this->shift_subtraction_initially_1;
            v13 = this->amount_to_shift_by;
            v14 = 0;
            if (v12 <= v13) {
                v16 = this->most_recently_acquired_src_byte;
            }
            else {
                v15 = this->src_index;
                v16 = this->most_recently_acquired_src_byte;
                v39 = this->src_size;
                while (1) {
                    v12 -= v13;
                    v14 |= (((1 << v13) - 1) & v16) << v12;
                    if (v15 >= v39) {
                        v16 = -1;
                    }
                    else {
                        v16 = this->src[v15];
                        v15 += this->src_word_size;
                        this->src_index = v15;
                    }
                    this->most_recently_acquired_src_byte = v16;
                    v13 = 8;
                    if (v12 <= 8)
                        break;
                    this->amount_to_shift_by = 8;
                }
            }
            v35 = v13 - v12;
            this->amount_to_shift_by = v35;
            v36 = ((1 << v12) - 1) & (v16 >> v35) | v14;
            v18 = v36 + 256;
            if (v36 == -1)
                v18 = -1;
        }
        else {
            v7 = this->amount_to_shift_by;
            v8 = 8;
            v9 = 0;
            if (v7 >= 8) {
                v11 = this->most_recently_acquired_src_byte;
            }
            else {
                v10 = this->src_index;
                v11 = this->most_recently_acquired_src_byte;
                v38 = this->src_size;
                while (1) {
                    v8 -= v7;
                    v9 |= (((1 << v7) - 1) & v11) << v8;
                    if (v10 >= v38) {
                        v11 = -1;
                    }
                    else {
                        v11 = this->src[v10];
                        v10 += this->src_word_size;
                        this->src_index = v10;
                    }
                    this->most_recently_acquired_src_byte = v11;
                    v7 = 8;
                    if (v8 <= 8)
                        break;
                    this->amount_to_shift_by = 8;
                }
            }
            v17 = v7 - v8;
            this->amount_to_shift_by = v17;
            v18 = ((1 << v8) - 1) & (v11 >> v17) | v9;
        }
        if (v18 == -1)
            return 0;
        if (v18 >= this->initially_256)
            break;
        v19 = a4;
        v20 = a3;
        a4 = v18;
        v21 = 0;
        a3 = 0;
        if (v18 != 0x2000) {
            v37 = v20;
            offset_from_end_of_a_buf = 0;
            v40 = v19;
            do {
                if (v18 < 256) {
                    v23 = &this->field_0[v18];
                }
                else if (v18 == this->some_buf_index_initially_0x2000) {
                    v23 = &this->field_0[v18];
                }
                else {
                    v30 = v18;
                    if (v18 == this->initially_0x2000_2) {
                        v23 = &this->field_0[v30];
                        v31 = v23[35073];
                        this->initially_0x2000_2 = v31;
                        this->some_buf2[v31] = 0x2000;
                    }
                    else {
                        v23 = &this->field_0[v30];
                        v32 = v23[43265];
                        v33 = v23[35073];
                        this->some_buf[v32] = v33;
                        this->some_buf2[v33] = v32;
                    }
                    v34 = this->some_buf_index_initially_0x2000;
                    if (v34 == 0x2000) {
                        v23[35073] = 0x2000;
                        v23[43265] = 0x2000;
                        this->initially_0x2000_2 = v18;
                        this->some_buf_index_initially_0x2000 = v18;
                    }
                    else if (v34 == 0x2000) {
                        v23[43265] = 0x2000;
                        v23[35073] = this->initially_0x2000_2;
                        this->some_buf2[this->initially_0x2000_2] = v18;
                        this->initially_0x2000_2 = v18;
                    }
                    else {
                        v23[35073] = 0x2000;
                        v23[43265] = v34;
                        this->some_buf[this->some_buf_index_initially_0x2000] = v18;
                        this->some_buf_index_initially_0x2000 = v18;
                    }
                }
                offset_from_end_of_a_buf -= 4;
                *(int *)((char *)&this->field_32594 + offset_from_end_of_a_buf) = *((unsigned char *)this->field_0 + v18);
                v18 = v23[2305];
                ++v21;
            } while (v18 != 0x2000);
            v20 = v37;
            v19 = v40;
            a3 = v21;
        }
        v24 = v21;
        v25 = &this->field_0[-v21];
        v26 = 0;
        v27 = v25 + 51557;
        if (v24 > 0) {
            v28 = this->filled_dest_space;
            do {
                if (v28 < this->available_dest_space) {
                    this->dest[v28] = v25[v26 + 51557];
                    v28 = this->some_size_always_1 + this->filled_dest_space;
                    this->filled_dest_space = v28;
                }
                ++v26;
            } while (v26 < a3);
            v27 = v25 + 51557;
        }
        this->sub_10001006(v27, a3, v19, v20);

        // This is the original code, but I think it might have a bug
        // if ( src_size <= this->src_index )
        //   return 1;
        if (src_size < this->src_index)
            return 1;
    }
    return 0;
}

signed int Compressor::GetBit() {
    int v1;            // eax
    unsigned int v2;   // edx
    signed int result; // eax
    unsigned int v4;   // eax
    bool v5;           // cf
    unsigned int v6;   // edx

    v1 = this->amount_to_shift_by - 1;
    if (v1 < 0) {
        v4 = this->src_index;
        v5 = v4 < this->src_size;
        this->amount_to_shift_by = 7;
        if (v5) {
            v6 = this->src[v4];
            this->src_index = this->src_word_size + v4;
        }
        else {
            v6 = -1;
        }
        this->most_recently_acquired_src_byte = v6;
        result = (v6 >> 7) & 1;
    }
    else {
        v2 = this->most_recently_acquired_src_byte;
        this->amount_to_shift_by = v1;
        result = (v2 >> v1) & 1;
    }
    return result;
}

signed int Compressor::Pack(unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size, unsigned int dw1, int bool1) {
    unsigned int v6;   // edx
    int v7;            // esi
    unsigned int v8;   // edi
    signed int v9;     // ebx
    int i;             // eax
    int v11;           // ebx
    unsigned int *v12; // ebx
    int v13;           // edx
    int v14;           // edx
    int v15;           // ecx
    int v16;           // edx
    int v17;           // edx
    unsigned int v18;  // edx
    int v19;           // esi
    signed int v20;    // edi
    unsigned int v21;  // edx
    unsigned int v22;  // eax
    signed int result; // eax
    signed int a4a;    // [esp+4h] [ebp-1Ch]
    int a5;            // [esp+8h] [ebp-18h]
    int v26;           // [esp+Ch] [ebp-14h]

    this->src_size = src_size;
    this->src = src;
    this->available_dest_space = 2 * (src_size / dw1);
    this->dest = (unsigned char *)malloc(2 * (src_size / dw1));
    this->some_size_always_1 = 1;
    this->src_index = 0;
    this->filled_dest_space = 0;
    this->src_word_size = dw1;
    this->InitState();
    v6 = this->src_index;
    v7 = 0;
    if (v6 >= this->src_size) {
        v8 = -1;
    }
    else {
        v8 = this->src[v6];
        this->src_index = this->src_word_size + v6;
    }
    if (v8 != -1) {
        v9 = 0x2000;
        do {
            a4a = v9;
            i = v8;
            a5 = v7;
            v7 = 0;
            v26 = this->some_buf_index_initially_0x2000;
            do {
                if (i >= 256) {
                    if (i == v26) {
                        v26 = this->some_buf2[i];
                    }
                    else {
                        v11 = i;
                        if (i == this->initially_0x2000_2) {
                            v12 = &this->field_0[v11];
                            v13 = v12[35073];
                            this->initially_0x2000_2 = v13;
                            this->some_buf2[v13] = 0x2000;
                        }
                        else {
                            v12 = &this->field_0[v11];
                            v14 = v12[43265];
                            v15 = v12[35073];
                            this->some_buf[v14] = v15;
                            this->some_buf2[v15] = v14;
                        }
                        v16 = this->some_buf_index_initially_0x2000;
                        if (v16 == 0x2000) {
                            v12[35073] = 0x2000;
                            v12[43265] = 0x2000;
                            this->initially_0x2000_2 = i;
                            this->some_buf_index_initially_0x2000 = i;
                        }
                        else if (v26 == 0x2000) {
                            v12[43265] = 0x2000;
                            v12[35073] = this->initially_0x2000_2;
                            this->some_buf2[this->initially_0x2000_2] = i;
                            this->initially_0x2000_2 = i;
                        }
                        else if (v26 == v16) {
                            v12[35073] = 0x2000;
                            v12[43265] = v16;
                            this->some_buf[this->some_buf_index_initially_0x2000] = i;
                            this->some_buf_index_initially_0x2000 = i;
                        }
                        else {
                            v12[43265] = v26;
                            v17 = this->some_buf[v26];
                            v12[35073] = v17;
                            this->some_buf2[v17] = i;
                            this->some_buf[v26] = i;
                        }
                    }
                }
                this->gap32404[v7] = v8;
                v18 = this->src_index;
                v9 = i;
                ++v7;
                if (v18 >= this->src_size) {
                    v8 = -1;
                }
                else {
                    v8 = this->src[v18];
                    this->src_index = this->src_word_size + v18;
                }
                for (i = this->some_buf5[i]; i != 0x2000; i = this->some_buf6[i]) {
                    if (v8 == *((unsigned char *)this->field_0 + i))
                        break;
                }
            } while (i != 0x2000);
            this->sub_10001BBB(v9);
            this->sub_10001006(this->gap32404, v7, a4a, a5);
        } while (v8 != -1);
    }
    v19 = this->initially_8;
    v20 = 7;
    if (v19 > 7) {
        v22 = this->most_recently_acquired_src_byte;
    }
    else {
        v21 = this->filled_dest_space;
        v22 = this->most_recently_acquired_src_byte;
        do {
            v20 -= v19;
            if (v21 < this->available_dest_space) {
                this->dest[v21] = v22;
                v21 = this->some_size_always_1 + this->filled_dest_space;
                this->filled_dest_space = v21;
            }
            ++this->initially_0;
            v22 = 0;
            v19 = 8;
        } while (v20 >= 8);
    }
    this->initially_8 = v19 - v20;
    this->most_recently_acquired_src_byte = v22;
    if (bool1 && src_size <= this->filled_dest_space) {
        free(this->dest);
        result = 0;
    }
    else {
        *dest = this->dest;
        result = 1;
        *dest_size = this->filled_dest_space;
    }
    return result;
}

int Compressor::sub_10001BBB(signed int a2) {
    Compressor *v2;   // edx
    signed int v3;    // ebx
    int v4;           // ebp
    int v5;           // eax
    int v6;           // ecx
    int v7;           // edi
    int v8;           // eax
    unsigned int v9;  // ebp
    signed int v10;   // ebp
    unsigned int v11; // ebx
    unsigned int v12; // esi
    int v13;          // eax
    int v14;          // edi
    int result;       // eax
    int v16;          // edi
    unsigned int v17; // eax
    unsigned int v18; // ebp
    signed int v19;   // ecx
    unsigned int v20; // ebp
    signed int v21;   // esi
    Compressor *v22;  // edi
    signed int v23;   // edx
    unsigned int v24; // eax
    int v25;          // edi

    v2 = this;
    v3 = a2;
    if (a2 < 256) {
        v16 = this->initially_8 - 1;
        if (this->initially_8 == 1) {
            v18 = this->filled_dest_space;
            if (v18 < this->available_dest_space) {
                this->dest[v18] = this->most_recently_acquired_src_byte;
                this->filled_dest_space += this->some_size_always_1;
            }
            ++this->initially_0;
            v17 = 0;
            v16 = 8;
        }
        else {
            v17 = this->most_recently_acquired_src_byte;
        }
        v19 = 8;
        if (v16 <= 8) {
            v20 = v2->filled_dest_space;
            v21 = v16;
            v22 = v2;
            v23 = 8;
            do {
                v23 -= v21;
                v24 = ((1 << v21) - 1) & ((unsigned int)a2 >> v23) | v17;
                if (v20 < v22->available_dest_space) {
                    v22->dest[v20] = v24;
                    v20 = v22->some_size_always_1 + v22->filled_dest_space;
                    v22->filled_dest_space = v20;
                }
                ++v22->initially_0;
                v17 = 0;
                v21 = 8;
            } while (v23 >= 8);
            v3 = a2;
            v19 = v23;
            v2 = v22;
            v16 = 8;
        }
        v25 = v16 - v19;
        v2->initially_8 = v25;
        result = ((v3 & ((1 << v19) - 1)) << v25) | v17;
        v2->most_recently_acquired_src_byte = result;
    }
    else {
        v4 = this->maybe_some_bit_initially_2;
        v5 = this->initially_256 - 256;
        if (v5 >= v4) {
            v6 = this->shift_subtraction_initially_1;
            do {
                ++v6;
                v4 *= 2;
            } while (v5 >= v4);
            v2->maybe_some_bit_initially_2 = v4;
            v2->shift_subtraction_initially_1 = v6;
        }
        v7 = v2->initially_8 - 1;
        v8 = v2->most_recently_acquired_src_byte | (1 << v7);
        if (v2->initially_8 == 1) {
            v9 = v2->filled_dest_space;
            if (v9 < v2->available_dest_space) {
                v2->dest[v9] = v8;
                v2->filled_dest_space += v2->some_size_always_1;
            }
            ++v2->initially_0;
            v8 = 0;
            v7 = 8;
        }
        v10 = v2->shift_subtraction_initially_1;
        v11 = a2 - 256;
        if (v10 >= v7) {
            v12 = v2->filled_dest_space;
            do {
                v10 -= v7;
                v13 = ((1 << v7) - 1) & (v11 >> v10) | v8;
                if (v12 < v2->available_dest_space) {
                    v2->dest[v12] = v13;
                    v12 = v2->some_size_always_1 + v2->filled_dest_space;
                    v2->filled_dest_space = v12;
                }
                ++v2->initially_0;
                v8 = 0;
                v7 = 8;
            } while (v10 >= 8);
        }
        v14 = v7 - v10;
        v2->initially_8 = v14;
        result = ((((1 << v10) - 1) & v11) << v14) | v8;
        v2->most_recently_acquired_src_byte = result;
    }
    return result;
}

int Unpack(const unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size) {
    Compressor compressor;
    return compressor.Unpack((unsigned char *)src, src_size, dest, dest_size, 1);
}

int Pack(const unsigned char *src, unsigned int src_size, unsigned char **dest, unsigned int *dest_size) {
    Compressor compressor;
    return compressor.Pack((unsigned char *)src, src_size, dest, dest_size, 1, 0);
}

void PackFree(unsigned char *mem) {
    free(mem);
}