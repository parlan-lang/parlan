extern int printf(char** fmt,...);

double square(double n) {
    return (n * n);
}

double fib(double n) {
    if ((n < 2)) {

        return n;
    } 
    return (fib((n - 1)) + fib((n - 2)));
}

double sqrt(double n,int prec) {
    double x = (n / 2);
    int i = 0;
    while ((i < prec)) {

        x = (0.5 * (x + (n / x)));
    }

    return x;
}

int main() {
    double fib5 = fib(5);
    double sqfib5 = square(fib5);
    double sqrtsqfib5 = sqrt(sqfib5,10);
    printf("%lf\n",sqrtsqfib5);
    return 0;
}

