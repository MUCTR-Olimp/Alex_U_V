#include<stdio.h>
#include<stdlib.h>
#define IX(x,y) 12*(y)+(x)

//ssize_t scan_line_from_stdin(char* line);
void bfs(int total);
void reverse_bfs(int x,int y);
int main() {
    //char* first_line = NULL;
    //size_t f = 0;
    //ssize_t len = getline(&first_line,&f,stdin);
    //ssize_t len = scan_line_from_stdin(first_line);
    char first_line[4] = {'\0','\0','\0','\0'};
    //fgets(first_line,5,stdin);
    char c = 0;
    int len = 0;
    scanf("%c",&c);
    while(c != '\n'){
        first_line[len] = c;
        len += 1;
        scanf("%c",&c);
    }
    //int len = 4;
    for(int i = 0; i < len; i++){
        if(first_line[i] == ' '){
            int x,y = 0;
            sscanf(first_line,"%d %d",&x,&y);
            reverse_bfs(x,y);
            //free(first_line);
            return 0;
        }
    }
    int total = 0;
    sscanf(first_line,"%d",&total);
    bfs(total);
    //free(first_line);
    return 0;
}
void bfs(int total){
    int matrix[144] = {0};
    //int matrix[12][12];
    int start_x = 0;
    int start_y = 0;
    scanf("%d %d",&start_x,&start_y);
    for(int i = 1; i < total; i ++){
        int a,b = 0;
        scanf("%d %d", &a, &b);
        matrix[IX(a,b)] = 1;
    }

    int *order = malloc(total*2*sizeof(int));
    for(int i = 0; i < total*2; i++){
        order[i] = 0;
    }
    int head = 0;
    int tail = 0;
    order[tail] = start_x;
    order[tail +1] = start_y;
    printf("%d %d\n",start_x,start_y);
    while(head < 2*total) {
        matrix[IX(order[head],order[head+1])] = 0;
        if(matrix[IX(order[head]+1,order[head+1])]==1){
            printf("R");
            matrix[IX(order[head]+1,order[head+1])] = 2;
            tail+=2;
            order[tail] = order[head]+1;
            order[tail+1] = order[head+1];
        }
        if(matrix[IX(order[head],order[head+1]+1)]==1){
            printf("T");
            matrix[IX(order[head],order[head+1]+1)] = 2;
            tail+=2;
            order[tail] = order[head];
            order[tail+1] = order[head+1]+1;
        }
        if(matrix[IX(order[head]-1,order[head+1])]==1){
            printf("L");
            matrix[IX(order[head]-1,order[head+1])] = 2;
            tail+=2;
            order[tail] = order[head]-1;
            order[tail+1] = order[head+1];
        }
        if(matrix[IX(order[head],order[head+1]-1)]==1){
            printf("B");
            matrix[IX(order[head],order[head+1]-1)] = 2;
            tail+=2;
            order[tail] = order[head];
            order[tail+1] = order[head+1]-1;
        }
        head+=2;
        if(head < total*2){ printf(",\n"); }else{printf(".");}
    }
    return;
}
void reverse_bfs(int x, int y){
    int queue_len = 1;
    int* queue = calloc(queue_len*2,sizeof(int));
    int head,tail = 0;
    queue[head] = x;
    queue[head + 1] = y;
    char current = 0;
    while(current != '.'){
        scanf("%c",&current);
        if(current == '\n') scanf("%c",&current);
        while(current != ',' && current != '.' && current != '\n' && current != '\0'){
            switch(current){
                case 'R':
                    if(tail/2+1==queue_len){
                        queue = realloc(queue,queue_len*4*sizeof(int));
                        queue_len*=2;
                    }
                    tail+=2;
                    queue[tail] = queue[head]+1;
                    queue[tail+1] = queue[head+1];
                    break;
                case 'T':
                    if(tail/2+1==queue_len){
                        queue = realloc(queue,queue_len*4*sizeof(int));
                        queue_len*=2;
                    }
                    tail+=2;
                    queue[tail] = queue[head];
                    queue[tail+1] = queue[head+1]+1;
                    break;
                case 'L':
                    if(tail/2+1==queue_len){
                        queue = realloc(queue,queue_len*4*sizeof(int));
                        queue_len*=2;
                    }
                    tail+=2;
                    queue[tail] = queue[head]-1;
                    queue[tail+1] = queue[head+1];
                    break;
                case 'B':
                    if(tail/2+1==queue_len){
                        queue = realloc(queue,queue_len*4*sizeof(int));
                        queue_len*=2;
                    }
                    tail+=2;
                    queue[tail] = queue[head];
                    queue[tail+1] = queue[head+1]-1;
                    break;
                default:
                    break;
            }
            scanf("%c",&current);
        }
        head+=2;
    }
    int matrix[144] = {0};
    printf("%d\n",tail/2+1);
    for(int i = 0; i <= tail; i+=2){
        matrix[IX(queue[i],queue[i+1])] = 1;
    }
    for(int i = 1; i < 11; i++) 
        for(int j = 1; j<11; j++)
            if(matrix[IX(i,j)]) printf("%d %d\n",i,j);
    return;
}
/*ssize_t scan_line_from_stdin(char* line) {
    char current_char = 0;
    ssize_t length = 0;
    int current_index = 0;
    scanf("%c",&current_char);
    while(current_char != '\0' && current_char != '\n'){
        length += 1;
        line = realloc(line,length*sizeof(char));
        line[current_index++] = current_char;
        scanf("%c", &current_char);
    }
    return length;
}*/