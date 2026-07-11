extern int printf(const char* fmt,...);

int fibonacci(int n) {
    if (n < 2) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
    int f20 = fibonacci(20);
    printf("fibonacci of 20 = %d",f20);
    return 0;
}

