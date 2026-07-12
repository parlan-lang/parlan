extern int printf(char **fmt, ...);

void swap(int* x, int* y){
    *x += *y;
    *y = *x - *y;
    *x -= *y;
}

int main(){
    int x = 3;
    int y = 2;

    printf("before: (%i, %i)\n", x, y);
    swap(&x, &y);
    printf("after: (%i, %i)\n", x, y);

    return 0;
}