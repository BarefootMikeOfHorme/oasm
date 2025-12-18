#include <windows.h>
__declspec(dllexport) int add_i32(int a,int b){return a+b;}
__declspec(dllexport) int mul_i32(int a,int b){return a*b;}
__declspec(dllexport) int div_i32(int a,int b){return b==0?0:a/b;}
