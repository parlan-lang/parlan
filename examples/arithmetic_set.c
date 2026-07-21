extern int printf(const char* fmt,...);
void set(int a, int d, int j){
    printf("set:\n{");
    for (int i = 1; i <= j; i++){
        int term = (a + (d*(i-1)));
        if (i!=j) printf("%i, ", term); else printf("%i}\n", term);
    }
}

int main(){
    int firstTerm = 3;
    int commonDifferences = 2;
    int terms = 5;
    set(firstTerm, commonDifferences, terms);
    return 0;
}