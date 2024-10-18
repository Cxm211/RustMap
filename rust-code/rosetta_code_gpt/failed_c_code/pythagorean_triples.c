typedef unsigned long xint; 
#define FMT "%lu" 


xint total, prim, max_peri;

xint U[][9] =  {{ 1, -2, 2,  2, -1, 2,  2, -2, 3},
                { 1,  2, 2,  2,  1, 2,  2,  2, 3},
                {-1,  2, 2, -2,  1, 2, -2,  2, 3}};



void new_tri(xint in[]) {
    int i;
    xint t[3], p = in[0] + in[1] + in[2];  

    if (p > max_peri) return; 

    prim++; 

    total += max_peri / p; 

    for (i = 0; i < 3; i++) {
        t[0] = U[i][0] * in[0] + U[i][1] * in[1] + U[i][2] * in[2];
        t[1] = U[i][3] * in[0] + U[i][4] * in[1] + U[i][5] * in[2];
        t[2] = U[i][6] * in[0] + U[i][7] * in[1] + U[i][8] * in[2];
        new_tri(t);  
    }
}

int main() {
    xint seed[3] = {3, 4, 5};  

    for (max_peri = 10; max_peri <= 100000000; max_peri *= 10) {
        total = prim = 0;
        new_tri(seed); 

        printf( "Up to "FMT": "FMT" triples, "FMT" primitives.\n",
                max_peri, total, prim);
    }
    return 0;
}