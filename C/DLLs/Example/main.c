// main.c
#include <windows.h>
#include <stdio.h>

typedef int (*AddFunc)(int, int);

int main() {
    HINSTANCE hDll = LoadLibrary("math.dll");
    if (hDll != NULL) {
        AddFunc add = (AddFunc)GetProcAddress(hDll, "add");
        if (add != NULL) {
            printf("Result: %d\n", add(5, 3));
        }
        FreeLibrary(hDll);
    }
    return 0;
}