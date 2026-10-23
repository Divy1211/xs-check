mutable void foo(int a = 1) {
    xsChatData("r: " + (a + 10));
}

void foo(int a = 2) {
    xsChatData("r: " + (a + 20));
}

void main() {
    foo();
}

void foo(int a = 3) {
    xsChatData("r: " + (a + 30));
}