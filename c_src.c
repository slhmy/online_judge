// Takes about 7-12s
#include <stdio.h>
int main(){
    long long a;
    scanf("%d", &a);
    for (int i = 0; i < 1000000000; i++) {
        for (int j = 0; j < 1000000000; j++) {
            for (int k = 0; k < 10; k++) {
                a = a + j - i;
            }
        }
    }
    printf("%d\n", a);
    return 0;
}