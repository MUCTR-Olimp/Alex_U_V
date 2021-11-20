#include<stdio.h>

int main(){
    int l,lp,a,b = 0;
    scanf("%d %d %d %d", &l, &lp, &a, &b);
    if(b>a) printf("%d",(b-a-1)*(l+2*lp)+2*lp);
    else printf("%d",(a-b-1)*(l+2*lp)+2*(lp+l));
    return 0;
}