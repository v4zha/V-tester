#include <stdio.h>
// Linear search
// unsafe C code
// for demo perpose : )
int main(int argc, char** argv) {
  int n, ele;
  // printf("Enter the size of array\n");
  scanf("%d", &n);
  int arr[n];
  // printf("Enter the elements of array\n");
  for (int i = 0; i < n; i++) {
    scanf("%d", &arr[i]);
  }
  // printf("Enter the number to search\n");
  scanf("%d", &ele);
  for (int i = 0; i < n; i++) {
    if (arr[i] == ele) {
      printf("Yes");
      return 0;
    }
  }
  printf("NO");
  return -1;
}