#include <iostream>

using namespace std;

extern "C"
{
    void hello();
    void greet(const char* name);
    int multiply(int x, int y);
}

void hello()
{
    cout << "Hello from C++!" << endl;
}

void greet(const char* name)
{
    cout << "Hello, " << name << "!" << endl;
}

int multiply(int x, int y)
{
    return x * y;
}