#include <stdio.h>                                                 
#include <stdlib.h>                                                
#include <limits.h>                                                
                                                                   
char *generate_yeet(unsigned int n)                                
                                                                  {
    n++                                                           ;
                                                                   
    int bit = 16                                                  ;
    int yeet_size = 4                                             ;
    while (bit <= INT_MAX && n >= bit)                            {
        bit <<= 1                                                 ;
        yeet_size++                                               ;}
                                                                  
                                                                   
    char *yeet = (char*) malloc(sizeof(char) * yeet_size + 1)     ;
    yeet[yeet_size] = '\0'                                        ;
                                                                   
    int capital                                                   ;
    for (int i = yeet_size - 1; i >= 0; i--)                      {
        capital = n % 2                                           ;
        if      (i == 0)             yeet[i] = capital ? 'Y' : 'y';
        else if (i == yeet_size - 1) yeet[i] = capital ? 'T' : 't';
        else                         yeet[i] = capital ? 'E' : 'e';
                                                                   
        n >>= 1                                                   ;}
                                                                  
                                                                   
    return yeet                                                   ;}
                                                                  
                                                                   
int main()                                                         
                                                                  {
    char *yeet                                                    ;
    for (int i = 0; i < 200; i++)                                 {
        yeet = generate_yeet(i)                                   ;
        printf("%-3d [%s]\n", i, yeet)                            ;
        free(yeet)                                                ;}
                                                                  }
                                                                  
